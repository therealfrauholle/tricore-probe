use serde::Serialize;
use tricore_windows::DeviceSelection;


pub fn pretty_print_devices(devices: &Vec<DeviceSelection>) {
    if devices.len() == 0 {
        println!("No devices available");
        return;
    }
    println!("Found {} devices:", devices.len());
    for (index, scanned_device) in devices.iter().enumerate() {
        println!("Device {index}: {:?}", scanned_device.info.acc_hw())
    }
}

#[derive(Serialize)]
struct Chip<'a> {
    id: &'a str
}

pub fn machine_output(devices: &Vec<DeviceSelection>) {
    let dev: Vec<_> = devices.iter().map(|dev| Chip { id: dev.info.acc_hw()}).collect();
    let json = serde_json::to_string(&dev).unwrap();
    println!("{json}");
}