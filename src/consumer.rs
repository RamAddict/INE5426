use std::cell::RefCell;
use std::fmt::Binary;
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
use crate::ast::AssignmentStatement;
use crate::ast::BinaryExpression;
use crate::ast::BlockStatement;
use crate::ast::BreakStatement;
use crate::ast::CallExpression;
use crate::ast::Class;
use crate::ast::ElementAccessExpression;
use crate::ast::ExpressionStatementValue;
use crate::ast::FloatLiteral;
use crate::ast::FunctionDeclaration;
use crate::ast::Identifier;
use crate::ast::InnerClass;
use crate::ast::IntegerLiteral;
use crate::ast::NewExpression;
use crate::ast::NullLiteral;
use crate::ast::Operation;
use crate::ast::PrintStatement;
use crate::ast::Program;
use crate::ast::ProgramValue;
use crate::ast::ReturnStatement;
use crate::ast::Statement;
use crate::ast::StringLiteral;
use crate::ast::UnaryExpression;
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

    // fn funcdef(input: Node) -> Result<FunctionDeclaration> {
    //     let mut children = input.children();
    //     let id = children.nth(1).unwrap();
    //     {
    //         let mut symbol_table = input.user_data().borrow_mut();
    //         symbol_table.add_new_symbol(Symbol {lexeme: id.clone().as_str(), attributes: SymbolAttributes::Function(

    //         )})
    //     }
    //     let maybe_param_list = children.nth(1).unwrap();
    //     match maybe_param_list.as_rule() {
    //         Rule::paramlist => {

    //         },
    //         Rule::kw_paren_close => {

    //         }
    //     }

    // }

    // fn funclist(input: Node) -> Result<Vec<FunctionDeclaration>> {
    //     let mut children = input.children();
    //     let func = Self::funcdef(children.next().unwrap())?;
    //     if let Some(tail) = children.next() {
    //         let mut func_def_list = Self::funclist(tail)?;
    //         func_def_list.insert(0, func);
    //         Ok(func_def_list)
    //     } else {
    //         Ok(vec![func])
    //     }
    // }

    fn program(input: Node) -> Result<ASTNodeValue> {
        // Parse Children
        let mut children = input.children();
        let child = children.next().unwrap();
        // Create Parent Node
        let ast = ASTNodeValue::Program(Program(
            // Parse Block
            match child.as_rule() {
                Rule::statement => {
                    let parsed = Self::statement(child)?;
                    ProgramValue::Statement(parsed)
                },
                // Rule::funclist => {
                //     input.user_data().borrow_mut().descend_scope();
                //     let parsed = Self::funclist(child)?;
                //     input.user_data().borrow_mut().ascend_scope();
                //     ProgramValue::FuncList(parsed)
                // }
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
                let parsed = Self::atribstat(fst_child)?;
                Statement::Assignment(parsed)
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
        // Prepare Data
        let symbol_name = input.as_str().to_string();
        // Check Undefined
        let symbol_table = input.user_data().clone();
        let mut borrowed = symbol_table.borrow_mut();
        match borrowed.lookup_mut(&symbol_name) {
            // Return Success
            Some(_) => Ok(Identifier(symbol_name)),
            // Return Undefined
            None => Err(gen_err_undefined(&input))
        }
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
        let identifier_node = children.next().unwrap();
        let vd_array = children.next();
        let class = match vd_array {
            Some(array_node) => Class::Array(inner_class, Self::vardecl_array(array_node)?),
            None => Class::Simple(inner_class)
        };
        // Insert Into Symbol Table
        let mut result: Option<std::result::Result<_, crate::stb::Error>> = None;
        {
            let mut symbol_table = input.user_data().borrow_mut();
            result = Some(symbol_table.add_new_symbol(Symbol {
                lexeme: identifier_node.as_str().to_string(),
                attributes: SymbolAttributes::Identifier(class.clone())
            }));
        }
        match result.unwrap() {
            Ok(_) => {
                Ok(VariableDeclaration(Self::ident(identifier_node)?, class))
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

    fn float_constant(input: Node) -> Result<FloatLiteral> {
        let value: f64 = input.as_str().parse().map_err(|_| { gen_err_str(&input, "Invalid float value".to_string()) })?;
        Ok(FloatLiteral(value))
    }
    fn int_constant(input: Node) -> Result<IntegerLiteral> {
        let value: isize = isize::from_str_radix(input.as_str(), 10).map_err(|_| { gen_err_str(&input, "Invalid int value".to_string()) })?;
        Ok(IntegerLiteral(value))
    }
    fn string_constant(input: Node) -> Result<StringLiteral> {
        Ok(StringLiteral(input.as_str().to_string()))
    }

    fn factor(input: Node) -> Result<ExpressionStatementValue> {
        let mut children = input.children();
        let fst_child = children.next().unwrap();
        match fst_child.as_rule() {
            Rule::float_constant => Ok(ExpressionStatementValue::Float(Self::float_constant(fst_child)?)),
            Rule::int_constant => Ok(ExpressionStatementValue::Integer(Self::int_constant(fst_child)?)),
            Rule::string_constant => Ok(ExpressionStatementValue::String(Self::string_constant(fst_child)?)),
            Rule::kw_null => Ok(ExpressionStatementValue::Null(NullLiteral)),
            Rule::lvalue => Ok(Self::lvalue(fst_child)?),
            Rule::kw_paren_open => Ok(Self::numexpression(children.next().unwrap())?),
            _ => Err(gen_err_invalid_token(&input))
        } 
    }

    fn sum_sub_op(input: Node) -> Result<Operation> {
        match input.as_str() {
            "+" => Ok(Operation::Plus),
            "-" => Ok(Operation::Minus),
            _ => Err(gen_err_invalid_token(&input))
        }
    }

    fn mult_div_mod_op(input: Node) -> Result<Operation> {
        match input.as_str() {
            "*" => Ok(Operation::Mult),
            "/" => Ok(Operation::Div),
            "%" => Ok(Operation::Mod),
            _ => Err(gen_err_invalid_token(&input))
        }
    }

    fn rel_op(input: Node) -> Result<Operation> {
        match input.as_str() {
            ">=" => Ok(Operation::Gte),
            ">" => Ok(Operation::Gt),
            "<=" => Ok(Operation::Lte),
            "<" => Ok(Operation::Lt),
            "==" => Ok(Operation::Eq),
            "!=" => Ok(Operation::Ne),
            _ => Err(gen_err_invalid_token(&input))
        }
    }

    fn unaryexpr(input: Node) -> Result<ExpressionStatementValue> {
        let mut children = input.children();
        let fst_child = children.next().unwrap();
        match fst_child.as_rule() {
            Rule::factor => Ok(Self::factor(fst_child)?),
            Rule::sum_sub_op => {
                let parsed_op = Self::sum_sub_op(fst_child)?;
                let factor = Self::factor(children.next().unwrap())?;
                Ok(ExpressionStatementValue::Unary(UnaryExpression(parsed_op, Box::from(factor))))
            }
            _ => Err(gen_err_invalid_token(&input))
        } 
    }

    fn term(input: Node) -> Result<ExpressionStatementValue> {
        let mut children = input.children();
        let unaryexp = Self::unaryexpr(children.next().unwrap())?;
        if let Some(term_mult_div_mod) = children.next() {
            let mut children_tmdm = term_mult_div_mod.children();
            let operation = Self::mult_div_mod_op(children_tmdm.next().unwrap())?;
            let right_uexpr = Self::unaryexpr(children_tmdm.next().unwrap())?;
            if let Some(tail) = children_tmdm.next() {
                let parsed = loop_tail_unaryexpr(right_uexpr, tail)?;
                Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(unaryexp), Box::from(parsed))))
            } else {
                Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(unaryexp), Box::from(right_uexpr))))
            }

        } else {
            Ok(unaryexp)
        }
    }

    fn numexpression(input: Node) -> Result<ExpressionStatementValue> {
        let mut children = input.children();
        let term = Self::term(children.next().unwrap())?;
        if let Some(add_sub_term_tail) = children.next() {
            let mut children_astt = add_sub_term_tail.children();
            let operation = Self::sum_sub_op(children_astt.next().unwrap())?;
            let right_term = Self::term(children_astt.next().unwrap())?;
            if let Some(tail) = children_astt.next() {
                let parsed = loop_tail_term(right_term, tail)?;
                Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(term), Box::from(parsed))))
            } else {
                Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(term), Box::from(right_term))))
            }
        } else {
            Ok(term)
        }
    }

    fn numexpression_array(input: Node) -> Result<Vec<ExpressionStatementValue>> {
        let mut children = input.children();
        let numexp = Self::numexpression(children.nth(1).unwrap())?;
        if let Some(tail) = children.nth(3) {
            let mut exp_list = Self::numexpression_array(tail)?;
            exp_list.insert(0, numexp);
            Ok(exp_list)
        } else {
            Ok(vec![numexp])
        }
    }

    fn lvalue(input: Node) -> Result<ExpressionStatementValue> {
        let mut children = input.children();
        // Get Identifier
        let ident_node = children.next().unwrap();
        let ident = Self::ident(ident_node.clone())?;
        if let Some(idx_tail) = children.next() {
            // Indexing Identifier
            // Check Type
            let mut attributes: Option<SymbolAttributes> = None;
            let mut symbol_table = input.user_data().clone();
            attributes = Some(symbol_table.borrow_mut().lookup_mut(&ident.0).unwrap().attributes.clone());
            match attributes {
                Some(SymbolAttributes::Identifier(class)) => { 
                    match class {
                        Class::Array(_, _) => {
                            // Get Array Values
                            let numexp_array = Self::numexpression_array(idx_tail)?;
                            // Build Expression
                            Ok(ExpressionStatementValue::ElementAccess(ElementAccessExpression(Box::from(ExpressionStatementValue::Identifier(ident)), numexp_array)))
                        },
                        _ => return Err(gen_err_str(&input, "Identifier is not an array type".to_string()))
                    }
                }, 
                _ => return Err(gen_err_str(&ident_node, "Symbol is not valid for this operation".to_string()))
            }
        } else {
            // Not Indexing
            Ok(ExpressionStatementValue::Identifier(ident))
        }
    }

    fn expression(input: Node) -> Result<ExpressionStatementValue> {
        let mut children = input.children();
        let numexp = Self::numexpression(children.next().unwrap())?; 
        if let Some(rel_op) = children.next() {
            let rexp = Self::numexpression(children.next().unwrap())?;
            let operation = Self::rel_op(rel_op)?;
            Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(numexp),  Box::from(rexp))))
        } else {
            Ok(numexp)
        }
    }

    fn allocexpression(input: Node) -> Result<ExpressionStatementValue> {
        let mut children = input.children();
        let inner_class = Self::gp_type(children.nth(1).unwrap())?;
        let numexp_arr = Self::numexpression_array(children.next().unwrap())?;
        Ok(ExpressionStatementValue::New(NewExpression(Class::Array(inner_class, vec![0usize]), numexp_arr)))
    }

    fn paramlistcall(input: Node) -> Result<Vec<Identifier>> {
        let mut children = input.children();
        let param = Self::ident(children.next().unwrap())?;
        if let Some(tail) = children.nth(1) {
            let mut paramlist = Self::paramlistcall(tail)?;
            paramlist.insert(0, param);
            Ok(paramlist)
        } else {
            Ok(vec![param])
        }
    }

    fn funccall(input: Node) -> Result<ExpressionStatementValue> {
        let mut children = input.children();
        let id = Self::ident(children.next().unwrap())?;
        if let Some(tail) = children.nth(1) {
            let params = Self::paramlistcall(tail)?;
            Ok(ExpressionStatementValue::Call(CallExpression(id, params)))
        } else {
            Ok(ExpressionStatementValue::Call(CallExpression(id, Vec::new())))
        }
    }

    fn atribstat(input: Node) -> Result<AssignmentStatement> {
        let mut children = input.children();
        let lvalue = Self::lvalue(children.next().unwrap())?;
        let rvalue = children.nth(1).unwrap();
        match rvalue.as_rule() {
            Rule::expression => {
                let parsed = Self::expression(rvalue)?;
                Ok(AssignmentStatement(Box::from(lvalue), Box::from(parsed)))
            },
            Rule::allocexpression => {
                let parsed = Self::allocexpression(rvalue)?;
                Ok(AssignmentStatement(Box::from(lvalue), Box::from(parsed)))
            }
            _ => Err(gen_err_invalid_token(&input))
        }
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
fn gen_err_undefined(input: &Node) -> pest::error::Error<Rule> {
    Error::new_from_span(
        ErrorVariant::CustomError {
            message: "Use of undefined indentifier".to_string(),
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

fn gen_err_str(input: &Node, message: String) -> pest::error::Error<Rule> {
    Error::new_from_span(
        ErrorVariant::CustomError {
            message: message,
        },
        input.as_span(),
    )
}

fn loop_tail_unaryexpr(upper_term: ExpressionStatementValue, node: Node) -> Result<ExpressionStatementValue> {
    let mut children = node.children();
    let operation = ParserCC20211A::mult_div_mod_op(children.next().unwrap())?;
    let unaryexpr = ParserCC20211A::unaryexpr(children.next().unwrap())?;
    if let Some(tail) = children.next() {
        Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(upper_term), Box::from(loop_tail_unaryexpr(unaryexpr, tail)?))))
    } else {
        Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(upper_term), Box::from(unaryexpr))))
    }
}

fn loop_tail_term(upper_term: ExpressionStatementValue, node: Node) -> Result<ExpressionStatementValue> {
    let mut children = node.children();
    let operation = ParserCC20211A::sum_sub_op(children.next().unwrap())?;
    let term = ParserCC20211A::term(children.next().unwrap())?;
    if let Some(tail) = children.next() {
        Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(upper_term), Box::from(loop_tail_term(term, tail)?))))
    } else {
        Ok(ExpressionStatementValue::Binary(BinaryExpression(operation, Box::from(upper_term), Box::from(term))))
    }
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

#[test]
fn test_syntax_attribstat() {
    let symbol_table = Rc::new(RefCell::new(SymbolTableManager::new()));
    let parsed = ParserCC20211A::parse_with_userdata(Rule::program,"
        {
            int zuleide[2][3];
            string onix;
            onix = \"mucha lucha\";
            int k;
            k = 2;
            k = 5 + 3 + 4 * 2 + 9;
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
