use crate::smart_device::Device;
use std::collections::{hash_map::Entry, HashMap};

#[allow(dead_code, unused)]
#[derive(Debug, PartialEq)]
pub struct Room {
    devices: HashMap<String, Device>,
}

#[allow(dead_code, unused)]
impl Room {
    pub fn new() -> Self {
        Room {
            devices: HashMap::new(),
        }
    }

    pub fn device_names_list(&self) -> impl Iterator<Item = &String> {
        self.devices.keys()
    }

    pub fn device_list(&self) -> impl Iterator<Item = &Device> {
        self.devices.values()
    }

    pub fn add_device(&mut self, unique_name: &str, device: Device) -> Option<&Device> {
        match self.devices.entry(unique_name.into()) {
            Entry::Occupied(_) => None,
            Entry::Vacant(entry) => Some(entry.insert(device)),
        }
    }

    pub fn remove_device(&mut self, device_name: &str) -> Option<Device> {
        self.devices.remove(device_name)
    }

    pub fn get_device_by_name(&self, device_name: &str) -> Option<&Device> {
        self.devices.get(device_name)
    }

    pub fn get_device_by_name_mut(&mut self, device_name: &str) -> Option<&mut Device> {
        self.devices.get_mut(device_name)
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room() {
        let room = Room::new();
        assert!(room.devices.is_empty());
    }

    #[test]
    fn test_add_get_device() {
        let mut room = Room::new();
        assert!(room.add_device("S1", Device::new_socket()).is_some());
        assert!(room.add_device("S2", Device::new_socket()).is_some());
        assert!(room.add_device("T", Device::new_thermometer()).is_some());
        assert!(room.add_device("S1", Device::new_thermometer()).is_none());
        assert_eq!(3, room.device_names_list().count());
        assert_eq!(3, room.device_list().count());
        assert_eq!(
            &Device::new_socket(),
            room.get_device_by_name("S1").unwrap()
        );
        assert_eq!(
            &Device::new_socket(),
            room.get_device_by_name("S2").unwrap()
        );
        assert_eq!(
            &Device::new_thermometer(),
            room.get_device_by_name("T").unwrap()
        );
        assert!(room.get_device_by_name("No device").is_none());
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room::new();
        assert!(room.add_device("S1", Device::new_socket()).is_some());
        assert!(room.add_device("S2", Device::new_socket()).is_some());
        assert!(room.add_device("T", Device::new_thermometer()).is_some());
        assert_eq!(3, room.device_names_list().count());
        assert_eq!(3, room.device_list().count());
        assert!(room.remove_device("S1").is_some());
        assert!(room.remove_device("S2").is_some());
        assert!(room.remove_device("T").is_some());
        assert!(room.remove_device("No device").is_none());
        assert!(room.device_names_list().next().is_none());
        assert!(room.device_list().next().is_none())
    }
}
