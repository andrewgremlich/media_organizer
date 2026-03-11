use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use lazy_static::lazy_static;

static SAVED_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static IGNORED_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SUCCESSFULLY_COMPARED_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static IDENTICAL_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SAME_NAME_DIFF_CONTENT_COUNTER: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
static ref UNSUPPORTED_TYPES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub(crate) fn increment_saved_file_counter() {
    SAVED_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub fn get_saved_file_counter() -> usize {
    SAVED_FILE_COUNTER.load(Ordering::SeqCst)
}

pub(crate) fn increment_ignored_file_counter() {
    IGNORED_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub fn get_ignored_file_counter() -> usize {
    IGNORED_FILE_COUNTER.load(Ordering::SeqCst)
}

pub(crate) fn increment_successfully_compared_file_counter() {
    SUCCESSFULLY_COMPARED_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub fn get_successfully_compared_file_counter() -> usize {
    SUCCESSFULLY_COMPARED_FILE_COUNTER.load(Ordering::SeqCst)
}


pub(crate) fn add_unsupported_type(unsupported_type: String) {
    let mut set = UNSUPPORTED_TYPES.lock().unwrap();
    set.insert(unsupported_type);
}

pub fn get_unsupported_types() -> String {
    let set = UNSUPPORTED_TYPES.lock().unwrap();
    set.to_owned().into_iter().collect::<Vec<_>>().join(", ")
}

pub(crate) fn increment_identical_file_counter() {
    IDENTICAL_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub fn get_identical_file_counter() -> usize {
    IDENTICAL_FILE_COUNTER.load(Ordering::SeqCst)
}

pub(crate) fn increment_same_name_diff_content_counter() {
    SAME_NAME_DIFF_CONTENT_COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub fn get_same_name_diff_content_counter() -> usize {
    SAME_NAME_DIFF_CONTENT_COUNTER.load(Ordering::SeqCst)
}