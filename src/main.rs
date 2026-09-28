mod reactor;

use reactor::{process_command, safety::check_reactor_sensor, ReactorCommand, PowerLevel};

fn main() {
    let cmd1 = ReactorCommand::SetPower(PowerLevel::Critical);
    
    if let Some(level) = check_reactor_sensor(7) {
        println!("Sensor alert: {:?}", level);
        process_command(ReactorCommand::SetPower(level));
    }

    let cmd2 = ReactorCommand::Override {
        target_sector: String::from("Sector 7"),
        bypass_code: 4444,

    };
    process_command(cmd1);
    process_command(cmd2);

}
