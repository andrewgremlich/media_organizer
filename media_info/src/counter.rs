use std::sync::atomic::{AtomicUsize, Ordering};

static FALLBACK_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn increment_fallback_counter() {
    FALLBACK_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub fn get_fallback_counter() -> usize {
    FALLBACK_FILE_COUNTER.load(Ordering::SeqCst)
}