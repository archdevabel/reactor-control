use std::io::{self, Write};
use std::collections::HashMap;

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

mod reactor;
use reactor::{process_command, safety::check_reactor_sensor, safety::check_coolant_temp, safety::parse_reactor_id, ReactorCommand, PowerLevel};

mod diagnose;
use diagnose::{ReactorCore, CoolantPump, Diagnose, print_diagnosis, TelemetryBuffer, compare_logs};

#[tokio::main]
async fn main() {
    let mut reactor_id: Option<u32> = None;

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        let alerts = vec![
            "Sector 2 coolant pressure stabilizing...",
            "Sector 5 core temperature rising: 720C",
            "WARNING: Sector 7 anomaly detected!",
        ];

        for alert in alerts {
            sleep(Duration::from_secs(4)).await;
            let _ = tx.send(String::from(alert)).await;
        } 
    });

    loop {
        while let Ok(alert) = rx.try_recv() {
            println!("\n[TELEMETRY ALERT]: {alert}");

        }

        print!("reactor>");
        io::stdout().flush().expect("Fail to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read input");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");

        match command {
            "status" => {
                if let Some(id) = reactor_id {
                    println!("Reactor ID: {id}");
                } else {
                    println!("Reactor ID: not set");
                }

                let reactor = ReactorCore { core_id: 1, temp: 850 };
                let pump = CoolantPump { pump_id: 1, is_active: true };
                let telemetry_u32 = TelemetryBuffer { value: 42 };
                let telemetry_f64 = TelemetryBuffer { value: 3.14 };

                print_diagnosis(&reactor);
                print_diagnosis(&pump);
                print_diagnosis(&telemetry_u32);
                print_diagnosis(&telemetry_f64);

                process_command(ReactorCommand::StatusQuery);
            }
            "set-id" => {
                match parts.next() {
                    Some(id_str) => {
                        match parse_reactor_id(id_str) {
                            Ok(id) => {
                                reactor_id = Some(id);
                                println!("Successfully initialized Reactor ID: {id}");
                            }
                            Err(err_msg) => {
                                println!("Fail to set ID: {err_msg}");
                            }
                        }
                    }
                    None => {
                        println!("Usage: set-id <ID>");
                    }
                }
            }
            "check-temp" => {
                match parts.next() {
                    Some(temp_str) => {
                        match temp_str.parse::<u32>() {
                            Ok(temp) => {
                                check_coolant_temp(temp);
                            }
                            Err(_) => {
                                println!("Error: Please enter valid numerical temperature.");
                            }
                        }
                    }
                    None => {
                        println!("Usage: check-temp <TEMP>");
                    }
                }
            }
            "exit" => {
                println!("Shutting down reactor control interface...");
                break;
            }
            _ => {
                println!("Unknown Command: {command}");
            }

        }


    }
}

