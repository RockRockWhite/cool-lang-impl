use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, hash::Hash};

// declarations
// ======================

pub struct Foo {
    x: i32,
    y: i32,
}

// ======================

/// Token
/// must implement Clone and Token trait
/// Token trait is used to convert a token to a tree node
pub trait Token: Clone {
    fn to_tree_node(&self) -> ParsingTreeNode;
}

pub struct ParsingTreeNode {
    pub symbol_type: String,
    pub data: String,
    pub children: Vec<ParsingTreeNode>,
}

impl ParsingTreeNode {
    pub fn build(symbol_type: String, data: String, children: Vec<ParsingTreeNode>) -> Self {
        ParsingTreeNode {
            symbol_type,
            data,
            children,
        }
    }
}

/// NodePair
/// a pair of a node and a state.
/// (TreeNode, state)
pub struct NodePair(ParsingTreeNode, usize);
impl NodePair {
    pub fn new(node: ParsingTreeNode, state: usize) -> Self {
        NodePair(node, state)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReduceDerivation {
    pub left: String,
    pub right: Vec<String>,
}

impl PartialEq for ReduceDerivation {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl Eq for ReduceDerivation {}

impl Hash for ReduceDerivation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        self.right.hash(state);
    }
}

impl ReduceDerivation {
    pub fn build(left: String, right: Vec<String>) -> Self {
        ReduceDerivation { left, right }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Shift(usize),
    Reduce(ReduceDerivation),
    Accept,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub actions: HashMap<String, Action>,
}

impl State {
    pub fn new() -> Self {
        State {
            actions: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ActionTable {
    pub states: Vec<State>,
}

impl ActionTable {
    pub fn get_action(&self, state: usize, symbol: &str) -> Option<&Action> {
        self.states[state].actions.get(symbol)
    }
}

#[derive(Default)]
pub struct RParser {
    action_table: ActionTable,
    handlers: HashMap<ReduceDerivation, Box<dyn Fn(Vec<String>) -> String>>,
    // variables
    // ======================
    pub a: i32,
    pub b: i64,
    // ======================
}

impl RParser {
    pub const END_SYMBOL: &'static str = "__$__";
    pub const EPSILON_SYMBOL: &'static str = "__EPSILON__";
    pub const DUMMY_START_SYMBOL: &'static str = "__DUMMY_START__";

    pub fn new() -> Self {
        // action table generated by rparser
        // ======================
        let action_table: ActionTable =  serde_json::from_str(r#"{"states":[{"actions":{"(":{"Shift":2},"S":{"Shift":1},"E":{"Shift":11},"__$__":"Accept","T":{"Shift":6},"int":{"Shift":3}}},{"actions":{"__$__":{"Reduce":{"left":"__DUMMY_START__","right":["S"]}}}},{"actions":{"int":{"Shift":3},"T":{"Shift":6},"E":{"Shift":9},"(":{"Shift":2}}},{"actions":{")":{"Reduce":{"left":"T","right":["int"]}},"+":{"Reduce":{"left":"T","right":["int"]}},"*":{"Shift":4},"__$__":{"Reduce":{"left":"T","right":["int"]}}}},{"actions":{"(":{"Shift":2},"int":{"Shift":3},"T":{"Shift":5}}},{"actions":{"+":{"Reduce":{"left":"T","right":["int","*","T"]}},")":{"Reduce":{"left":"T","right":["int","*","T"]}},"__$__":{"Reduce":{"left":"T","right":["int","*","T"]}}}},{"actions":{"__$__":{"Reduce":{"left":"E","right":["T"]}},")":{"Reduce":{"left":"E","right":["T"]}},"+":{"Shift":7}}},{"actions":{"(":{"Shift":2},"int":{"Shift":3},"E":{"Shift":8},"T":{"Shift":6}}},{"actions":{"__$__":{"Reduce":{"left":"E","right":["T","+","E"]}},")":{"Reduce":{"left":"E","right":["T","+","E"]}}}},{"actions":{")":{"Shift":10}}},{"actions":{")":{"Reduce":{"left":"T","right":["(","E",")"]}},"__$__":{"Reduce":{"left":"T","right":["(","E",")"]}},"+":{"Reduce":{"left":"T","right":["(","E",")"]}}}},{"actions":{"__$__":{"Reduce":{"left":"S","right":["E"]}}}}]}"#).unwrap();
        // ======================

        let mut handlers: HashMap<ReduceDerivation, Box<dyn Fn(Vec<String>) -> String>> =
            HashMap::new();

        // handlers generated by rparser
        // ======================
        handlers.insert(
            ReduceDerivation::build("S".into(), vec!["E".into()]),
            Box::new(|datas| {
                println!("reduce: S -> E");
                datas[0].clone()
            }),
        );
        handlers.insert(
            ReduceDerivation::build("E".into(), vec!["T".into(), "+".into(), "E".into()]),
            Box::new(|datas| {
                println!("reduce: E -> T + E");
                let left = datas[0].parse::<i64>().unwrap();
                let right = datas[2].parse::<i64>().unwrap();
                (left + right).to_string()
            }),
        );
        handlers.insert(
            ReduceDerivation::build("E".into(), vec!["T".into()]),
            Box::new(|datas| {
                println!("reduce: E -> T");
                datas[0].clone()
            }),
        );
        handlers.insert(
            ReduceDerivation::build("T".into(), vec!["int".into(), "*".into(), "T".into()]),
            Box::new(|datas| {
                println!("reduce: T -> int * T");
                let left = datas[0].parse::<i64>().unwrap();
                let right = datas[2].parse::<i64>().unwrap();
                (left * right).to_string()
            }),
        );
        handlers.insert(
            ReduceDerivation::build("T".into(), vec!["int".into()]),
            Box::new(|datas| {
                println!("reduce: T -> int");
                datas[0].clone()
            }),
        );
        handlers.insert(
            ReduceDerivation::build("T".into(), vec!["(".into(), "E".into(), ")".into()]),
            Box::new(|datas| {
                println!("reduce: T -> int");
                datas[0].clone()
            }),
        );
        // ======================

        handlers.insert(
            ReduceDerivation::build(Self::DUMMY_START_SYMBOL.into(), vec!["S".into()]),
            Box::new(|vals| vals[0].clone()),
        );

        let mut res: RParser = RParser::default();
        res.action_table = action_table;
        res.handlers = handlers;
        res
    }

    // do shift-reduce parsing
    pub fn parse<T>(&self, tokens: Vec<T>) -> Result<ParsingTreeNode, Box<dyn Error>>
    where
        T: Token,
    {
        let mut shift_index = 0;
        let mut stack: Vec<NodePair> = Vec::new();

        stack.push(NodePair::new(
            ParsingTreeNode::build(Self::DUMMY_START_SYMBOL.into(), String::new(), Vec::new()),
            0,
        ));

        loop {
            let token_node = &tokens[shift_index].to_tree_node();

            let action = self
                .action_table
                .get_action(stack.last().unwrap().1, &token_node.symbol_type);

            match action {
                Some(Action::Shift(next_state)) => {
                    // shift
                    stack.push(NodePair::new(
                        tokens[shift_index].to_tree_node(),
                        *next_state,
                    ));
                    shift_index += 1;
                }
                Some(Action::Reduce(derivation)) => {
                    // pop right hand
                    let mut children: Vec<ParsingTreeNode> = Vec::new();
                    let mut datas = Vec::new();
                    for _ in 0..derivation.right.len() {
                        if let Some(top) = stack.pop() {
                            datas.push(top.0.data.clone());
                            children.push(top.0);
                        } else {
                            Err("parsing error: stack is empty.")?;
                        }
                    }

                    let children: Vec<_> = children.into_iter().rev().collect();
                    let datas: Vec<_> = datas.into_iter().rev().collect();
                    let handler = self.handlers.get(&derivation).unwrap();

                    // if the left hand side is dummy start symbol
                    // do nothing
                    if derivation.left == Self::DUMMY_START_SYMBOL {
                        stack.push(NodePair(
                            ParsingTreeNode::build(
                                derivation.left.to_string(),
                                handler(datas),
                                children,
                            ),
                            0,
                        ));
                        continue;
                    }

                    // goto[top_state(stack), X]
                    if let Action::Shift(next_state) = self
                        .action_table
                        .get_action(stack.last().unwrap().1, &derivation.left)
                        .unwrap()
                    {
                        stack.push(NodePair(
                            ParsingTreeNode::build(
                                derivation.left.to_string(),
                                handler(datas),
                                children,
                            ),
                            *next_state,
                        ));
                    } else {
                        Err("parsing error: invalid Shift action.")?;
                    }
                }
                Some(Action::Accept) => {
                    let res = stack.pop().unwrap().0;
                    return Ok(res);
                }
                _ => {
                    Err("parsing error: unknown.")?;
                }
            }
        }
    }
}
