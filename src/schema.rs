use std::collections::HashMap;

pub type FiregenCollections = Vec<FiregenCollection>;
#[derive(Debug, PartialEq)]
struct FiregenCollection {
    __collection_name__: String,
    document: DocumentNode,
    rule: Rule,
}

/**
 * Rules
 */
pub struct Rule {
    __rule_name__: String,
    get: Option<String>,
    list: Option<String>,
    create: Option<String>,
    update: Option<String>,
    delete: Option<String>,
}

/**
 * Documents
 */
// TODO: sub collection name対応
pub enum DocumentNode {
    value(DataMap),
    children(Box<DocumentNode>),
}
pub type DataMap = HashMap<String, Hoge>;
pub enum Hoge {
    DataType(DataType),
    SubCollectionName(String),
}
pub enum DataType {
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
