pub trait Diagnose {
    fn status_report(&self) -> String;
}

pub struct TelemetryBuffer<T> {
    pub value: T,
}

pub struct ReactorCore {
    pub core_id: u32,
    pub temp: u32,
}

pub struct CoolantPump {
    pub pump_id: u32,
    pub is_active: bool,
}

impl Diagnose for ReactorCore {
    fn status_report(&self) -> String {
        format!("Core #{}: Temp is {}C", self.core_id, self.temp)
    }
}

impl Diagnose for CoolantPump {
    fn status_report(&self) -> String {
        format!("Pump #{}: Active Status = {}", self.pump_id, self.is_active)
    }
}

impl<T: std::fmt::Display> Diagnose for TelemetryBuffer<T> {
    fn status_report(&self) -> String {
        format!("Telemetry: Value is {}", self.value)
    }
}

#[allow(dead_code)]
pub fn compare_logs<'a>(log1: &'a str, log2: &'a str) -> &'a str {
    if log1.len() >= log2.len() { log1 } else { log2 }
}

pub fn print_diagnosis<T: Diagnose>(item: &T) {
    println!("[DIAGNOSTIC LOG]: {}", item.status_report());
}
