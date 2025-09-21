mod models;
mod services;
mod utils;

use models::{Station, Zone};
use services::{FareCalculator, TicketPrinter};
use utils::InputHandler;
use std::collections::HashMap;

fn main() {
    println!("🚇 Welcome to the Train Ticketing System!");
    println!("==========================================\n");

    // Initialize stations and zones
    let stations = initialize_stations();
    let zones = initialize_zones();

    // Get user input
    let source = InputHandler::get_station_input("Enter source station: ", &stations);
    let destination = InputHandler::get_station_input("Enter destination station: ", &stations);
    let ticket_type = InputHandler::get_ticket_type();

    // Calculate fare
    let fare_calculator = FareCalculator::new(&zones);
    let fare = fare_calculator.calculate_fare(&source, &destination, &ticket_type);

    // Print ticket
    let ticket_printer = TicketPrinter;
    ticket_printer.print_ticket(&source, &destination, &ticket_type, fare);

    println!("\nThank you for using our ticketing system! 🎫");
}

fn initialize_stations() -> HashMap<String, Station> {
    let mut stations = HashMap::new();
    
    stations.insert("KINGS_CROSS".to_string(), Station::new("King's Cross St. Pancras", 1));
    stations.insert("VICTORIA".to_string(), Station::new("Victoria", 1));
    stations.insert("WATERLOO".to_string(), Station::new("Waterloo", 1));
    stations.insert("LIVERPOOL_ST".to_string(), Station::new("Liverpool Street", 1));
    
    stations.insert("CAMDEN_TOWN".to_string(), Station::new("Camden Town", 2));
    stations.insert("CLAPHAM_JUNC".to_string(), Station::new("Clapham Junction", 2));
    stations.insert("BRIXTON".to_string(), Station::new("Brixton", 2));
    
    stations.insert("WIMBLEDON".to_string(), Station::new("Wimbledon", 3));
    stations.insert("EALING_BROAD".to_string(), Station::new("Ealing Broadway", 3));
    
    stations.insert("HEATHROW".to_string(), Station::new("Heathrow Airport", 6));
    stations.insert("RICHMOND".to_string(), Station::new("Richmond", 4));

    stations
}

fn initialize_zones() -> Vec<Zone> {
    vec![
        Zone::new(1, 2.50, 4.00, 12.00),
        Zone::new(2, 2.00, 3.50, 10.00),
        Zone::new(3, 1.80, 3.00, 8.50),
        Zone::new(4, 1.60, 2.80, 7.50),
        Zone::new(5, 1.50, 2.50, 7.00),
        Zone::new(6, 1.40, 2.20, 6.50),
    ]
}