mod read_schema;
fn main() {
    let dir_paths = vec!["playground"];
    let result = read_schema::read_multi_schema(dir_paths);

    println!("{}", result);
}
