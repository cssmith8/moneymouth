#[derive(serde::Serialize, serde::Deserialize)]
pub struct Simpledate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Simpledate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Simpledate { year, month, day }
    }
}
