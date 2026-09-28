use std::collections::HashMap;

mod reactor;

use reactor::{process_command, safety::check_reactor_sensor, ReactorCommand, PowerLevel};

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

}
