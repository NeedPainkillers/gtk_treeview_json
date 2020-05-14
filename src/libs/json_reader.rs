use crate::libs::node::Node;
use std::io::BufReader;
use std::io::prelude::*;

pub struct JsonReader
{

}

impl JsonReader
{
    pub fn new() -> JsonReader
    {
        JsonReader {}
    }

    pub fn read_file(&self, path: &std::path::Path) -> Vec<Node>
    {
        let mut file = std::fs::File::open(path).expect(format!("{0}{1}{2}","Cannot open file at '", (*path).display(), "'").as_str());
        let mut reader = std::io::BufReader::new(file);

        let mut data = String::new();
        let mut res = reader.read_to_string(&mut data);

        let array: Vec<Node> = serde_json::from_str(data.as_str()).unwrap();
        return array;
    }
}
