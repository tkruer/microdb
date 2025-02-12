use microdb::{Database, MicroDB};
use std::env;

fn main() {
    let mut database: Database = MicroDB::new().builder(env::args().collect());

    database.run();
}
