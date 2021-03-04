// Kernel File System

// Holds all the meta data about a drive
pub struct MetaBlock {
    free_space: u64, // Free drive space left
    used_space: u64,
}
