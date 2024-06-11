use crate::ast::{ASTNode, TextNode, VariableNode};

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
    let mut tokens = tokenize(template);
    let mut root = TextNode {
        children: vec!(), content: String::new()
    };

    for token in tokens.drain(..) {
        if token.starts_with(OPEN_EXPRESSION_CHAR.to_string().repeat(2).as_str()) && token.ends_with(&CLOSE_EXPRESSION_CHAR.to_string().repeat(2).as_str()) {
            let var: &str = &token[2..token.len()-2];
            root.children.push(ASTNode::VariableNode(
                VariableNode {
                    variable: var.to_owned()
                }
            ));
        } else {
            root.children.push(ASTNode::TextNode(
                TextNode {
                    children: vec!(),
                    content: token
                }
            ))
        }
    }


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
    use crate::ast::{TextNode, ASTNode};

    use super::{parse, tokenize};

    #[test]
    fn tokenize_simple() {
        let template = "{{expression}} here {{another expr}}";

        let tokens = tokenize(&template);

        assert_eq!(3, tokens.len());
        assert_eq!("{{expression}}", tokens.get(0).unwrap());
        assert_eq!(" here ", tokens.get(1).unwrap());
        assert_eq!("{{another expr}}", tokens.get(2).unwrap());
    }

    #[test]
    fn parse_simple() {
        let template = "Hello {{harry}}.";
    
        let ast = parse(template);
    
        match ast {
            ASTNode::TextNode(TextNode { children: nodes, content }) => {
                assert_eq!(nodes.len(), 3, "Invalid number of root children.");
                assert_eq!(content, "");
    
                match (&nodes[0], &nodes[1], &nodes[2]) {
                    (
                        ASTNode::TextNode(node1),
                        ASTNode::VariableNode(node2),
                        ASTNode::TextNode(node3)
                    ) => {
                        assert_eq!(node1.content, "Hello ");
                        assert_eq!(node2.variable, "harry");
                        assert_eq!(node3.content, ".");
                    }
                    _ => assert_eq!(-1, 1, "Nodes are of incorrect type or order!"),
                }
            }
            _ => assert_eq!(-1, 1, "Invalid root node."),
        }
    }
}