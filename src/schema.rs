use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct FirestoreSchemaTrees {
    pub Rules: FireStoreRule,
    pub Documents: FireStoreDocumentNode,
    pub Collections: FireStoreCollection,
}

/**
 * Rules
 */
#[derive(Debug, PartialEq)]
pub struct FireStoreRule {
  rule_name: String,
  rule: HashMap<RULE_TYPE, String>,
}
pub enum RULE_TYPE {
  get,
  list,
  create,
  update,
  delete,
}

/**
 * Documents
 */
#[derive(Debug, PartialEq)]
pub struct FireStoreDocumentNode {
  document_type: FireStoreDocumentNodeType,
  children: Vec<Box<FireStoreDocumentNodeType>>,
}
pub enum FireStoreDocumentNodeType {
  FireStoreDocument(FireStoreDocument),
  FireStoreData(FireStoreData),
}
#[derive(Debug, PartialEq)]
pub struct FireStoreDocument {
  document_name: String,
  document_value: HashMap<String, DATA_TYPE>,
}
pub type FireStoreData = HashMap<String, DATA_TYPE>;

/**
 * Collections
 */
pub type FireStoreCollection = HashMap<String, String>;

/**
 * Firestore base types
 */
pub enum DATA_TYPE {
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
