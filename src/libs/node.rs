use std::option;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Clone)]
pub struct Node
{
    pub value: String,
    pub node: Vec<Node>
}

impl Node
{
    pub fn new() -> Node
    {
        Node
            {
                value: String::new(),
                node: Vec::new()
            }
    }
}