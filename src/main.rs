mod read_schema;
mod parse_schema_str;
mod schema;
fn main() {
    let dir_paths = vec!["playground/schema"];
    let result = read_schema::read_multi_schema(dir_paths);

    println!("{}", result);
}
