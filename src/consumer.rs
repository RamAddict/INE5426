// Import Dependencies
use crate::ast::*;
use crate::ParserCC20211;
use pest::error::ErrorVariant;
use pest_consume;
use pest_consume::match_nodes;
use pest_consume::Error;
use pest_consume::Parser;
// use crate::Rule;
// Define Types
type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;
// Define Structs
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ParserCC20211A {}
// Define Consumer
#[pest_consume::parser]
impl ParserCC20211A {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn rel_op(input: Node) -> Result<OpRel> {
        match input.as_str() {
            ">" => Ok(OpRel::Gt),
            ">=" => Ok(OpRel::Gte),
            "<" => Ok(OpRel::Lt),
            "<=" => Ok(OpRel::Lte),
            "==" => Ok(OpRel::Eq),
            "!=" => Ok(OpRel::Ne),
            _ => Err(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: "Invalid OpRel".to_owned(),
                },
                input.as_span(),
            )),
        }
    }

    // fn expression(input: Node) -> Result<Expression> {
    //     Ok(match_nodes!(input.children();
    //         [numexpression(l_exp)] => Expression(l_exp, None),
    //         [numexpression(l_exp), rel_op(op), numexpression(r_exp)] => Expression(l_exp, Some((op, r_exp)))
    //     ))
    // }

    fn int_constant(input: Node) -> Result<LiteralInteger> {
        Ok(LiteralInteger {
            value: isize::from_str_radix(input.as_str(), 10).unwrap(),
            span: input.as_span(),
        })
    }

    fn float_constant(input: Node) -> Result<LiteralFloat> {
        Ok(LiteralFloat {
            value: input.as_str().parse().map_err(|_| {
                Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "Invalid value while parsing".to_owned(),
                    },
                    input.as_span(),
                )
            })?,
            span: input.as_span(),
        })
    }

    fn string_constant(input: Node) -> Result<LiteralString> {
        Ok(LiteralString {
            value: input.children().next().unwrap().as_str().to_owned(),
            span: input.as_span(),
        })
    }

    fn kw_bracket_open(input: Node) -> Result<()> {
        Ok(())
    }
    fn kw_bracket_close(input: Node) -> Result<()> {
        Ok(())
    }
    fn kw_paren_open(input: Node) -> Result<()> {
        Ok(())
    }
    fn kw_paren_close(input: Node) -> Result<()> {
        Ok(())
    }

    fn vardecl_array(input: Node) -> Result<Vec<usize>> {
        Ok(match_nodes!(input.children();
            [kw_bracket_open(_), int_constant(size), kw_bracket_close(_)] => vec![size.value as usize],
            [kw_bracket_open(_), int_constant(size), kw_bracket_close(_), vardecl_array(mut vec_sizings)] => {
                vec_sizings.insert(0, size.value as usize);
                vec_sizings
            }
        ))
    }

    fn kw_int(input: Node) -> Result<InnerClass> {
        Ok(InnerClass::Integer)
    }
    fn kw_float(input: Node) -> Result<InnerClass> {
        Ok(InnerClass::Float)
    }
    fn kw_string(input: Node) -> Result<InnerClass> {
        Ok(InnerClass::String)
    }
    fn kw_null(input: Node) -> Result<LiteralNull> {
        Ok(LiteralNull {
            span: input.as_span(),
        })
    }

    fn gp_type(input: Node) -> Result<InnerClass> {
        Ok(match_nodes!(input.children();
            [kw_int(node)] => node,
            [kw_float(node)] => node,
            [kw_string(node)] => node
        ))
    }

    fn vardecl(input: Node) -> Result<VarDeclaration> {
        let mut children = input.children();
        let var_type = Self::gp_type(children.next().unwrap())?;
        let identifier_node = children.next().unwrap();

        let identifier = Identifier {
            name: identifier_node.as_str().to_owned(),
            span: identifier_node.as_span(),
            class: match children.next() {
                Some(vardecl_array) => Class::Array(var_type, Self::vardecl_array(vardecl_array)?),
                None => Class::Simple(var_type),
            },
        };

        Ok(VarDeclaration {
            id: identifier,
            span: input.as_span(),
        })
    }

    fn ident(input: Node) -> Result<Identifier> {
        Ok(Identifier {
            name: input.as_str().to_owned(),
            class: Class::Undefined,
            span: input.as_span(),
        })
    }

    fn kw_plus(input: Node) -> Result<OpSumSub> {
        Ok(OpSumSub::Sum)
    }
    fn kw_minus(input: Node) -> Result<OpSumSub> {
        Ok(OpSumSub::Sub)
    }
    fn kw_mult(input: Node) -> Result<OpMultDivMod> {
        Ok(OpMultDivMod::Mult)
    }
    fn kw_div(input: Node) -> Result<OpMultDivMod> {
        Ok(OpMultDivMod::Div)
    }
    fn kw_mod(input: Node) -> Result<OpMultDivMod> {
        Ok(OpMultDivMod::Mod)
    }
    fn sum_sub_op(input: Node) -> Result<OpSumSub> {
        Ok(match_nodes!(input.into_children();
            [kw_plus(op)] => op,
            [kw_minus(op)] => op
        ))
    }
    fn mult_div_mod_op(input: Node) -> Result<OpMultDivMod> {
        Ok(match_nodes!(input.into_children();
            [kw_mult(op)] => op,
            [kw_div(op)] => op,
            [kw_mod(op)] => op
        ))
    }

    fn unaryexpr(input: Node) -> Result<UnaryExpression> {
        Ok(
            match_nodes!(input.children();
                [kw_plus(kp), factor(factor)] => UnaryExpression {
                    factor: factor,
                    op: Some(kp),
                    span: input.as_span(),
                },
                [kw_minus(kp), factor(factor)] => UnaryExpression {
                    factor: factor,
                    op: Some(kp),
                    span: input.as_span(),
                },
                [factor(factor)] => UnaryExpression {
                    factor: factor,
                    op: None,
                    span: input.as_span(),
                }
            )
        )
    }

    fn add_subtract_term(input: Node) -> Result<SumSubTerm> {
        Ok(match_nodes!(input.children();
            [sum_sub_op(op), term(term), add_subtract_term(chain)] => SumSubTerm {
                op: op,
                term: term,
                chain: Some(Box::from(chain)),
                span: input.as_span()
            },
            [sum_sub_op(op), term(term)] => SumSubTerm {
                op: op,
                term: term,
                chain: None,
                span: input.as_span()
            },
        ))
    }

    fn term_mult_div_mod(input: Node) -> Result<MultDivModTerm> {
        Ok(match_nodes!(input.children();
            [mult_div_mod_op(op), unaryexpr(uexp), term_mult_div_mod(chain)] => MultDivModTerm {
                op: op,
                unary_exp: uexp,
                chain: Some(Box::from(chain)),
                span: input.as_span()
            },
            [mult_div_mod_op(op), unaryexpr(uexp)] => MultDivModTerm {
                op: op,
                unary_exp: uexp,
                chain: None,
                span: input.as_span()
            },
        ))
    }

    fn term(input: Node) -> Result<Term> {
        Ok(
            match_nodes!(input.children();
                [unaryexpr(unaryexpr)] => Term {
                    unary_exp: unaryexpr,
                    span: input.as_span(),
                    mult_div_mod_term: None
                },
                [unaryexpr(unaryexpr), term_mult_div_mod(multdivmod_term)] => Term {
                    unary_exp: unaryexpr,
                    span: input.as_span(),
                    mult_div_mod_term: Some(multdivmod_term),
                }
            )
        )
    }

    fn numexpression(input: Node) -> Result<NumericExpression> {
        Ok(
            match_nodes!(input.children();
                [term(term_val)] => NumericExpression {
                    value: term_val,
                    span: input.as_span(),
                    sum_sub_term: None
                },
                [term(term_val), add_subtract_term(sumsub_term)] => NumericExpression {
                    value: term_val,
                    span: input.as_span(),
                    sum_sub_term: Some(sumsub_term)
                }
            )
        )
    }

    fn numexpression_array(input: Node) -> Result<Vec<NumericExpression>> {
        Ok(
            match_nodes!(input.children();
                [kw_bracket_open(_), numexpression(num_exp), kw_bracket_close(_), numexpression_array(mut num_exp_array)] => {
                    num_exp_array.insert(0, num_exp);
                    num_exp_array
                },
                [kw_bracket_open(_), numexpression(num_exp), kw_bracket_close(_)] => vec![num_exp]
            )
        )
    }

    fn lvalue(input: Node) -> Result<LValue> {
        Ok(match_nodes!(input.children();
            [ident(id), numexpression_array(num_exp_array)] => LValue { id, array_exp: Some(num_exp_array), span: input.as_span() },
            [ident(id)] => LValue { id, array_exp: None, span: input.as_span() },
        ))
    }

    fn factor(input: Node) -> Result<Factor> {
        Ok(match_nodes!(input.children();
            [float_constant(lit_val)] => Factor { value: FactorValue::LitFloat(lit_val), span: input.as_span() },
            [int_constant(lit_val)] => Factor { value: FactorValue::LitInt(lit_val), span: input.as_span() },
            [string_constant(lit_val)] => Factor { value: FactorValue::LitStr(lit_val), span: input.as_span() },
            [kw_null(lit_val)] => Factor { value: FactorValue::LitNull(lit_val), span: input.as_span() },
            [lvalue(val)] => Factor { value: FactorValue::LitValue(val), span: input.as_span() },
            [kw_paren_open(_), numexpression(num_exp), kw_paren_close(_)] => Factor { value: FactorValue::LitNumExp(Box::from(num_exp)) , span: input.as_span() }
        ))
    }

    fn expression(input: Node) -> Result<Expression> {
        Ok(match_nodes!(input.children();
            [numexpression(num_exp), rel_op(op), numexpression(num_exp_rel)] => Expression { num_exp: num_exp, rel: Some((op, num_exp_rel)), span: input.as_span() },
            [numexpression(num_exp)] => Expression { num_exp: num_exp, rel: None, span: input.as_span() },
        ))
    }

    // fn returnstat(input: Node) -> Result<KWord> {
    //     match input.as_str() {
    //         "return" => KWord::retur,
    //         "read" => KWord::read,
    //         "print" => KWord::print
    //         }
    // }

    // fn printstat(input: Node) -> Result<PrintStat> {
    //     Ok(
    //         match_nodes!(input.children();
    //             [kw_print(node1), Expression(node2)] => PrintStat(node2, None)
    //         )
    //     )
    // }

    // fn readstat(input: Node) -> Result<ReadStat> {
    //     Ok(
    //         match_nodes!(input.children();
    //             [kw_Read(node1), Expression(node2)] => ReadStat(node2, None)
    //         )
    //     )
    // }
    // fn returnstat(input: Node) -> Result<ReturnStat> {
    //     Ok(
    //         match_nodes!(input.children();
    //             [kw_return(node1), Expression(node2)] => ReturnStat(node2, None)
    //         )
    //     )
    // }
}

#[test]
fn test_syntax_vardecl_array() {
    let parsed = ParserCC20211A::parse(Rule::vardecl, "int zuleide[2][3]").unwrap();
    println!("{}", parsed);
    let ast = ParserCC20211A::vardecl(parsed.single().unwrap()).unwrap();
    println!("{:?}", ast)
}

#[test]
fn test_syntax_factor_float() {
    let parsed = ParserCC20211A::parse(Rule::factor, "1.66").unwrap();
    println!("{}", parsed);
    let ast = ParserCC20211A::factor(parsed.single().unwrap()).unwrap();
    println!("{:?}", ast)
}

#[test]
fn test_syntax_factor_array() {
    let parsed = ParserCC20211A::parse(Rule::factor, "inpp[22]").unwrap();
    println!("{}\n", parsed);
    let ast = ParserCC20211A::factor(parsed.single().unwrap()).unwrap();
    println!("{:?}", ast)
}

#[test]
fn test_syntax_expressions() {
    let parsed = ParserCC20211A::parse(Rule::expression, "2 + 3 + 5").unwrap();
    println!("{}\n", parsed);
    let ast = ParserCC20211A::expression(parsed.single().unwrap()).unwrap();
    println!("{:?}", ast)
}

