#[derive(Debug, Clone, Copy)]
pub enum TicketType {
    Single,
    Return,
    DayPass,
}

impl TicketType {
    pub fn to_string(&self) -> String {
        match self {
            TicketType::Single => "Single".to_string(),
            TicketType::Return => "Return".to_string(),
            TicketType::DayPass => "Day Pass".to_string(),
        }
    }
}