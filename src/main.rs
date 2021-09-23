mod read_schema;
fn main() {
    let file_path = String::from("playground/schema.fireSchema");
    let result = read_schema::read_schema(&file_path);

    println!("{}", result.unwrap());
}
