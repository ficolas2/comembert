use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
pub struct Metadata {
    pub selectors: Vec<Selector>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Selector {
    #[serde(rename = "type")]
    pub selector_type: SelectorType,
    pub slots: Vec<String>,
    pub fzf_string: Option<String>,

    // List
    #[serde(default)]

    pub list: Vec<String>,
    // Command
    pub delimiter: Option<String>,
    pub command: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SelectorType {
    Command,
    List,
}
