use std::io;
use std::collections::HashMap;

mod reactor;

use reactor::{process_command, safety::check_reactor_sensor, safety::check_coolant_temp, safety::parse_reactor_id, ReactorCommand, PowerLevel};

fn main() {
    let sectors = vec![2, 5, 7, 9];
    let mut sector_status: HashMap<u32, PowerLevel> = HashMap::new();
        
    for sector in &sectors {
        println!("Checking Sector {sector}...");
        if let Some(level) = check_reactor_sensor(*sector) {
            println!("Sensor alert: {:?}", level);
            process_command(ReactorCommand::SetPower(level.clone()));

            sector_status.insert(*sector, level);
            for (s_id, s_lvl) in &sector_status {
                println!("{s_id}: {:?}", s_lvl);
            }
        }
 
    }
    
    let sector_name = String::from("Sector");
    let sector_id = String::from("7");
    let sector_full_name = format!("{sector_name} {sector_id}");

    let cmd1 = ReactorCommand::SetPower(PowerLevel::Critical);
    
        let cmd2 = ReactorCommand::Override {
        target_sector: sector_full_name,
        bypass_code: 4444,

    };
    process_command(cmd1);
    process_command(cmd2);

    let mut reactor_id = String::new();
    println!("Input the reactor ID: ");
   
    io::stdin()
        .read_line(&mut reactor_id)
        .expect("Fail to read reactor ID!");

    match parse_reactor_id(&reactor_id) {
        Ok(id) => println!("Successfully initialized Reactor ID: {id}"),
        Err(err_msg) => println!("Failed to set ID: {err_msg}"),
    } 

    let mut temp_input = String::new();
    println!("Input current coolant temperature: ");

    io::stdin()
        .read_line(&mut temp_input)
        .expect("Fail to read temperature!");

    let temp: u32 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Please enter a valid numerical temperature.");
            return;
        } 
    };

    check_coolant_temp(temp);

}
