use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct StreamDataType {
    pub e: String, // Event type
    pub E: i64,    // Event time
    pub T: i64,    // Transaction time
    pub s: String, // Symbol
    pub U: i64,
    pub u: i64,
    pub pu: i64,
    pub b: Vec<Vec<String>>,
    pub a: Vec<Vec<String>>,
}
