/**
 * Collections
 */
pub type Collections = Vec<Collection>;
#[derive(Debug, PartialEq)]
pub struct Collection {
    pub __collection_name__: String,
    pub document: Vec<Data>,
    pub rules_names: Vec<String>,
}

/**
 * Documents
 */
#[derive(Debug, PartialEq)]
pub struct Data {
    pub key: Key,
    pub value: Value,
}
impl Data {
    pub fn new(key_string: &str, optional_string: &str, value_string: &str) -> Data {
        let key = Key::new(key_string, optional_string);
        let value = match value_string {
            "Text" => FirestoreDataType::Text,
            "Int" => FirestoreDataType::Int,
            "Float" => FirestoreDataType::Float,
            "Boolean" => FirestoreDataType::Boolean,
            "Bytes" => FirestoreDataType::Bytes,
            "Array" => FirestoreDataType::Array,
            "Map" => FirestoreDataType::Map,
            "DateTime" => FirestoreDataType::DateTime,
            "Geographical" => FirestoreDataType::Geographical,
            "Reference" => FirestoreDataType::Reference,
            "Null" => FirestoreDataType::Null,
            // TODO:
            _ => FirestoreDataType::Null,
            // _ => Value::Data(DataType::SubCollectionName(value_string.to_string())),
        };

        Data {
            key,
            // TODO:
            value: Value::Data(DataType::FirestoreDataType(value)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Key {
    pub name: String,
    pub optional: bool,
}
impl Key {
    fn new(key_string: &str, optional_string: &str) -> Key {

        let key_name = key_string.to_string();
        let optional = match optional_string {
            "?" => true,
            _ => false,
        };

        Key {
            name: key_name,
            optional,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Data(DataType),
    Node(Vec<Box<Data>>),
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    FirestoreDataType(FirestoreDataType),
    SubCollectionName(String),
}

#[derive(Debug, PartialEq)]
pub enum FirestoreDataType {
    Text,
    Int,
    Float,
    Boolean,
    Bytes,
    Array,
    Map,
    DateTime,
    Geographical,
    Reference,
    Null,
}

/**
 * Rules
 */
pub type Rules = Vec<Rule>;
#[derive(Debug, PartialEq)]
pub struct Rule {
    pub __rule_name__: String,
    pub get: Option<String>,
    pub list: Option<String>,
    pub create: Option<String>,
    pub update: Option<String>,
    pub delete: Option<String>,
}
