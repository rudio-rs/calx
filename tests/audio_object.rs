extern crate calx;
use calx::audio_object::{AudioSystemObject, Scope};

#[test]
fn test_devices_list_with_default_devices() {
    let system_device = AudioSystemObject::default();
    let devices = system_device.get_all_devices().unwrap();
    let input = system_device.get_default_device(Scope::Input);
    let output = system_device.get_default_device(Scope::Output);
    assert_eq!(
        devices.is_empty(),
        (input.is_err() || !input.unwrap().is_valid())
            && (output.is_err() || !output.unwrap().is_valid())
    );
}

#[test]
fn test_default_devices_are_in_scope() {
    check_device_is_in_scope(Scope::Input);
    check_device_is_in_scope(Scope::Output);

    fn check_device_is_in_scope(scope: Scope) {
        let system_device = AudioSystemObject::default();
        match system_device.get_default_device(scope.clone()) {
            Ok(device) => {
                if device.is_valid() {
                    assert!(device.in_scope(scope).unwrap());
                } else {
                    // kAudioHardwareBadObjectError
                    assert_eq!(device.in_scope(scope).unwrap_err(), 560947818);
                }
            }
            Err(status) => {
                println!(
                    "Error {} when getting default device for {:?}.",
                    status, scope
                );
            }
        }
    }
}

#[test]
fn test_get_channel_count_of_default_devices() {
    get_channel_count_of_default_device(Scope::Input);
    get_channel_count_of_default_device(Scope::Output);

    fn get_channel_count_of_default_device(scope: Scope) {
        let system_device = AudioSystemObject::default();
        match system_device.get_default_device(scope.clone()) {
            Ok(device) => {
                if device.is_valid() {
                    assert!(device.get_channel_count(scope).unwrap() > 0);
                } else {
                    // kAudioHardwareBadObjectError
                    assert_eq!(device.get_channel_count(scope).unwrap_err(), 560947818);
                }
            }
            Err(status) => {
                println!(
                    "Error {} when getting default device for {:?}.",
                    status, scope
                );
            }
        }
    }
}
