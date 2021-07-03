use comfy_table::Cell;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use pest::iterators::Pairs;

use crate::token_type::TokenType;
use crate::Rule;
struct Token(String, TokenType, usize, usize);

pub struct TokenList {
    data: Vec<Token>
}

impl TokenList {
    pub fn new(raw_tokens: &mut Pairs<Rule>) -> Self { 
        // Filter SOI and EOI pairs
        let token_pairs = raw_tokens.next().unwrap().into_inner().filter(|t_pair| t_pair.as_rule() != Rule::EOI);
        // Put Tokens on List
        let token_list = token_pairs.map(|t_pair| {
            let token_type = serde_plain::from_str::<TokenType>(&format!("{:?}", t_pair.as_rule())).unwrap_or(TokenType::Ident);
            Token(t_pair.as_str().to_string(), token_type, t_pair.as_span().start(), t_pair.as_span().end())
        }).collect::<Vec<Token>>();
        // Return TokenList Struct
        Self { 
            data: token_list
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn to_table(&self) -> Table {
        let headers = vec![
            "Lexeme",
            "Type",
            "Start",
            "End"
        ];

        let mut table = Table::new();
            table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
            .set_header(headers);

        for Token(lexeme, token_type, start, end) in &self.data {
            table.add_row(vec![
                Cell::new(lexeme),
                Cell::new(format!("{:?}", token_type)),
                Cell::new(start),
                Cell::new(end)
            ]);
        }

        return table;
    }
}