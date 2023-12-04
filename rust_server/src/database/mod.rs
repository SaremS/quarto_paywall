///Generic interface to interact with various databases
pub mod database_base;

///Concrete implementation of an in-memory database via HashMaps
///(primarily for testing)
pub mod in_memory_db;

pub use database_base::*;
pub use in_memory_db::*;
