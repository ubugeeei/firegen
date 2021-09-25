use std::collections::HashMap;

/**
 * Collections
 */
pub type Collections = Vec<Collection>;
#[derive(Debug, PartialEq)]
struct Collection {
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
pub struct key {
    pub name: String,
    pub optional: Boolean,
}
pub enum Value {
    Data(HashMap<String, DataType>),
    Node(Vex<Box<DocumentNode>>),
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
    pub __rule_name__: String,
    pub get: Option<String>,
    pub list: Option<String>,
    pub create: Option<String>,
    pub update: Option<String>,
    pub delete: Option<String>,
}
