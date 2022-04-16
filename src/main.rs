use calx::audio_device::{Device, Side, SystemDevice};

fn main() {
    let system_device = SystemDevice::default();

    match system_device.get_default_device(&Side::Input) {
        Ok(device) => println!("default input device: {}", device.id()),
        Err(e) => println!("Failed to get default input device. Error {}", e),
    }

    match system_device.get_default_device(&Side::Output) {
        Ok(device) => println!("default output device: {}", device.id()),
        Err(e) => println!("Failed to get default output device. Error {}", e),
    }

    match system_device.get_all_devices() {
        Ok(devices) => {
            fn print_devices_in_scope(devices: &[Device], s: Side) {
                println!("{} devices:", s);
                for device in devices.iter() {
                    if device.in_scope(&s).unwrap_or(false) {
                        println!(
                            "\tid: {}\n\tchannel count: {}\n\ttransport type: {}",
                            device.id(),
                            device
                                .channel_count(&s)
                                .map_or_else(|e| format!("Error: {}", e), |c| c.to_string()),
                            device
                                .transport_type(&s)
                                .map_or_else(|e| format!("Error: {}", e), |t| t.to_string()),
                        );
                    }
                }
            }
            print_devices_in_scope(&devices, Side::Input);
            print_devices_in_scope(&devices, Side::Output);
        }
        Err(e) => println!("Failed to get all devices. Error {}", e),
    }
}
