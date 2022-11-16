use std::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;

type SimpleCollection = HashMap<Vec<u8>, Vec<u8>>;
type Records = Arc<RwLock<SimpleCollection>>;

pub struct SuperDB {
    records: Records
}

unsafe impl Sync for SuperDB {}
unsafe impl Send for SuperDB {}

fn main() {
    let db = SuperDB {
        records: Arc::new(RwLock::new(SimpleCollection::new()))
    };
    env_logger::init();



}
