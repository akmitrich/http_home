#![allow(unused, dead_code)]

use std::fmt::format;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Device {
    Socket(Socket),
    Thermometer(Thermometer),
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Socket {
    voltage: f64,
    current: f64,
    on: bool,
}

#[derive(Debug, PartialEq)]
pub struct Thermometer {
    temperature: f64,
}

pub trait DeviceInfo {
    fn device_info(&self) -> Vec<String>;
}

impl Device {
    pub fn new_socket() -> Self {
        Device::Socket(Socket::new(220_f64, 0_f64, false))
    }

    pub fn new_thermometer() -> Self {
        Device::Thermometer(Thermometer::new(20_f64))
    }

    pub fn report(&self) -> String {
        String::from("Device...")
    }
}

impl DeviceInfo for Device {
    fn device_info(&self) -> Vec<String> {
        match self {
            Device::Socket(s) => s.device_info(),
            Device::Thermometer(t) => t.device_info(),
            _ => vec![String::from("Unknown device.")],
        }
    }
}

impl From<Socket> for Device {
    fn from(s: Socket) -> Self {
        Device::Socket(s)
    }
}

impl From<Thermometer> for Device {
    fn from(t: Thermometer) -> Self {
        Device::Thermometer(t)
    }
}

impl Socket {
    pub fn new(voltage: f64, current: f64, on: bool) -> Self {
        Self {
            voltage,
            current,
            on,
        }
    }

    pub fn get_voltage(&self) -> f64 {
        self.voltage
    }

    pub fn set_voltage(&mut self, voltage: f64) {
        self.voltage = voltage;
    }

    pub fn get_current(&self) -> f64 {
        self.current
    }

    pub fn set_current(&mut self, current: f64) {
        self.current = current;
    }

    pub fn get_current_power(&self) -> f64 {
        self.current * self.voltage
    }

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn switch(&mut self, on: bool) {
        self.on = on;
    }
}

impl DeviceInfo for Socket {
    fn device_info(&self) -> Vec<String> {
        let mut result = vec![];
        result.push("socket".into());
        result.push((if self.on { "on" } else { "off" }).into());
        result.push(format!("{}", self.current));
        result.push(format!("{}", self.voltage));
        result
    }
}

impl Thermometer {
    pub fn new(temperature: f64) -> Self {
        Self { temperature }
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.set_temperature(temperature);
        self
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
    }
}

impl DeviceInfo for Thermometer {
    fn device_info(&self) -> Vec<String> {
        let mut result = vec![];
        result.push("thermometer".into());
        result.push(format!("{}", self.temperature));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket() {
        let mut device = Device::new_socket();
        if let Device::Socket(mut socket) = device {
            assert_eq!(220_f64, socket.voltage);
            assert_eq!(0_f64, socket.current);
            assert!(!socket.is_on());
            assert_eq!(0_f64, socket.get_current_power());
            socket.set_voltage(225_f64);
            socket.set_current(3_f64);
            socket.switch(true);
            assert_eq!(225_f64, socket.voltage);
            assert_eq!(3_f64, socket.current);
            assert!(socket.is_on());
            assert!((socket.get_current_power() - 675_f64).abs() < 1e-6);
        } else {
            panic!("Device::new_socket gives unexpected result.");
        }
    }

    #[test]
    fn test_thermometer() {
        let device = Device::new_thermometer();
        if let Device::Thermometer(mut thermometer) = device {
            assert_eq!(20_f64, thermometer.temperature);
            let thermometer = thermometer.temperature(25_f64);
            assert_eq!(25_f64, thermometer.temperature);
        } else {
            panic!("Device::new_thermometer gives unexpected result.");
        }
    }
}
