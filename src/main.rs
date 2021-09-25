mod parse_schema_str;
mod read_schema;
mod schema;

#[allow(unused_imports)]
use combine::EasyParser;

fn main() {
    let dir_paths = vec!["playground/schema"];
    let result = read_schema::read_multi_schema(dir_paths);
    println!("read schema: {}", result);
}
