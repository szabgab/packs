use std::path::Path;

use crate::packs::ProcessedFile;

use super::{CacheResult, EmptyCacheEntry};

pub trait Cache {
    fn get(&self, path: &Path) -> CacheResult;

    fn write(
        &self,
        empty_cache_entry: &EmptyCacheEntry,
        processed_file: &ProcessedFile,
    );
}
