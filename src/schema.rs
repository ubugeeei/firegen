use std::collections::HashMap;

/**
 * Collections
 */
pub type Collections = Vec<Collection>;
#[derive(Debug, PartialEq)]
struct Collection {
    __collection_name__: String,
    document: Vec<Data>,
    rules_names: Vec<String>,
}

/**
 * Documents
 */
#[derive(Debug, PartialEq)]
pub struct Data {
    key: String,
    value: Value,
}
pub enum Value {
    Data(HashMap<String, DataType>),
    Node(Vex<Box<DocumentNode>>)
}
pub enum DataType {
    DataType(FirestoreDataType),
    SubCollectionName(String),
}
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
    __rule_name__: String,
    get: Option<String>,
    list: Option<String>,
    create: Option<String>,
    update: Option<String>,
    delete: Option<String>,
}