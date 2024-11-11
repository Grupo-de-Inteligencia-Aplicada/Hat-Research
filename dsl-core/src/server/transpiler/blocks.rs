use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct XmlWorkspace {
    #[serde(rename = "block")]
    pub blocks: Vec<Block>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    #[serde(rename = "@type")]
    pub block_type: String,

    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@x")]
    pub x: Option<i32>,

    #[serde(rename = "@y")]
    pub y: Option<i32>,

    #[serde(rename = "field", default = "Vec::new")]
    pub fields: Vec<Field>,

    #[serde(rename = "value", default = "Vec::new")]
    pub values: Vec<Value>,

    #[serde(rename = "statement", default = "Vec::new")]
    pub statements: Vec<Statement>,

    pub next: Option<Box<Next>>,
}

impl Block {
    pub fn get_field(&self, name: &str) -> Option<&String> {
        self.fields
            .iter()
            .filter(|field| field.name == name)
            .next()
            .map(|field| &field.text)
    }
    pub fn get_value(&self, name: &str) -> Option<&Value> {
        self.values.iter().filter(|val| val.name == name).next()
    }
    pub fn get_statement(&self, name: &str) -> Option<&Statement> {
        self.statements
            .iter()
            .filter(|stmt| stmt.name == name)
            .next()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Value {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "block")]
    pub inner_block: Block,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Statement {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "block")]
    pub inner_block: Block,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Next {
    #[serde(rename = "block")]
    pub inner_block: Block,
}
