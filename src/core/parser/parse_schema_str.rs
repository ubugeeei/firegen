extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use combine::EasyParser;
use combine::{
    between,
    error::ParseError,
    many, many1, optional,
    parser::char::char,
    parser::char::{letter, newline, space},
    satisfy, Parser, Stream,
};

#[allow(unused_imports)]
use crate::core::schema::{Data, DataType, FirestoreDataType, Key, Value};

/**
 * main parser
 */
#[allow(unused)]
pub fn parse_schema_str(schema_str: &str) {}

/**
 * <text> Collection Users: User
 * ↓
 * <hash mao> {"Collections": {"Users": "User", "Todos" : "Todo"}}
 */
#[allow(unused)]
fn parse_schema_collection(collection_str: &str) {}

/**
 * <text> Document User{} \n Document Todo {}
 * ↓
 * <hash mao> {"Documents": {"User": {}, "Todo" : {}}}
 */
#[allow(unused)]
fn parse_schema_document(document_str: &str) {}

/**
 * <text> id: Int
 * ↓
 * <struct> {
 *    key: { name: "id", optional: false },
 *    value: Int
 * }
 */
#[allow(unused)]
pub fn parse_key_value<Input>() -> impl Parser<Input, Output = (String, String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let key_string_token = many1::<String, _, _>(letter());
    let optional_string_token = many::<String, _, _>(char('?').or(space().or(newline())));
    let value_token = many1::<String, _, _>(letter()).or(between(
        char('{'),
        char('}'),
        many1::<String, _, _>(satisfy(|c: char| c != '}')),
    ));
    (
        key_string_token,
        optional_string_token,
        many::<String, _, _>(space().or(newline())),
        char(':'),
        many::<String, _, _>(space().or(newline())),
        value_token,
    )
        .map(|v| (v.0, v.1, v.5))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_key_value() {
        assert_eq!(
            Ok((("id".to_string(), "".to_string(), "Int".to_string()), "")),
            parse_key_value().easy_parse("id: Int")
        )
    }
    #[test]
    fn test_parse_optional_key_value() {
        assert_eq!(
            Ok((("id".to_string(), "?".to_string(), "Int".to_string()), "")),
            parse_key_value().easy_parse("id?: Int")
        )
    }

    #[test]
    fn test_new_data_instance() {
        let input = "memo: Text";
        let expected = Data {
            key: Key {
                name: String::from("memo"),
                optional: false,
            },
            value: Value::Data(DataType::FirestoreDataType(FirestoreDataType::Text)),
        };

        // parse
        let parse_result = parse_key_value().easy_parse(input).ok().unwrap().0;
        let key_string = parse_result.0;
        let optional_string = parse_result.1;
        let value_string = parse_result.2;
        let result = Data::new(&key_string, &optional_string, &value_string);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_new_optional_data_instance() {
        let input = "memo?: Text";
        let expected = Data {
            key: Key {
                name: String::from("memo"),
                optional: true,
            },
            value: Value::Data(DataType::FirestoreDataType(FirestoreDataType::Text)),
        };

        // parse
        let parse_result = parse_key_value().easy_parse(input).ok().unwrap().0;
        let key_string = parse_result.0;
        let optional_string = parse_result.1;
        let value_string = parse_result.2;
        let result = Data::new(&key_string, &optional_string, &value_string);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_new_data_instance_sub_collection() {
        let input = "todos?: Todo";
        let expected = Data {
            key: Key {
                name: String::from("todos"),
                optional: true,
            },
            value: Value::Data(DataType::SubCollectionName("Todo".to_string())),
        };

        // parse
        let parse_result = parse_key_value().easy_parse(input).ok().unwrap().0;
        let key_string = parse_result.0;
        let optional_string = parse_result.1;
        let value_string = parse_result.2;
        let result = Data::new(&key_string, &optional_string, &value_string);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_parse_nested_key_value() {
        let input = "status: {isDone: Boolean\ncompletedDate: DateTime}";

        assert_eq!(
            Ok((("status".to_string(), "".to_string(), "isDone: Boolean\ncompletedDate: DateTime".to_string()), "")),
            parse_key_value().easy_parse(input)
        )
    }
}
