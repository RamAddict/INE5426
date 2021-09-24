use std::cell::RefCell;
use std::fmt::write;
use std::fmt::Display;
use std::rc::Rc;

// Import Dependencies
use crate::ParserCC20211;
use crate::Rule;
use pest::Parser;
use pest::Span;
use pest_ast::*;
use ptree::print_tree;
use ptree::TreeBuilder;
use ptree::TreeItem;
// Define Structs
#[derive(Clone, Copy, Debug)]
pub enum InnerClass {
    Integer,
    Float,
    String,
    Null,
}

#[derive(Clone, Debug)]
pub enum Class {
    Simple(InnerClass),
    Array(InnerClass, Vec<usize>),
    Undefined,
}

// #[derive(Clone, Debug)]
// pub struct Identifier<'i> {
//     pub name: String,
//     pub class: Class,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct LiteralInteger<'i> {
//     pub value: isize,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct LiteralFloat<'i> {
//     pub value: f64,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct LiteralString<'i> {
//     pub value: String,
//     pub span: Span<'i>,
// }
// #[derive(Clone, Debug)]
// pub struct LiteralNull<'i> {
//     pub span: Span<'i>,
// }
// #[derive(Clone, Debug)]
// pub struct VarDeclaration<'i> {
//     pub id: Identifier<'i>,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct Factor<'i> {
//     pub value: FactorValue<'i>,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub enum FactorValue<'i> {
//     LitInt(LiteralInteger<'i>),
//     LitFloat(LiteralFloat<'i>),
//     LitStr(LiteralString<'i>),
//     LitNull(LiteralNull<'i>),
//     LitValue(LValue<'i>),
//     LitNumExp(Box<NumericExpression<'i>>),
// }

// #[derive(Clone, Debug)]
// pub enum OpSumSub {
//     Sum,
//     Sub,
// }

// #[derive(Clone, Debug)]
// pub enum OpMultDivMod {
//     Mult,
//     Div,
//     Mod,
// }

// #[derive(Clone, Debug)]
// enum KWord {
//     print,
//     read,
//     retur,
// }

// #[derive(Clone, Debug)]
// pub enum OpRel {
//     Lte,
//     Gte,
//     Lt,
//     Gt,
//     Eq,
//     Ne,
// }

// #[derive(Clone, Debug)]
// pub struct UnaryExpression<'i> {
//     pub factor: Factor<'i>,
//     pub op: Option<OpSumSub>,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct Term<'i> {
//     pub unary_exp: UnaryExpression<'i>,
//     pub mult_div_mod_term: Option<MultDivModTerm<'i>>,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct SumSubTerm<'i> {
//     pub op: OpSumSub,
//     pub term: Term<'i>,
//     pub chain: Option<Box<SumSubTerm<'i>>>,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct MultDivModTerm<'i> {
//     pub op: OpMultDivMod,
//     pub unary_exp: UnaryExpression<'i>,
//     pub chain: Option<Box<MultDivModTerm<'i>>>,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct NumericExpression<'i> {
//     pub value: Term<'i>,
//     pub sum_sub_term: Option<SumSubTerm<'i>>,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct LValue<'i> {
//     pub id: Identifier<'i>,
//     pub array_exp: Option<Vec<NumericExpression<'i>>>,
//     pub span: Span<'i>,
// }

// #[derive(Clone, Debug)]
// pub struct Expression<'i> {
//     pub num_exp: NumericExpression<'i>,
//     pub rel: Option<(OpRel, NumericExpression<'i>)>,
//     pub span: Span<'i>,
// }
// // Define Traits
// trait Typed {
//     fn get_type(&self) -> &Class;
// }
// impl Typed for Identifier<'_> {
//     fn get_type(&self) -> &Class {
//         &self.class
//     }
// }
// impl Typed for LiteralInteger<'_> {
//     fn get_type(&self) -> &Class {
//         &Class::Simple(InnerClass::Integer)
//     }
// }

// impl Typed for LiteralFloat<'_> {
//     fn get_type(&self) -> &Class {
//         &Class::Simple(InnerClass::Float)
//     }
// }
// impl Typed for LiteralString<'_> {
//     fn get_type(&self) -> &Class {
//         &Class::Simple(InnerClass::String)
//     }
// }
// impl Typed for LiteralNull<'_> {
//     fn get_type(&self) -> &Class {
//         &Class::Simple(InnerClass::Null)
//     }
// }
// impl Typed for VarDeclaration<'_> {
//     fn get_type(&self) -> &Class {
//         self.id.get_type()
//     }
// }
// Define Pretty Print in a Tree Like Struct
// trait TreeItemBuilder {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder);
// }
// impl TreeItemBuilder for LiteralInteger<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.add_empty_child(format!("LiteralInteger({})", self.value));
//     }
// }
// impl TreeItemBuilder for LiteralFloat<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.add_empty_child(format!("LiteralFloat({})", self.value));
//     }
// }
// impl TreeItemBuilder for LiteralString<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.add_empty_child(format!("LiteralString(\"{}\")", self.value));
//     }
// }
// impl TreeItemBuilder for LiteralNull<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.add_empty_child(format!("LiteralNull()"));
//     }
// }
// impl TreeItemBuilder for Identifier<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.add_empty_child(format!("Identifier({}, {:?})", self.name, self.class));
//     }
// }
// impl TreeItemBuilder for VarDeclaration<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.begin_child(format!("VarDeclaration()"));
//         self.id.add_items_in_tree(tb);
//         tb.end_child();
//     }
// }
// impl TreeItemBuilder for Factor<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.begin_child(format!("Factor()"));
//         match &self.value {
//             FactorValue::LitFloat(val) => val.add_items_in_tree(tb),
//             FactorValue::LitInt(val) => val.add_items_in_tree(tb),
//             FactorValue::LitStr(val) => val.add_items_in_tree(tb),
//             FactorValue::LitNull(val) => val.add_items_in_tree(tb),
//             FactorValue::LitValue(_val) => (),// val.add_items_in_tree(tb),
//             FactorValue::LitNumExp(_val) => (), //val.add_items_in_tree(tb),
//         }
//         tb.end_child();
//     }
// }
// impl TreeItemBuilder for OpSumSub {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.add_empty_child(format!("OpSumSub({:?})", self));
//     }
// }
// impl TreeItemBuilder for OpMultDivMod {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.add_empty_child(format!("OpMultDivMod({:?})", self));
//     }
// }
// impl TreeItemBuilder for OpRel {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.add_empty_child(format!("OpRel({:?})", self));
//     }
// }
// impl TreeItemBuilder for UnaryExpression<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         let op_str = self.op.as_ref().and_then(|val| { Some(format!("{:?}", &val))}).unwrap_or(String::new());
//         tb.begin_child(format!("UnaryExpression({})", op_str));
//         self.factor.add_items_in_tree(tb);
//         tb.end_child();
//     }
// }
// impl TreeItemBuilder for Term<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.begin_child(format!("Term()"));
//         self.unary_exp.add_items_in_tree(tb);
//         if let Some(op_term) = self.mult_div_mod_term {
//             self.mult_div_mod_term.add_items_in_tree(tb);
//         }
//         tb.end_child();
//     }
// }
// impl TreeItemBuilder for MultDivModTerm<'_> {
//     fn add_items_in_tree(&self, tb: &mut TreeBuilder) -> () {
//         tb.begin_child(format!("MultDivModTerm({:?})", self.op));
//         self.factor.add_items_in_tree(tb);
//         tb.end_child();
//     }
// }
// #[test]
// fn test_pretty_print_lit() {
//     let lit_str = LiteralString {
//         span: Span::new("Olá, Mundo", 0, 10).unwrap(),
//         value: "Olá, Mundo".to_owned(),
//     };
//     let mut tree = TreeBuilder::new("".to_owned());
//     lit_str.add_items_in_tree(&mut tree);
//     print_tree(&(tree.build())).unwrap();
// }
// #[test]
// fn test_pretty_print_id() {
//     let id_str = Identifier {
//         name: "test".to_owned(),
//         span: Span::new("test", 0, 4).unwrap(),
//         class: Class::Simple(InnerClass::Float),
//     };
//     let mut tree = TreeBuilder::new("".to_owned());
//     id_str.add_items_in_tree(&mut tree);
//     print_tree(&(tree.build())).unwrap();
// }
// #[derive(Clone)]
// pub struct  PrintStatement<'i> {
//     // expression:
// }
// ===================================================================
pub enum Operation {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    And,
    Or
}
pub enum ASTNodeValue {
    Program(Program),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    FloatLiteral(FloatLiteral),
    NullLiteral(NullLiteral),
    Identifier(Identifier),
    VariableDeclaration(VariableDeclaration),
    FuncionDeclaration(FuncionDeclaration),
    IfStatement(IfStatement),
    BlockStatement(BlockStatement),
    ReturnStatement(ReturnStatement),
    PrintStatement(PrintStatement),
    ReadStatement(ReadStatement),
    ForStatement(ForStatement),
    BreakStatement(BreakStatement),
    ExpressionStatement(ExpressionStatement),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    AssignmentExpression(AssignmentExpression),
    CallExpression(CallExpression),
    NewExpression(NewExpression),
    ElementAccessExpression(ElementAccessExpression)
}

pub struct IntegerLiteral(pub isize);
pub struct FloatLiteral(pub f64);
pub struct StringLiteral(pub String);
pub struct NullLiteral;
pub struct Identifier(pub String);
pub struct VariableDeclaration(pub Identifier, pub Class);
pub struct FuncionDeclaration(pub Identifier, pub Vec<Identifier>);
pub struct IfStatement(pub BinaryExpression, pub Box<Statement>, pub Option<Box<Statement>>);
pub struct BlockStatement(pub Vec<Statement>);
pub struct ReturnStatement;
pub struct PrintStatement(pub ExpressionStatementValue);
pub struct ReadStatement(pub ExpressionStatementValue);
pub struct ForStatement(pub AssignmentExpression, pub Box<BinaryExpression>, pub AssignmentExpression, pub Box<Statement>);
pub struct BreakStatement;
pub enum ExpressionStatementValue {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
    Null(NullLiteral),
    Identifier(Identifier),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Assignment(AssignmentExpression),
    Call(CallExpression),
    New(NewExpression),
    ElementAccess(ElementAccessExpression),
}
pub struct BinaryExpression(pub Operation, pub Box<ExpressionStatementValue>, pub Box<ExpressionStatementValue>);
pub struct UnaryExpression(pub Operation, pub Box<ExpressionStatementValue>);
pub struct AssignmentExpression(pub Box<ExpressionStatementValue>, pub Box<ExpressionStatementValue>);
pub struct CallExpression(pub Box<ExpressionStatementValue>, pub Vec<Box<ExpressionStatementValue>>);
pub struct NewExpression(pub Box<ExpressionStatementValue>, pub Class, pub Vec<Box<ExpressionStatementValue>>);
pub struct ElementAccessExpression(pub Box<ExpressionStatementValue>, pub Vec<Box<ExpressionStatementValue>>);
pub struct ExpressionStatement(pub ExpressionStatementValue);
pub enum Statement {
    If(IfStatement),
    Block(BlockStatement),
    Return(ReturnStatement),
    Print(PrintStatement),
    Read(ReadStatement),
    For(ForStatement),
    Break(BreakStatement),
    VariableDeclaration(VariableDeclaration)
}
pub struct Program(pub BlockStatement);

pub struct ASTNode<'i> {
    // pub parent: Option<Rc<RefCell<ASTNode<'i>>>>,
    pub value: ASTNodeValue, 
    pub span: Span<'i>,
    pub children: Vec<Rc<RefCell<ASTNode<'i>>>>
}

pub fn gen_ptree_ast(tb: &mut TreeBuilder, node: ASTNodeValue) -> &mut TreeBuilder {
    match node {
        ASTNodeValue::Program(Program(block)) => {
            tb.begin_child("Program".to_string());
            gen_ptree_ast(tb, ASTNodeValue::BlockStatement(block));
            tb.end_child()
        },
        ASTNodeValue::IntegerLiteral(_) => todo!(),
        ASTNodeValue::StringLiteral(_) => todo!(),
        ASTNodeValue::FloatLiteral(_) => todo!(),
        ASTNodeValue::NullLiteral(_) => todo!(),
        ASTNodeValue::Identifier(Identifier(name)) => {
            tb.add_empty_child(format!("Identifier('{}')", name))
        },
        ASTNodeValue::VariableDeclaration(VariableDeclaration(id, class)) => {
            tb.begin_child("VariableDeclaration".to_string());
            tb.add_empty_child(format!("Class({:?})", class));
            gen_ptree_ast(tb, ASTNodeValue::Identifier(id));
            tb.end_child()
        },
        ASTNodeValue::FuncionDeclaration(_) => todo!(),
        ASTNodeValue::IfStatement(_) => todo!(),
        ASTNodeValue::BlockStatement(BlockStatement(statements)) => {
            tb.begin_child("BlockStatement".to_string());
            for statement in statements {
                match statement {
                    Statement::If(stmt) => gen_ptree_ast(tb, ASTNodeValue::IfStatement(stmt)),
                    Statement::Block(stmt) => gen_ptree_ast(tb, ASTNodeValue::BlockStatement(stmt)),
                    Statement::Return(stmt) => gen_ptree_ast(tb, ASTNodeValue::ReturnStatement(stmt)),
                    Statement::Print(stmt) => gen_ptree_ast(tb, ASTNodeValue::PrintStatement(stmt)),
                    Statement::Read(stmt) => gen_ptree_ast(tb, ASTNodeValue::ReadStatement(stmt)),
                    Statement::For(stmt) => gen_ptree_ast(tb, ASTNodeValue::ForStatement(stmt)),
                    Statement::Break(stmt) => gen_ptree_ast(tb, ASTNodeValue::BreakStatement(stmt)),
                    Statement::VariableDeclaration(stmt) => gen_ptree_ast(tb, ASTNodeValue::VariableDeclaration(stmt)),
                };
            }
            tb.end_child()
        },
        ASTNodeValue::ReturnStatement(_) => todo!(),
        ASTNodeValue::PrintStatement(_) => todo!(),
        ASTNodeValue::ReadStatement(_) => todo!(),
        ASTNodeValue::ForStatement(_) => todo!(),
        ASTNodeValue::BreakStatement(_) => todo!(),
        ASTNodeValue::ExpressionStatement(_) => todo!(),
        ASTNodeValue::BinaryExpression(_) => todo!(),
        ASTNodeValue::UnaryExpression(_) => todo!(),
        ASTNodeValue::AssignmentExpression(_) => todo!(),
        ASTNodeValue::CallExpression(_) => todo!(),
        ASTNodeValue::NewExpression(_) => todo!(),
        ASTNodeValue::ElementAccessExpression(_) => todo!(),
    }
}