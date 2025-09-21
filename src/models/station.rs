#[derive(Debug, Clone)]
pub struct Station {
    pub name: String,
    pub zone: u32,
}

impl Station {
    pub fn new(name: &str, zone: u32) -> Self {
        Self {
            name: name.to_string(),
            zone,
        }
    }
}