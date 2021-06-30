#[cfg(test)]
mod test;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ParserCC20211;

fn main() {
    println!("Hello, world!");
    let fields = ParserCC20211::parse(Rule::program, "10 + 5").unwrap();
    for field in fields {
        println!("{}", field.as_span().as_str());
    }
}