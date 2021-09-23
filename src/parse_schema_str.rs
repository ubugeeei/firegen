/**
 * main parser
 */
pub fn parse_schema_str(schema_str: &str) {}

/**
 * <text> Collection Users: User
 * ↓
 * <hash mao> {"Collections": {"Users": "User", "Todos" : "Todo"}}
 */
fn parse_schema_collection(collection_str: &str) {}

/**
 * <text> Document User{} \n Document Todo {}
 * ↓
 * <hash mao> {"Documents": {"User": {}, "Todo" : {}}}
 */
fn parse_schema_document(document_str: &str) {}

/**
 * <text> id: Int
 * ↓
 * <hash map> {'id': 'Int'}
 */
fn parse_schema_value_type(value_type_str: &str) {}

// TODO: unit test