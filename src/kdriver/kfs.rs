// Kernel File System

// Holds all the meta data about a drive
pub struct MetaBlock {
    pub free_space: u64, // Free drive space left in bytes
    pub used_space: u64,
}

pub fn get_metadata() -> MetaBlock {
    let meta_data = MetaBlock {
        free_space: 10,
        used_space: 10,
    };

    meta_data.free_space;
    meta_data.used_space;
    meta_data
}
