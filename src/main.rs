#[allow(unused_imports)]
use firegen::core::{schema, parser};
use firegen::reader::read_schema;

#[allow(unused_imports)]
use combine::EasyParser;

fn main() {
    let dir_paths = vec!["playground/schema"];
    let result = read_schema::read_multi_schema(dir_paths);
    println!("read schema: {}", result);
}
