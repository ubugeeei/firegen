extern crate yaml_rust;
use yaml_rust::YamlLoader;

use super::fs::read_file;

#[derive(Debug, PartialEq)]
pub struct FiregenConfigs {
    pub schema_path: String,
    pub export: FiregenExportConfig,
}

#[derive(Debug, PartialEq)]
pub struct FiregenExportConfig {
    pub rules_path: String,
    pub target: String,
    pub client_code_path: String,
    pub types_path: String,
}

pub fn read_config_yml() -> FiregenConfigs {
    let mut firegen_configs = FiregenConfigs {
        schema_path: String::from("schema/**/**.fireSchema"),
        export: FiregenExportConfig {
            rules_path: String::from("generated/firestore.rules"),
            target: String::from("ts"),
            client_code_path: String::from("generated/firestoreClient.ts"),
            types_path: String::from("generated/firestoreClient.d.ts"),
        },
    };

    // TODO: path
    let file_path = "playground/firegen.yml";
    let contents = read_file(file_path).ok().unwrap();

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];

    // TODO: override
    // adapt
    // match &doc["schema_path"].as_str() {
    //     Some(path) => firegen_configs.schema_path = path.to_string(),
    //     None => {}
    // }
    // match &doc["export"].as_str() {
    // Some() => {firegen_configs.schema_path = path.to_string()},
    // None => {}
    // }
    firegen_configs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config_yml() {
        let expected = FiregenConfigs {
            schema_path: String::from("schema/**/**.fireSchema"),
            export: FiregenExportConfig {
                rules_path: String::from("generated/firestore.rules"),
                target: String::from("ts"),
                client_code_path: String::from("generated/firestoreClient.ts"),
                types_path: String::from("generated/firestoreClient.d.ts"),
            },
        };
        let result = read_config_yml();

        assert_eq!(expected, result)
    }
}
