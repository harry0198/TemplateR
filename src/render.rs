use std::collections::HashMap;

use crate::ast::*;

trait Render {
    fn render(&self, map: &HashMap<&str, &str>) -> String;
}

impl Render for TextNode {
    fn render(&self, map: &HashMap<&str, &str>) -> String {
        let mut rendered_content = self.content.clone();
        for child in &self.children {
            let content = match child {
                ASTNode::TextNode(node) => node.render(map),
                ASTNode::VariableNode(node) => node.render(map)
            };
            rendered_content.push_str(&content);
        }

        rendered_content
    }
}

impl Render for VariableNode {
    fn render(&self, map: &HashMap<&str, &str>) -> String {
        match map.get(self.variable.as_str()) {
            None => String::new(),
            Some(value) => String::from(*value)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr};

    use crate::render::{Render, VariableNode};

    use super::TextNode;


    #[test]
    fn render_text_node_simple_string() {
        let node = TextNode {
            children: vec!(),
            content: String::from_str("hello there").unwrap()
        };
        let map: HashMap<&str, &str> = HashMap::new();

        assert_eq!("hello there", node.render(&map));
    }

    #[test]
    fn render_variable_node() {
        let node = VariableNode {
            variable: String::from("name")
        };
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("name", "harry");

        assert_eq!("harry", node.render(&map));
    }
}

