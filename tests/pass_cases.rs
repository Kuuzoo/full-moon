use full_moon::{
    ast, print,
    tokenizer::{self, Token},
};
use pretty_assertions::assert_eq;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_pass_cases() {
    for entry in fs::read_dir("./tests/cases/pass").expect("couldn't read directory") {
        let entry = entry.unwrap();
        let path = entry.path();
        let source = fs::read_to_string(path.join("source.lua")).expect("couldn't read source.lua");

        let tokens = tokenizer::tokens(&source).expect("couldn't tokenize");

        let tokens_path = path.join("tokens.json");
        if let Ok(tokens_file) = fs::read_to_string(&tokens_path) {
            let expected_tokens: Vec<Token> =
                serde_json::from_str(&tokens_file).expect("couldn't deserialize tokens file");
            assert_eq!(tokens, expected_tokens);
        } else {
            let mut file = File::create(&tokens_path).expect("couldn't write tokens file");
            file.write_all(
                serde_json::to_string_pretty(&tokens)
                    .expect("couldn't serialize")
                    .as_bytes(),
            )
            .expect("couldn't write to tokens file");
        }

        let nodes = ast::nodes(&tokens).expect("couldn't make ast");
        let ast = ast::Ast {
            nodes,
            tokens: &tokens,
        };

        assert_eq!(print(&ast), source);
    }
}
