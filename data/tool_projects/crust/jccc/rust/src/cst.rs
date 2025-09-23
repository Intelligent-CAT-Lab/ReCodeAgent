use crate::list::List;
/// Represents different node types in the AST/CST.
#[derive(Debug)]
pub enum NodeType {
NT_STMT,
NT_EXPR,
NT_BLOCK_STMT,
NT_RETURN_STMT,
NT_FUNCDECL,
NT_FUNCCALL,
NT_LITERAL,
}
/// A block statement containing a list of statements.
#[derive(Debug)]
pub struct BlockStatement {
pub stmts: List,
}
/// A function declaration, including its body and name.
#[derive(Debug)]
pub struct FunctionDeclaration {
pub body: BlockStatement,
pub name: String,
}
/// A top-level declaration node that holds a function declaration in this simplified design.
#[derive(Debug)]
pub struct TopLevelDeclaration {
pub fd: FunctionDeclaration,
pub node_type: NodeType,
}
/// A function call node with a simple string name.
#[derive(Debug)]
pub struct FunctionCall {
pub name: String,
}
/// An expression node that can hold either a function call or a literal string.
#[derive(Debug)]
pub struct Expression {
pub fc: Option<FunctionCall>,
pub literal: Option<String>,
pub node_type: NodeType,
}
/// A file tree representing a list of top-level declarations.
#[derive(Debug)]
pub struct ConcreteFileTree {
pub decls: List,
}