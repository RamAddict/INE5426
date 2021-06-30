use pest::Parser;

use crate::ParserCC20211;
use crate::Rule;

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
fn test1() {
    let string = "
        {
            int j;
            int k;
            k = 10.5;
            j = 1e-70;
        }
    ";
    let pairs = ParserCC20211::parse(Rule::program, string).expect("Error ocurred");
    println!("{}", pairs);
}

#[test]
fn test_float_rule() {
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