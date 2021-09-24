use std::cell::RefCell;
use std::rc::Rc;

// Import Dependencies
// use crate::ast::*;
// use crate::ParserCC20211;
// use crate::Rule;
use pest::error::ErrorVariant;
use pest_consume::Error;
use pest_consume::Parser;
use ptree::TreeBuilder;
use ptree::print_tree;

// use crate::ast::ASTNode;
use crate::ast::ASTNodeValue;
use crate::ast::BlockStatement;
use crate::ast::BreakStatement;
use crate::ast::Class;
use crate::ast::Identifier;
use crate::ast::InnerClass;
use crate::ast::PrintStatement;
use crate::ast::Program;
use crate::ast::ReturnStatement;
use crate::ast::Statement;
use crate::ast::VariableDeclaration;
use crate::ast::gen_ptree_ast;
use crate::stb::Symbol;
use crate::stb::SymbolAttributes;
use crate::stb::SymbolTableManager;
// Define Types
#[allow(dead_code)]
type Result<T> = std::result::Result<T, Error<Rule>>;
#[allow(dead_code)]
type Node<'i> = pest_consume::Node<'i, Rule, Rc<RefCell<SymbolTableManager>>>;
// Define Structs
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ParserCC20211A;
// // Define Consumer
#[pest_consume::parser]
impl ParserCC20211A {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn program(input: Node) -> Result<ASTNodeValue> {
        // Parse Children
        let mut children = input.children();
        let child = children.next().unwrap();
        // Create Parent Node
        let ast = ASTNodeValue::Program(Program(
            // Parse Block
            match child.as_rule() {
                Rule::statement => {
                    input.user_data().borrow_mut().descend_scope();
                    let parsed = Self::statement(child)?;
                    input.user_data().borrow_mut().ascend_scope();
                    BlockStatement(vec![parsed])
                }
                // Rule::funclist
                _ => {
                    return Err(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: "Invalid Token".to_string(),
                        },
                        child.as_span(),
                    ))
                }
            },
        ));
        return Ok(ast);
    }

    fn statement(input: Node) -> Result<Statement> {
        let fst_child = input.children().next().unwrap();
        let ast = match fst_child.as_rule() {
            Rule::vardecl => {
                let parsed = Self::vardecl(fst_child)?;
                Statement::VariableDeclaration(parsed)
            },
            Rule::kw_cur_bracket_open => {
                // Block Statement
                let parsed = Self::statelist(input.children().nth(1).unwrap())?;
                Statement::Block(parsed)
            },
            Rule::returnstat => Statement::Return(ReturnStatement),
            // TODO: Check Break inside FOR Statement
            Rule::kw_break => Statement::Break(BreakStatement),
            Rule::atribstat => {
                let parsed = Self::atribstat(fast_child)?;
                Statement::
            }
            _ => {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Invalid Token".to_string(),
                    },
                    fst_child.as_span(),
                ))
            }
        };
        return Ok(ast);
    }

    fn gp_type(input: Node) -> Result<InnerClass> {
        Ok(match input.children().single()?.as_rule() {
            Rule::kw_int => InnerClass::Integer,
            Rule::kw_float => InnerClass::Float,
            Rule::kw_string => InnerClass::String,
            _ => return Err(gen_err_invalid_token(&input))
        })
    }

    fn ident(input: Node) -> Result<Identifier> {
        Ok(Identifier(input.as_str().to_string()))
    }

    fn vardecl_array(input: Node) -> Result<Vec<usize>> {
        let mut children = input.children();
        let parsed_value = usize::from_str_radix(children.nth(1).unwrap().as_str(), 10).unwrap();
        if let Some(tail) = children.nth(3) {
            let mut index_list = Self::vardecl_array(tail)?;
            index_list.insert(0, parsed_value);
            Ok(index_list)
        } else {
            Ok(vec![parsed_value])
        }
    }

    fn vardecl(input: Node) -> Result<VariableDeclaration> {
        let mut children = input.children();
        // Parse Data
        let inner_class = Self::gp_type(children.next().unwrap())?;
        let identifier = Self::ident(children.next().unwrap())?;
        let vd_array = children.next();
        let class = match vd_array {
            Some(array_node) => Class::Array(inner_class, Self::vardecl_array(array_node)?),
            None => Class::Simple(inner_class)
        };
        // Insert Into Symbol Table
        let mut symbol_table = input.user_data().borrow_mut();
        match symbol_table.add_new_symbol(Symbol {
            lexeme: identifier.0.clone(),
            attributes: SymbolAttributes::Identifier(class.clone())
        }) {
            Ok(_) => {
                Ok(VariableDeclaration(identifier, class))
            },
            Err(error) => Err(gen_err(&input, error))
        }
    }

    fn statelist(input: Node) -> Result<BlockStatement> {
        let mut children = input.children();
        // let mut symbol_table = input.user_data().borrow_mut();
        // Descend Scope
        {
            input.user_data().borrow_mut().descend_scope();
        }
        // Process Statement
        let stmt = Self::statement(children.next().unwrap())?;
        // Process Next Statements
        let mut stmts = vec![stmt];
        if let Some(tail) = children.next() {
            let mut current_tail = tail;
            loop {
                // Get Tail Statement
                let mut current_tail_children = current_tail.children();
                let new_stmt = Self::statement(current_tail_children.next().unwrap())?;
                stmts.push(new_stmt);
                // Check New Tail
                if let Some(new_tail) = current_tail_children.next() {
                    current_tail = new_tail
                } else {
                    break
                }
            }
        }
        // Ascend Scope
        {
            input.user_data().borrow_mut().ascend_scope();
        }
        // Return Values
        Ok(BlockStatement(stmts))
    }
}

#[allow(dead_code)]
fn gen_err_invalid_token(input: &Node) -> pest::error::Error<Rule> {
    Error::new_from_span(
        ErrorVariant::CustomError {
            message: "Invalid Token".to_string(),
        },
        input.as_span(),
    )
}
#[allow(dead_code)]
fn gen_err(input: &Node, error: crate::stb::Error) -> pest::error::Error<Rule> {
    Error::new_from_span(
        ErrorVariant::CustomError {
            message: format!("{:?}", error),
        },
        input.as_span(),
    )
}

#[test]
fn test_syntax_vardecl_array() {
    let symbol_table = Rc::new(RefCell::new(SymbolTableManager::new()));
    let parsed = ParserCC20211A::parse_with_userdata(Rule::program,"
        {
            int zuleide[2][3];
            string onix;
        }",
        Rc::clone(&symbol_table)
    ).unwrap();
    println!("{}\n", parsed);
    let root = parsed.single().unwrap();
    println!("{:?}", root.as_rule());
    let ast = ParserCC20211A::program(root).unwrap();
    println!("{:?}", symbol_table.borrow());
    print_tree( &gen_ptree_ast(&mut TreeBuilder::new("AST".to_string()), ast).build()).unwrap();

}

// #[test]
// fn test_syntax_factor_float() {
//     let parsed = ParserCC20211A::parse(Rule::factor, "1.66").unwrap();
//     println!("{}", parsed);
//     let ast = ParserCC20211A::factor(parsed.single().unwrap()).unwrap();
//     println!("{:?}", ast)
// }

// #[test]
// fn test_syntax_factor_array() {
//     let parsed = ParserCC20211A::parse(Rule::factor, "inpp[22]").unwrap();
//     println!("{}\n", parsed);
//     let ast = ParserCC20211A::factor(parsed.single().unwrap()).unwrap();
//     println!("{:?}", ast)
// }

#[test]
fn test_syntax_expressions() {
    let symbol_table = Rc::new(RefCell::new(SymbolTableManager::new()));
    let parsed = ParserCC20211A::parse_with_userdata(
        Rule::expression,
        "2 + 3 + 5",
        Rc::clone(&symbol_table),
    )
    .unwrap();
    println!("{}", symbol_table.borrow_mut().current_table_id());
    println!("{}\n", parsed);
    // let ast = ParserCC20211A::expression(parsed.single().unwrap()).unwrap();
    // println!("{:?}", ast)
}
// pub fn parse_from_program(stbm: SymbolTableManager, pairs: Pairs<Rule>) {
//     ParserCC20211A::parse_with_userdata();
//     // Check Children
//     // pairs.nex
// }
