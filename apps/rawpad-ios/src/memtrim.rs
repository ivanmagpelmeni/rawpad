//! Memory warnings: iPadOS warns an app before it ends it for using too much memory (jetsam). On
//! a warning RawPad drops the engine's cached decodes and rendered previews (everything but the
//! photo on screen, which `MediaCache::enforce_budget` never evicts), frees the GPU renderer's
//! recycled buffers and hands freed pages back to the system. Caches refill on demand.
//! Plain engine calls, so the test runs on the CI's macOS host too.

use lightcraft_engine::Session;

/// The cache budget is squeezed to this for a moment, which evicts the least recently used
/// entries at once; then the normal budget comes back (so the next photos cache as before).
const SQUEEZE_BYTES: usize = 16 << 20;

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
    fn trimming_an_empty_session_is_harmless() {
        let mut s = Session::new();
        let t = trim_caches(&mut s);
        assert_eq!(t, Trimmed { before: 0, after: 0 });
    }
}
