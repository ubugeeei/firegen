use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct FirestoreSchemaTrees {
    pub Rules: FirestoreRule,
    pub Documents: FirestoreDocumentNode,
    pub Collections: FirestoreCollection,
}

/**
 * Rules
 */
#[derive(Debug, PartialEq)]
pub struct FirestoreRule {
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
pub struct FirestoreDocumentNode {
  document_type: FirestoreDocumentNodeType,
  children: Vec<Box<FirestoreDocumentNodeType>>,
}
pub enum FirestoreDocumentNodeType {
  FirestoreDocument(FirestoreDocument),
  FirestoreData(FirestoreData),
}
#[derive(Debug, PartialEq)]
pub struct FirestoreDocument {
  document_name: String,
  document_value: HashMap<String, DATA_TYPE>,
}
pub type FirestoreData = HashMap<String, DATA_TYPE>;

/**
 * Collections
 */
pub type FirestoreCollection = HashMap<String, String>;

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
