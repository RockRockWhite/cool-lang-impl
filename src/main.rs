use rlex_gen::rparser::{RParser, Token};

#[derive(Clone)]
struct MyToken {
    symbol_type: String,
    data: String,
}

impl MyToken {
    pub fn build(symbol_type: String, data: String) -> Self {
        MyToken { symbol_type, data }
    }
}

impl Token for MyToken {
    fn to_tree_node(&self) -> rlex_gen::rparser::ParsingTreeNode {
        rlex_gen::rparser::ParsingTreeNode::build(
            self.symbol_type.clone(),
            self.data.clone(),
            vec![],
        )
    }
}

fn main() {
    println!("Hello, world!");

    let parser = RParser::new();
    let input = Vec::from([
        MyToken::build("int".into(), "123".into()),
        MyToken::build("*".into(), "*".into()),
        MyToken::build("int".into(), "234".into()),
        MyToken::build(RParser::END_SYMBOL.into(), "".into()),
    ]);
    let res = parser.parse(input).unwrap();

    println!("{}", res.data);
}
