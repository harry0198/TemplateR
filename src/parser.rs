use crate::ast::{ASTNode, TextNode};

const OPEN_EXPRESSION_CHAR: char = '{';
const CLOSE_EXPRESSION_CHAR: char = '}';

fn tokenize(template: &str) -> Vec<String> {

    let mut tokens: Vec<String> = Vec::new();
    let mut characters = template.chars().peekable();
    let mut current_token = String::new();

    while let Some(c) = characters.next() {
        if let Some(&next_char) = characters.peek() {
            match (c, next_char) {
                (OPEN_EXPRESSION_CHAR, OPEN_EXPRESSION_CHAR) => {
                    characters.next(); // Consume
                    append_token(&mut current_token, &mut tokens);
                    current_token.push(c);
                    current_token.push(next_char);
                },
                (CLOSE_EXPRESSION_CHAR, CLOSE_EXPRESSION_CHAR) => {
                    characters.next(); // Consume
                    current_token.push(c);
                    current_token.push(next_char);
                    append_token(&mut current_token, &mut tokens);
                }
                _ => {
                    current_token.push(c);
                }
            }
        } else {
            current_token.push(c);
        }
    }
    append_token(&mut current_token, &mut tokens);

    tokens
}

pub fn parse(template: &str) -> ASTNode {
    let tokens = tokenize(template);
    let root = TextNode {
        children: vec!(), content: String::new()
    };

    ASTNode::TextNode(root)
}

fn append_token(current_token: &mut String, tokens: &mut Vec<String>) {
    if !current_token.is_empty() {
        tokens.push(current_token.clone());
        current_token.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn tokenize_simple() {
        let template = "{{expression}} here {{another expr}}";

        let tokens = tokenize(&template);

        assert_eq!(3, tokens.len());
        assert_eq!("{{expression}}", tokens.get(0).unwrap());
        assert_eq!(" here ", tokens.get(1).unwrap());
        assert_eq!("{{another expr}}", tokens.get(2).unwrap());
    }
}