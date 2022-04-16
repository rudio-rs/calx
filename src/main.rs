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
                println!("{} devices\n--------------", s);
                for device in devices.iter() {
                    if device.in_scope(&s).unwrap_or(false) {
                        println!(
                            "id: {}\n\
                            \tbuffer frame size range: {}\n\
                            \tchannel count: {}\n\
                            \tclock domain: {}\n\
                            \tlatency: {}\n\
                            \tmodel uid:\n\
                                \t\t{} - {}\n\
                                \t\tglobal - {}\n\
                            \tname:\n\
                                \t\t{} - {}\n\
                                \t\tglobal - {}\n\
                            \tsample rate: {}\n\
                            \tsample rate ranges: {}\n\
                            \tsource: {}\n\
                            \tsource name: {}\n\
                            \ttransport type: {}\n\
                            \tuid:\n\
                                \t\t{} - {}\n\
                                \t\tglobal - {}",
                            device.id(),
                            device.buffer_frame_size_range(&s).map_or_else(
                                |e| format!("Error: {}", e),
                                |(min, max)| format!("[{}, {}]", min, max)
                            ),
                            device
                                .channel_count(&s)
                                .map_or_else(|e| format!("Error: {}", e), |c| c.to_string()),
                            device
                                .clock_domain(&s)
                                .map_or_else(|e| format!("Error: {}", e), |d| d.to_string()),
                            device
                                .latency(&s)
                                .map_or_else(|e| format!("Error: {}", e), |l| l.to_string()),
                            s,
                            device
                                .model_uid(Some(&s))
                                .map_or_else(|e| format!("Error: {}", e), |u| u),
                            device
                                .model_uid(None)
                                .map_or_else(|e| format!("Error: {}", e), |u| u),
                            s,
                            device
                                .name(Some(&s))
                                .map_or_else(|e| format!("Error: {}", e), |u| u),
                            device
                                .name(None)
                                .map_or_else(|e| format!("Error: {}", e), |u| u),
                            device
                                .sample_rate(&s)
                                .map_or_else(|e| format!("Error: {}", e), |r| r.to_string()),
                            device.sample_rate_ranges(&s).map_or_else(
                                |e| format!("Error: {}", e),
                                |ranges| format!("{:?}", ranges)
                            ),
                            device
                                .source(&s)
                                .map_or_else(|e| format!("Error: {}", e), u32_to_string),
                            device
                                .source_name(&s)
                                .map_or_else(|e| format!("Error: {}", e), |n| n),
                            device
                                .transport_type(&s)
                                .map_or_else(|e| format!("Error: {}", e), |t| t.to_string()),
                            s,
                            device
                                .uid(Some(&s))
                                .map_or_else(|e| format!("Error: {}", e), |u| u),
                            device
                                .uid(None)
                                .map_or_else(|e| format!("Error: {}", e), |u| u),
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

fn u32_to_string(data: u32) -> String {
    // Reverse 0xWXYZ into 0xZYXW.
    let mut buffer = [b'\x00'; 4]; // 4 bytes for u32.
    buffer[0] = (data >> 24) as u8;
    buffer[1] = (data >> 16) as u8;
    buffer[2] = (data >> 8) as u8;
    buffer[3] = (data) as u8;
    String::from_utf8_lossy(&buffer).to_string()
}
