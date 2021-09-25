mod read_schema;
fn main() {
    let dir_paths = vec!["playground/schema"];
    let result = read_schema::read_multi_schema(dir_paths);

    println!("{}", result);
}
