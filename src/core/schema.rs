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
        // fn type_of<T>(_: T) -> String {
        //     std::any::type_name::<T>().to_string()
        // }

        let key = Key::new(key_string, optional_string);
        let value: DataType = match value_string {
            "Text" => DataType::FirestoreDataType(FirestoreDataType::Text),
            "Int" => DataType::FirestoreDataType(FirestoreDataType::Int),
            "Float" => DataType::FirestoreDataType(FirestoreDataType::Float),
            "Boolean" => DataType::FirestoreDataType(FirestoreDataType::Boolean),
            "Bytes" => DataType::FirestoreDataType(FirestoreDataType::Bytes),
            "Array" => DataType::FirestoreDataType(FirestoreDataType::Array),
            "Map" => DataType::FirestoreDataType(FirestoreDataType::Map),
            "DateTime" => DataType::FirestoreDataType(FirestoreDataType::DateTime),
            "Geographical" => DataType::FirestoreDataType(FirestoreDataType::Geographical),
            "Reference" => DataType::FirestoreDataType(FirestoreDataType::Reference),
            "Null" => DataType::FirestoreDataType(FirestoreDataType::Null),
            _ => {
                DataType::SubCollectionName(value_string.to_string())
                // TODO: #14 nested data
            }
        };

        Data {
            key,
            value: Value::Data(value),
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
