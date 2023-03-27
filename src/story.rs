use serde::Deserialize;

// list of all nodes
#[derive(Debug, Deserialize)]
pub struct Story(Vec<Node>);

// text to display and options to pick
#[derive(Debug, Deserialize)]
struct Node {
    id: u32,
    text: String,
    options: DecisionOptions,
}

// list of decision options for a node
#[derive(Debug, Deserialize)]
struct DecisionOptions(Vec<DecisionOption>);

// id of the node this option should lead to and description for this option
#[derive(Debug, Deserialize)]
struct DecisionOption {
    target_id: u32,
    description: String,
}

impl Story {
    // create new story from JSON
    pub fn new(file: std::fs::File) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(std::io::BufReader::new(file))
    }
}
