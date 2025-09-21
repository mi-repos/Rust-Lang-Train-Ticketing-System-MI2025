use crate::models::{Station, TicketType, Zone};

pub struct FareCalculator<'a> {
    zones: &'a Vec<Zone>,
}

impl<'a> FareCalculator<'a> {
    pub fn new(zones: &'a Vec<Zone>) -> Self {
        Self { zones }
    }

    pub fn calculate_fare(&self, source: &Station, destination: &Station, ticket_type: &TicketType) -> f64 {
        let max_zone = source.zone.max(destination.zone);
        
        // Find the appropriate zone fare
        let zone = self.zones.iter()
            .find(|z| z.number == max_zone)
            .unwrap_or_else(|| {
                // Fallback to highest zone if not found
                self.zones.last().expect("No zones configured")
            });

        match ticket_type {
            TicketType::Single => zone.single_fare,
            TicketType::Return => zone.return_fare,
            TicketType::DayPass => zone.day_pass_fare,
        }
    }
}