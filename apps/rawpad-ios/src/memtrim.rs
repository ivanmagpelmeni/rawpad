//! Memory warnings: iPadOS warns an app before it ends it for using too much memory (jetsam). On
//! a warning RawPad drops the engine's cached decodes and rendered previews (everything but the
//! photo on screen, which `MediaCache::enforce_budget` never evicts), frees the GPU renderer's
//! recycled buffers and hands freed pages back to the system. Caches refill on demand.
//! Plain engine calls, so the test runs on the CI's macOS host too.
//!
//! [`set_ipad_limits`] also keeps fewer loupe-sized decodes than the desktop (DECISIONS 74).

use lightcraft_engine::Session;

/// The cache budget is squeezed to this for a moment, which evicts the least recently used
/// entries at once; then the normal budget comes back (so the next photos cache as before).
const SQUEEZE_BYTES: usize = 16 << 20;

/// Loupe-sized decodes (≤ 2560 px, linear float: about 52 MB each) kept at once. The desktop
/// keeps 4; 3 still holds both sides of Compare plus the next photo, like the web build.
pub const PREVIEW_CAPACITY: usize = 3;

/// iPad cache limits, set once when the session opens (they survive opening another library).
pub fn set_ipad_limits(session: &mut Session) {
    session.media.preview_capacity = PREVIEW_CAPACITY;
}

/// Bytes the engine's in-memory caches held before and after a trim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trimmed {
    pub before: usize,
    pub after: usize,
}

/// Give memory back after a system memory warning.
pub fn trim_caches(session: &mut Session) -> Trimmed {
    let before = session.media.held();
    let budget = session.media.budget();
    session.media.set_budget(SQUEEZE_BYTES.min(budget));
    session.media.set_budget(budget);
    lightcraft_engine::gpu::trim_pool(0);
    lightcraft_engine::memory::release();
    Trimmed { before, after: session.media.held() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_memory_warning_drops_cached_photos_but_keeps_the_budget() {
        let mut s = Session::with_demo();
        let ids: Vec<_> = s.catalog.photos().map(|p| p.id).take(4).collect();
        assert!(ids.len() >= 3, "demo library has photos");
        for id in &ids {
            s.render_now(*id, 800, 600).unwrap();
        }
        let budget = s.media.budget();
        let t = trim_caches(&mut s);
        assert!(t.before > 0, "renders filled the caches: {t:?}");
        assert!(t.after < t.before, "the trim freed something: {t:?}");
        assert_eq!(s.media.budget(), budget, "the normal budget is back");
        // renders still work after a trim
        s.render_now(ids[0], 800, 600).unwrap();
    }

    #[test]
    fn the_ipad_keeps_at_most_three_loupe_sized_decodes() {
        let mut s = Session::with_demo();
        set_ipad_limits(&mut s);
        let ids: Vec<_> = s.catalog.photos().map(|p| p.id).take(5).collect();
        assert!(ids.len() >= 5, "demo library has photos");
        // 1600 px wide is above the thumbnail level (520 px), so each render decodes a preview
        for id in &ids {
            s.render_now(*id, 1600, 1200).unwrap();
        }
        let previews = s.media.usage().1;
        assert!(previews.count >= 1, "renders decoded previews: {previews:?}");
        assert!(previews.count <= PREVIEW_CAPACITY, "kept {} previews", previews.count);
        // and renders keep working with the lower cap
        s.render_now(ids[0], 1600, 1200).unwrap();
    }

    #[test]
    fn trimming_an_empty_session_is_harmless() {
        let mut s = Session::new();
        let t = trim_caches(&mut s);
        assert_eq!(t, Trimmed { before: 0, after: 0 });
    }
}
