use crate::models::{Station, TicketType};
use std::collections::HashMap;
use std::io;

pub struct InputHandler;

impl InputHandler {
    pub fn get_station_input(prompt: &str, stations: &HashMap<String, Station>) -> Station {
        loop {
            println!("{}", prompt);
            println!("Available stations:");
            
            // Display stations in a more organized way
            let mut station_keys: Vec<&String> = stations.keys().collect();
            station_keys.sort();
            
            for key in station_keys {
                if let Some(station) = stations.get(key) {
                    println!("  {} - {} (Zone {})", key, station.name, station.zone);
                }
            }
            
            print!("Enter station code: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.trim().to_uppercase();
            
            if input.is_empty() {
                println!("Please enter a station code.\n");
                continue;
            }
            
            if let Some(station) = stations.get(&input) {
                return station.clone();
            }
            
            println!("Invalid station code '{}'. Please try again.\n", input);
        }
    }

    pub fn get_ticket_type() -> TicketType {
        loop {
            println!("\nSelect ticket type:");
            println!("1. Single");
            println!("2. Return");
            println!("3. Day Pass");
            
            print!("Enter your choice (1-3): ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            
            match input.trim() {
                "1" => return TicketType::Single,
                "2" => return TicketType::Return,
                "3" => return TicketType::DayPass,
                _ => println!("Invalid choice '{}'. Please enter 1, 2, or 3.", input.trim()),
            }
        }
    }
}