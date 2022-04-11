use calx::audio_device::{Device, Scope, SystemDevice};

fn main() {
    let system_device = SystemDevice::default();

    match system_device.get_default_device(&Scope::Input) {
        Ok(device) => println!("default input device: {}", device.id()),
        Err(e) => println!("Failed to get default input device. Error {}", e),
    }

    match system_device.get_default_device(&Scope::Output) {
        Ok(device) => println!("default output device: {}", device.id()),
        Err(e) => println!("Failed to get default output device. Error {}", e),
    }

    match system_device.get_all_devices() {
        Ok(devices) => {
            fn print_devices_in_scope(devices: &[Device], scope: Scope) {
                println!("{} devices:", scope);
                for device in devices.iter() {
                    if device.in_scope(&scope).unwrap_or(false) {
                        println!(
                            "\tid: {}\n\tchannel count: {}",
                            device.id(),
                            device.channel_count(&scope).unwrap_or(0)
                        );
                    }
                }
            }
            print_devices_in_scope(&devices, Scope::Input);
            print_devices_in_scope(&devices, Scope::Output);
        }
        Err(e) => println!("Failed to get all devices. Error {}", e),
    }
}
