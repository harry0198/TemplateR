

pub enum ASTNode {
    TextNode(TextNode),
    VariableNode(VariableNode)
}

pub struct TextNode {
    pub children: Vec<ASTNode>,
    pub content: String
}

pub struct VariableNode {
    pub variable: String
}

