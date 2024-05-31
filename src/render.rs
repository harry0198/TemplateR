use crate::ast::*;

trait Render {
    fn render(&self) -> String;
}

impl Render for TextNode {
    fn render(&self) -> String {
        let mut rendered_content = self.content.clone();
        for child in &self.children {
            let content = match child {
                ASTNode::TextNode(node) => node.render(),
                ASTNode::VariableNode(node) => node.render()
            };
            rendered_content.push_str(content.as_str());
        }

        rendered_content
    }
}

impl Render for VariableNode {
    fn render(&self) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::render::Render;

    use super::TextNode;


    #[test]
    fn render_text_node_simple_string() {
        let node = TextNode {
            children: vec!(),
            content: String::from_str("hello there").unwrap()
        };

        assert_eq!("hello there", node.render());
    }

    #[test]
    fn render_variable_node() {

    }
}

