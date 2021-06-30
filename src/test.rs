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

    let pairs = ParserCC20211::parse(Rule::program, test_program).unwrap();
    println!("{}", pairs)
}