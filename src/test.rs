// use crate::symbols_table::{SymbolAttributes, SymbolTableEntry, TokenPosition};
// use crate::token_type::TokenType;
use crate::ParserCC20211;
use crate::Rule;
use pest::Parser;
// use pest::iterators::Pair;
// use std::collections::{HashMap};
// use comfy_table::Table;
// use comfy_table::presets::UTF8_FULL;

#[test]
fn test_example1() {
    let test_program = "
    {
        {
          float x;
          float z;
          int i;
          int max;
          x = 0;
          max = 10000;
          for (i = 1; i <= max; i = i + 1){
            print x;
            x = x + 0.001;
            z = x;
            if (z != x){
              print \"Erro numérico na atribuição de números na notação ponto flutuante!\";
              break;
            }
          }
        }
        
        
        {
          int y;
          int j;
          int i;
          y = new int[10];
          j = 0;
          for (i = 0; i < 20; i = i + 1) 
            if (i % 2 == 0){
              y[j] = i + 1;
              j = j + 1;
            }
            else
              print 0;
        
          for (i = 0; i < 10; i = i + 1)
            print y[i];
        
          return;
        }
        }
    ";

    let pairs_result = ParserCC20211::parse(Rule::program, test_program);
    match pairs_result {
        Ok(pairs) => {
            println!("{}", pairs)
        }
        Err(error) => {
            println!("{}", error);
            println!("{:?}", error.variant);
            panic!()
        }
    }
}

#[test]
fn test_atribstat_rule() {
    let string = "k = 10.5";
    let pairs = ParserCC20211::parse(Rule::atribstat, string).expect("Error ocurred");
    println!("{}", pairs);
}

#[test]
fn test_string_rule() {
    let string = r#""hello, \nworld!""#;
    let pairs = ParserCC20211::parse(Rule::string_constant, string).expect("Error ocurred");
    println!("{}", pairs);
}

// #[test]
// fn test_lexical_analysis() {
//     let string = r#"
//         {
//             {
//                 float x;
//                 float z;
//                 int i;
//                 int max;
//                 x = 0;
//                 max = 10000;
//                 for (i = 1; i <= max; i = i + 1){
//                     print x;
//                     x = x + 0.001;
//                     z = x;
//                     if (z != x){
//                         print "Erro numérico na atribuição de números na notação ponto flutuante!";
//                         break;
//                     }
//                 }
//             }
        
        
//             {
//                 int y;
//                 int j;
//                 int i;
//                 y = new int[10];
//                 j = 0;
//                 for (i = 0; i < 20; i = i + 1) 
//                 if (i % 2 == 0){
//                     y[j] = i + 1;
//                     j = j + 1;
//                 }
//                 else
//                     print 0;
            
//                 for (i = 0; i < 10; i = i + 1)
//                 print y[i];
            
//                 return;
//             }
//         }
//     "#;
//     match ParserCC20211::parse(Rule::tokens, string) {
//         Ok(tokens_matchers) => {
//             let mut occurrence_id = 0;
//             let mut symbols_table = HashMap::<&str, SymbolTableEntry>::new();
//             let tm = tokens_matchers.into_iter().next().unwrap();
//             let a = tm.into_inner().filter(|pair| pair.as_rule() != Rule::EOI).collect::<Vec<Pair<Rule>>>();
//             println!("{:#?}", a);
//             // for token_matcher in tokens_matchers {
//             //     println!("{}", token_matcher.as_str().len());
//             //     for token in token_matcher.into_inner() {
//             //         occurrence_id += 1;
//             //         let token_rule = token.as_rule();

//             //         if token_rule == Rule::EOI {
//             //             continue;
//             //         }

//             //         let token_str = token.as_str();
//             //         let (token_start_line, token_start_column) =
//             //             token.as_span().start_pos().line_col();
//             //         let token_type = format!("{:?}", token.as_rule());
//             //         let token_type =
//             //             serde_plain::from_str::<TokenType>(token_type.as_str()).unwrap();
//             //         let attributes = match token_rule {
//             //             Rule::ident => SymbolAttributes::IdentifierAttributes(),
//             //             Rule::int_constant => {
//             //                 let constant_value = token_str.parse::<usize>().unwrap();
//             //                 SymbolAttributes::IntConstantAttributes {
//             //                     value: constant_value
//             //                 }
//             //             }
//             //             Rule::float_constant => {
//             //                 let constant_value = token_str.parse::<f64>().unwrap();
//             //                 SymbolAttributes::FloatConstantAttributes {
//             //                     value: constant_value
//             //                 }
//             //             }
//             //             Rule::string_constant => {
//             //                 let constant_value = &token_str[1..(token_str.len() - 1)];
//             //                 SymbolAttributes::StringConstantAttributes {
//             //                     value: constant_value.to_string()
//             //                 }
//             //             }
//             //             _ => continue,
//             //         };

//             //         if !symbols_table.contains_key(token_str) {
//             //             symbols_table.insert(token_str, SymbolTableEntry {
//             //                 lexeme: token_str.to_string(),
//             //                 length: token_str.len(),
//             //                 r#type: token_type,
//             //                 occurences: Vec::new(),
//             //                 attributes: attributes,
//             //             });
//             //         }

//             //         let symbol_entry = symbols_table.get_mut(token_str).unwrap();

//             //         // Update Occurrency
//             //         symbol_entry.occurences.push(TokenPosition {
//             //             token_stream_index: occurrence_id - 1,
//             //             line: token_start_line,
//             //             column: token_start_column,
//             //         });
//             //     }
//             // }
//             // let mut table = Table::new();
//             // let table = table
//             //     .load_preset(UTF8_FULL)
//             //     .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);
//             // table.set_header(vec!["Lexeme", "Length", "Type", "Occurences (Token Indexes)", "Attributes"]);
//             // for ht_row in symbols_table.values() {
//             //     table.add_row(vec![
//             //         ht_row.lexeme.clone(),
//             //         ht_row.length.to_string(),
//             //         format!("{:?}", ht_row.r#type),
//             //         format!("{:#?}", ht_row.occurences),
//             //         format!("{:?}", ht_row.attributes)
//             //     ]);
//             // };
//             // println!("{}", table);
//         }
//         Err(error) => {
//             println!("{}", error);
//             panic!("Error while processing")
//         }
//     }
// }
