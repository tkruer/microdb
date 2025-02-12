use microdb::MicroDB;
use std::env;

fn main() {
    let mut database = MicroDB::new().builder(env::args().collect());

    database.run();
}
