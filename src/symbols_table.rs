use std::collections::HashMap;

use crate::token_type::TokenType;
use crate::Rule;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Table};
use pest::iterators::Pairs;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum SymbolAttributes {
    StringConstantAttributes { value: String },
    IntConstantAttributes { value: usize },
    FloatConstantAttributes { value: f64 },
    IdentifierAttributes(),
}

#[derive(Clone)]
pub struct TokenPosition {
    pub token_stream_index: usize,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(TIdx: {}, Line: {}, Column: {})",
            self.token_stream_index, self.line, self.column
        )
    }
}

impl std::fmt::Debug for TokenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(TIdx: {:?}, Line: {:?}, Column: {:?})",
            self.token_stream_index, self.line, self.column
        )
    }
}

#[derive(Clone)]
pub struct SymbolTableEntry {
    pub lexeme: String,
    pub length: usize,
    pub r#type: TokenType,
    pub occurences: Vec<TokenPosition>,
    pub attributes: SymbolAttributes,
}

pub struct SymbolsTable {
    data: HashMap<String, SymbolTableEntry>,
}

impl SymbolsTable {
    pub fn new(raw_tokens: &mut Pairs<Rule>) -> Self {
        // Define Symbols Hash Table
        let mut hash_table = HashMap::<String, SymbolTableEntry>::new();
        // Filter SOI and EOI pairs
        let token_pairs = raw_tokens
            .next()
            .unwrap()
            .into_inner()
            .filter(|t_pair| t_pair.as_rule() != Rule::EOI);
        for (idx, token) in token_pairs.enumerate() {
            // Get Lexeme
            let lexeme = token.as_str();
            // Get Starting Position (Line/Column)
            let (start_line, start_col) = token.as_span().start_pos().line_col();
            // Identitfy Type
            let token_type_str = format!("{:?}", token.as_rule());
            let token_type =
                serde_plain::from_str::<TokenType>(&token_type_str).unwrap_or(TokenType::Ident);
            // Parse Attributes
            let attributes = match token.as_rule() {
                Rule::ident => SymbolAttributes::IdentifierAttributes(),
                Rule::int_constant => {
                    let constant_value = lexeme.parse::<usize>().unwrap();
                    SymbolAttributes::IntConstantAttributes {
                        value: constant_value,
                    }
                }
                Rule::float_constant => {
                    let constant_value = lexeme.parse::<f64>().unwrap();
                    SymbolAttributes::FloatConstantAttributes {
                        value: constant_value,
                    }
                }
                Rule::string_constant => {
                    let constant_value = &lexeme[1..(lexeme.len() - 1)];
                    SymbolAttributes::StringConstantAttributes {
                        value: constant_value.to_string(),
                    }
                }
                _ => continue,
            };

            // Add entry if not already exists
            if !hash_table.contains_key(lexeme) {
                hash_table.insert(
                    lexeme.to_string(),
                    SymbolTableEntry {
                        lexeme: lexeme.to_string(),
                        length: lexeme.len(),
                        r#type: token_type,
                        occurences: Vec::new(),
                        attributes: attributes,
                    },
                );
            }
            // Update occurrencies
            let symbol_entry = hash_table.get_mut(lexeme).unwrap();
            symbol_entry.occurences.push(TokenPosition {
                token_stream_index: idx,
                line: start_line,
                column: start_col,
            })
        }

        Self { data: hash_table }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn to_table(&self) -> Table {
        // Create Table
        let mut table = Table::new();
        // Define Table Options
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);
        // Define Headers
        table.set_header(vec![
            "Lexeme",
            "Length",
            "Type",
            "Occurences (Token Indexes)",
            "Attributes",
        ]);
        // Add Rows
        for st_entry in self.data.values() {
            table.add_row(vec![
                st_entry.lexeme.clone(),
                st_entry.length.to_string(),
                format!("{:?}", st_entry.r#type),
                format!("{:#?}", st_entry.occurences),
                format!("{:?}", st_entry.attributes)
            ]);
        }
        // Return Built Table
        return table;
    }
}
