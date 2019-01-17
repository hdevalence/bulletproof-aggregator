
#[macro_use]
extern crate serde_derive;

pub fn foo() -> u64 {
    12
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}
