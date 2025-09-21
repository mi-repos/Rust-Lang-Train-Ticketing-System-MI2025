#[derive(Debug, Clone)]
pub struct Zone {
    pub number: u32,
    pub single_fare: f64,
    pub return_fare: f64,
    pub day_pass_fare: f64,
}

impl Zone {
    pub fn new(number: u32, single_fare: f64, return_fare: f64, day_pass_fare: f64) -> Self {
        Self {
            number,
            single_fare,
            return_fare,
            day_pass_fare,
        }
    }
}