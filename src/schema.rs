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

#[derive(Debug, PartialEq)]
pub struct Key {
    pub name: String,
    pub optional: bool,
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
type Rules = Vec<Rule>;
#[derive(Debug, PartialEq)]
pub struct Rule {
    pub __rule_name__: String,
    pub get: Option<String>,
    pub list: Option<String>,
    pub create: Option<String>,
    pub update: Option<String>,
    pub delete: Option<String>,
}
