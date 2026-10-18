//! UIKit bridges for the iPad host: the Files document picker behind `Services::picker`, the
//! window's safe area, the share sheet behind `Services::reveal` (`share`), and saving exports to
//! the Photos app (`photos_runner`).
//!
//! This is the only module of RawPad with `unsafe` (Objective-C messages through objc2), allowed
//! here and nowhere else (DECISIONS.md, 2026-10-18). Every call happens on the main thread, where
//! `LightcraftApp` runs its commands; a call from any other thread is refused, never made. The one
//! exception is `photos_runner`, which runs on the engine's worker thread and refuses the main one.
#![allow(unsafe_code)]
// objc2 marks some generated methods `unsafe` and others safe; one `unsafe` block per call keeps
// the reasoning local without tracking which is which.
#![allow(unused_unsafe)]

use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};

use lightcraft_ui_egui::pick::{PickKind, PickRequest};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{NSArray, NSObject, NSObjectProtocol, NSURL};
use objc2_ui_kit::{UIDocumentPickerDelegate, UIDocumentPickerViewController, UIViewController};
use objc2_uniform_type_identifiers::{UTType, UTTypeFolder, UTTypeImage, UTTypeItem};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};

/// Where exports land when a command asks for a save location: `Documents/Exports`, which the
/// Files app shows under *On My iPad › RawPad* (`UIFileSharingEnabled`).
pub fn exports_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").filter(|h| !h.is_empty())?;
    Some(PathBuf::from(home).join("Documents").join("Exports"))
}

pub struct Ivars {
    /// Answers the waiting command once; `None` after that.
    tx: RefCell<Option<Sender<Vec<String>>>>,
    /// Picked items are originals outside the sandbox (folders): keep their security scope open.
    scoped: bool,
    /// Wakes the UI so the waiting command runs on the next frame.
    ctx: egui::Context,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements; this class doesn't implement Drop.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "RawPadDocumentPickerDelegate"]
    #[ivars = Ivars]
    pub struct PickerDelegate;

    // SAFETY: NSObjectProtocol has no extra requirements.
    unsafe impl NSObjectProtocol for PickerDelegate {}

    // SAFETY: the signatures match UIDocumentPickerDelegate's methods.
    unsafe impl UIDocumentPickerDelegate for PickerDelegate {
        #[unsafe(method(documentPicker:didPickDocumentsAtURLs:))]
        fn did_pick(&self, _controller: &UIDocumentPickerViewController, urls: &NSArray<NSURL>) {
            let scoped = self.ivars().scoped;
            let mut paths = Vec::new();
            for url in urls.iter() {
                if scoped {
                    // SAFETY: a file URL the picker just returned. The access is kept for the
                    // app's lifetime (imports and exports read and write it later).
                    let ok = unsafe { url.startAccessingSecurityScopedResource() };
                    if !ok {
                        log::warn!("no security-scoped access to a picked folder");
                    }
                }
                // SAFETY: plain property read on a valid NSURL.
                if let Some(p) = unsafe { url.path() } {
                    paths.push(p.to_string());
                }
            }
            self.finish(paths);
        }

        #[unsafe(method(documentPickerWasCancelled:))]
        fn cancelled(&self, _controller: &UIDocumentPickerViewController) {
            self.finish(Vec::new());
        }
    }
);

impl PickerDelegate {
    fn new(mtm: MainThreadMarker, tx: Sender<Vec<String>>, scoped: bool, ctx: egui::Context) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(Ivars { tx: RefCell::new(Some(tx)), scoped, ctx });
        // SAFETY: NSObject's designated initializer on a freshly allocated instance.
        unsafe { msg_send![super(this), init] }
    }

    fn finish(&self, paths: Vec<String>) {
        log::info!("document picker: {} item(s)", paths.len());
        if let Some(tx) = self.ivars().tx.borrow_mut().take() {
            // the command may have been abandoned meanwhile: nobody to answer then
            let _ = tx.send(paths);
        }
        self.ivars().ctx.request_repaint();
    }
}

thread_local! {
    /// UIDocumentPickerViewController holds its delegate weakly: keep the last one alive.
    static DELEGATE: RefCell<Option<Retained<PickerDelegate>>> = const { RefCell::new(None) };
}

/// The view controller winit made for the window: dialogs are presented from it.
fn root_view_controller(window: &winit::window::Window) -> Result<Retained<UIViewController>, String> {
    let handle = window.window_handle().map_err(|e| format!("no window handle: {e}"))?;
    let RawWindowHandle::UiKit(h) = handle.as_raw() else {
        return Err("not a UIKit window".into());
    };
    let ptr = h.ui_view_controller.ok_or("the window has no view controller")?;
    // SAFETY: winit's UIKit handle points at the window's live UIViewController, owned by the
    // window that outlives this call; retaining it keeps it alive while we use it.
    unsafe { Retained::retain(ptr.as_ptr().cast::<UIViewController>()) }.ok_or_else(|| "null view controller".to_string())
}

/// Answer a command's request for paths with the Files picker (`Services::picker`).
pub fn pick(window: &winit::window::Window, ctx: &egui::Context, req: &PickRequest) -> Result<Receiver<Vec<String>>, String> {
    let (tx, rx) = channel();
    if req.kind == PickKind::Save {
        // No save dialog: the file goes to Documents/Exports (visible in the Files app).
        let dir = exports_dir().ok_or("no Documents folder")?;
        std::fs::create_dir_all(&dir).map_err(|e| format!("can't create {}: {e}", dir.display()))?;
        let name = req.file_name.clone().unwrap_or_else(|| "export".into());
        let _ = tx.send(vec![dir.join(name).to_string_lossy().into_owned()]);
        return Ok(rx);
    }
    let mtm = MainThreadMarker::new().ok_or("the file picker must be opened on the main thread")?;
    let vc = root_view_controller(window)?;
    let images = req.filter.as_ref().is_some_and(|(_, exts)| exts.iter().any(|e| e.eq_ignore_ascii_case("jpg")));
    // SAFETY: reading UniformTypeIdentifiers' constant UTType statics.
    let ty: &UTType = unsafe {
        match req.kind {
            PickKind::Folder => UTTypeFolder,
            _ if images => UTTypeImage,
            _ => UTTypeItem,
        }
    };
    let types = NSArray::from_slice(&[ty]);
    // Files are copied into the app's inbox (no lasting access needed); folders are opened in
    // place (an SD card, a folder in Files) and keep their security scope.
    let folder = req.kind == PickKind::Folder;
    // SAFETY: initializer on a freshly allocated picker, main thread.
    let picker =
        unsafe { UIDocumentPickerViewController::initForOpeningContentTypes_asCopy(UIDocumentPickerViewController::alloc(mtm), &types, !folder) };
    let delegate = PickerDelegate::new(mtm, tx, folder, ctx.clone());
    // SAFETY: setters on a live picker, main thread; the delegate is kept alive in DELEGATE.
    unsafe {
        picker.setAllowsMultipleSelection(req.kind == PickKind::Files);
        picker.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
        vc.presentViewController_animated_completion(&picker, true, None);
    }
    DELEGATE.with(|d| *d.borrow_mut() = Some(delegate));
    Ok(rx)
}

/// `UIEdgeInsets` (CGFloat is f64 on every 64-bit Apple target), declared here so reading the
/// safe area needs no extra objc2-ui-kit features.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EdgeInsets {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

// SAFETY: the layout above matches UIKit's `UIEdgeInsets {CGFloat top, left, bottom, right}`.
unsafe impl objc2::Encode for EdgeInsets {
    const ENCODING: objc2::Encoding = objc2::Encoding::Struct(
        "UIEdgeInsets",
        &[objc2::Encoding::Double, objc2::Encoding::Double, objc2::Encoding::Double, objc2::Encoding::Double],
    );
}

/// The window's safe area (home indicator, rounded corners, status bar) in points, read from the
/// view winit draws into. `None` off the main thread or without a UIKit view.
pub fn safe_area(window: &winit::window::Window) -> Option<EdgeInsets> {
    MainThreadMarker::new()?;
    let handle = window.window_handle().ok()?;
    let RawWindowHandle::UiKit(h) = handle.as_raw() else { return None };
    let view = h.ui_view.as_ptr().cast::<NSObject>();
    // SAFETY: winit's handle points at the window's live UIView (owned by the window, which
    // outlives this call); `safeAreaInsets` is a plain UIView property read on the main thread.
    let insets: EdgeInsets = unsafe { msg_send![&*view, safeAreaInsets] };
    let ok = [insets.top, insets.left, insets.bottom, insets.right].iter().all(|v| v.is_finite() && *v >= 0.0);
    ok.then_some(insets)
}

/// `CGRect` (`{CGPoint origin; CGSize size}`, CGFloat = f64), declared here like [`EdgeInsets`].
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

// SAFETY: four doubles in the order and nesting of UIKit's `CGRect {CGPoint {x, y}, CGSize {width, height}}`.
unsafe impl objc2::Encode for Rect {
    const ENCODING: objc2::Encoding = objc2::Encoding::Struct(
        "CGRect",
        &[
            objc2::Encoding::Struct("CGPoint", &[objc2::Encoding::Double, objc2::Encoding::Double]),
            objc2::Encoding::Struct("CGSize", &[objc2::Encoding::Double, objc2::Encoding::Double]),
        ],
    );
}

/// The share sheet (`UIActivityViewController`) for one file or folder: Save to Files, AirDrop,
/// Mail, other apps. The iPad host's `Services::reveal`, so every *Show in Finder* place in the UI
/// (a photo's original, a folder, Help › Open Log Folder) shares instead. On iPad the sheet is a
/// popover and must have an anchor, or UIKit raises an exception: it points at `at` (window
/// points, normally where the finger was), or at the middle of the window.
pub fn share(window: &winit::window::Window, path: &str, at: Option<(f64, f64)>) -> Result<(), String> {
    share_files(window, &[path], at)
}

/// The share sheet for several files at once (the photos of one export).
pub fn share_files(window: &winit::window::Window, paths: &[&str], at: Option<(f64, f64)>) -> Result<(), String> {
    use objc2::runtime::{AnyClass, AnyObject};
    use objc2_foundation::NSString;
    MainThreadMarker::new().ok_or("the share sheet must be opened on the main thread")?;
    let paths: Vec<&str> = paths.iter().copied().filter(|p| std::path::Path::new(p).exists()).collect();
    let Some(first) = paths.first() else {
        return Err("the exported files aren't there any more".into());
    };
    let vc = root_view_controller(window)?;
    let class = AnyClass::get(c"UIActivityViewController").ok_or("no share sheet on this system")?;
    let mut urls: Vec<Retained<NSURL>> = Vec::with_capacity(paths.len());
    for p in &paths {
        // SAFETY: makes a file URL from a string (no file access).
        urls.push(unsafe { NSURL::fileURLWithPath(&NSString::from_str(p)) });
    }
    let refs: Vec<&NSURL> = urls.iter().map(|u| &**u).collect();
    let items = NSArray::from_slice(&refs);
    // SAFETY: `alloc` on UIActivityViewController's class, then its designated initializer with an
    // array of items and no custom activities, on the main thread.
    let sheet: Option<Retained<AnyObject>> = unsafe {
        let a: objc2::rc::Allocated<AnyObject> = msg_send![class, alloc];
        msg_send![a, initWithActivityItems: &*items, applicationActivities: Option::<&AnyObject>::None]
    };
    let sheet = sheet.ok_or("the share sheet couldn't be made")?;
    // SAFETY: plain property reads on live view controllers, main thread.
    let (popover, view): (Option<Retained<AnyObject>>, Option<Retained<AnyObject>>) =
        unsafe { (msg_send![&*sheet, popoverPresentationController], msg_send![&*vc, view]) };
    let (Some(popover), Some(view)) = (popover, view) else {
        return Err("the share sheet has no place to appear".into());
    };
    // SAFETY: a property read on the live root view, main thread.
    let bounds: Rect = unsafe { msg_send![&*view, bounds] };
    let (x, y) = at
        .filter(|(x, y)| x.is_finite() && y.is_finite() && (0.0..=bounds.width).contains(x) && (0.0..=bounds.height).contains(y))
        .unwrap_or((bounds.width / 2.0, bounds.height / 2.0));
    let anchor = Rect { x, y, width: 1.0, height: 1.0 };
    log::info!("share sheet for {first}{}", if paths.len() > 1 { format!(" and {} more", paths.len() - 1) } else { String::new() });
    // SAFETY: setters on the sheet's popover controller (the view is retained by it), then
    // presenting the sheet from the window's view controller; all on the main thread. The cast is
    // sound: UIActivityViewController is a UIViewController subclass.
    unsafe {
        let _: () = msg_send![&*popover, setSourceView: &*view];
        let _: () = msg_send![&*popover, setSourceRect: anchor];
        let sheet: Retained<UIViewController> = Retained::cast_unchecked(sheet);
        vc.presentViewController_animated_completion(&sheet, true, None);
    }
    Ok(())
}

// Photos.framework, for "Add to Photos" after an export (`photos_runner`). Its classes are looked
// up by name at run time, so only the link is needed here.
#[link(name = "Photos", kind = "framework")]
unsafe extern "C" {}

/// Adds exported files to the Photos app: the engine's Apple Photos import (made for macOS's
/// osascript, see `lightcraft_engine::apple_photos`) with a runner that reads the file paths from
/// the osascript arguments and saves each one with `PHPhotoLibrary`. Prints the new assets' local
/// ids, one per line, as the script would. Albums aren't supported (the iPad hides the field).
/// Runs on the engine's worker thread (`Session::apple_photos_background`): on the main thread it
/// refuses, because `performChangesAndWait` must not block it. iPadOS asks for permission the first
/// time (`NSPhotoLibraryAddUsageDescription` in `Info.plist`).
pub fn photos_runner() -> lightcraft_engine::apple_photos::Runner {
    use lightcraft_engine::apple_photos::{MARKER, Output, RunError};
    std::sync::Arc::new(|args: &[String], _limit| {
        if MainThreadMarker::new().is_some() {
            return Err(RunError::Spawn("adding to Photos can't wait on the main thread".into()));
        }
        let paths = args.iter().position(|a| a == MARKER).and_then(|m| args.get(m.saturating_add(2)..)).unwrap_or_default();
        let (mut ids, mut errors) = (Vec::new(), Vec::new());
        for p in paths {
            match save_to_photos(p) {
                Ok(id) => ids.push(id),
                Err(e) => errors.push(format!("{p}: {e}")),
            }
        }
        let failed = ids.is_empty() && !errors.is_empty();
        Ok(Output { status: Some(if failed { 1 } else { 0 }), stdout: ids.join("\n"), stderr: errors.join("\n") })
    })
}

/// Save one image file to the photo library; the new asset's local id.
fn save_to_photos(path: &str) -> Result<String, String> {
    use objc2::runtime::{AnyClass, AnyObject};
    use objc2_foundation::{NSError, NSString};
    let library_class = AnyClass::get(c"PHPhotoLibrary").ok_or("the Photos framework isn't available")?;
    let request_class = AnyClass::get(c"PHAssetChangeRequest").ok_or("the Photos framework isn't available")?;
    // SAFETY: makes a file URL from a string (no file access).
    let url = unsafe { NSURL::fileURLWithPath(&NSString::from_str(path)) };
    // SAFETY: a class method of PHPhotoLibrary with no arguments, returning the shared library.
    let library: Option<Retained<AnyObject>> = unsafe { msg_send![library_class, sharedPhotoLibrary] };
    let library = library.ok_or("no photo library")?;
    let id = std::sync::Arc::new(std::sync::Mutex::new(None::<String>));
    let block = {
        let id = std::sync::Arc::clone(&id);
        block2::RcBlock::new(move || {
            // SAFETY: called by Photos inside its change block, as the API requires; `url` is a
            // live NSURL owned by this closure.
            let request: Option<Retained<AnyObject>> = unsafe { msg_send![request_class, creationRequestForAssetFromImageAtFileURL: &*url] };
            let Some(request) = request else { return };
            // SAFETY: plain property reads on the request just made.
            let placeholder: Option<Retained<AnyObject>> = unsafe { msg_send![&*request, placeholderForCreatedAsset] };
            let Some(placeholder) = placeholder else { return };
            // SAFETY: as above.
            let local: Option<Retained<NSString>> = unsafe { msg_send![&*placeholder, localIdentifier] };
            if let Some(local) = local {
                *id.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(local.to_string());
            }
        })
    };
    // SAFETY: `performChangesAndWait:error:` runs the block synchronously on Photos' queue; this
    // is a worker thread (checked by the runner), never the main thread.
    let done: Result<(), Retained<NSError>> = unsafe { msg_send![&*library, performChangesAndWait: &*block, error: _] };
    if let Err(e) = done {
        // SAFETY: a property read on the error Photos returned.
        return Err(unsafe { e.localizedDescription() }.to_string());
    }
    let id = id.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    id.ok_or_else(|| "Photos couldn't read this file as an image".to_string())
}
