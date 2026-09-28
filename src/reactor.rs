#[derive(Debug)]
pub enum PowerLevel {
    Low,
    Medium,
    Critical,
}

#[derive(Debug)]
pub enum ReactorCommand {
    Shutdown,
    SetPower(PowerLevel),
    Override {target_sector: String, bypass_code: u32},
    StatusQuery,
}

pub mod safety {
    use super::PowerLevel;

    pub fn check_reactor_sensor(sector_code: u32) -> Option<PowerLevel> {
        if sector_code == 7 {
            Some(PowerLevel::Critical)
        } else {
            None
        }
    }
}

pub fn process_command(cmd: ReactorCommand) {
    match cmd {
        ReactorCommand::Shutdown => {
            println!("WARNING: Initializing emergency shutdown sequence!");
        }
        ReactorCommand::SetPower(level) => {
            println!("Adjusting reactor power level to: {:?}", level);
        }
        ReactorCommand::Override {target_sector, bypass_code} => {
            println!("SECURITY OVERRIDE! Sector: {}, Code: {}", target_sector, bypass_code);
        }
        ReactorCommand::StatusQuery => {
            println!("Running full reactor diagnostics...");
        }

    }
}


