use crate::models::{Station, TicketType};

pub struct TicketPrinter;

impl TicketPrinter {
    pub fn print_ticket(&self, source: &Station, destination: &Station, ticket_type: &TicketType, fare: f64) {
        println!("\n");
        println!("╔══════════════════════════════════════════╗");
        println!("║           TRAIN TICKET                  ║");
        println!("╠══════════════════════════════════════════╣");
        println!("║ From: {:35} ║", Self::truncate(&source.name, 35));
        println!("║ To:   {:35} ║", Self::truncate(&destination.name, 35));
        println!("║ Type: {:35} ║", Self::truncate(&ticket_type.to_string(), 35));
        println!("║ Zone: {:35} ║", format!("{} → {}", source.zone, destination.zone));
        println!("║ Fare: £{:<34.2} ║", fare);
        println!("╠══════════════════════════════════════════╣");
        println!("║          Thank you for traveling!       ║");
        println!("╚══════════════════════════════════════════╝");
    }

    // Helper function to truncate long strings
    fn truncate(s: &str, max_length: usize) -> String {
        if s.len() > max_length {
            format!("{}...", &s[..max_length - 3])
        } else {
            s.to_string()
        }
    }
}