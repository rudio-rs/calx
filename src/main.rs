use calx::audio_object::{AudioSystemObject, Scope};

fn main() {
    let system_device = AudioSystemObject::default();
    let input = system_device.get_default_device(Scope::Input);
    let output = system_device.get_default_device(Scope::Output);
    println!("default input: {:?}, default output: {:?}", input, output);

    let devices = system_device.get_all_devices();
    println!("devices: {:?}", devices);
    if let Ok(devices) = system_device.get_all_devices() {
        let mut input_devices = Vec::new();
        let mut output_devices = Vec::new();
        for device in devices.iter() {
            if device.in_scope(Scope::Input).unwrap_or(false) {
                input_devices.push(device);
            }
            if device.in_scope(Scope::Output).unwrap_or(false) {
                output_devices.push(device);
            }
        }
        if !input_devices.is_empty() {
            println!("Input Devices:");
            for device in input_devices {
                println!(
                    "\t{:?}\n\t\tchannel count: {:?}",
                    device,
                    device.get_channel_count(Scope::Input)
                );
            }
        }
        if !output_devices.is_empty() {
            println!("Output Devices:");
            for device in output_devices {
                println!(
                    "\t{:?}\n\t\tchannel count: {:?}",
                    device,
                    device.get_channel_count(Scope::Output)
                );
            }
        }
    }
}
