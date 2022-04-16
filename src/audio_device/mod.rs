mod audio_object;
mod property_address;

use audio_object::AudioObject;
use coreaudio_sys::{
    kAudioObjectSystemObject, kAudioObjectUnknown, noErr, AudioBuffer, AudioBufferList,
    AudioObjectID, AudioStreamID, OSStatus,
};
use property_address::{get_property_address, Property, Scope};
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::slice;

const NO_ERR: OSStatus = noErr as OSStatus;

pub enum Side {
    Input,
    Output,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Side::Input => "input",
                Side::Output => "output",
            }
        )
    }
}

impl From<&Side> for Scope {
    fn from(s: &Side) -> Self {
        match s {
            Side::Input => Scope::Input,
            Side::Output => Scope::Output,
        }
    }
}

pub struct SystemDevice(AudioObject);

impl SystemDevice {
    pub fn get_default_device(&self, s: &Side) -> Result<Device, OSStatus> {
        let address = get_property_address(
            match s {
                Side::Input => Property::DefaultInputDevice,
                Side::Output => Property::DefaultOutputDevice,
            },
            Scope::Global,
        );
        let mut device = kAudioObjectUnknown;
        let mut size = mem::size_of::<AudioObjectID>();
        let status = self.0.get_property_data(
            &address,
            0,
            ptr::null_mut::<c_void>(),
            &mut size,
            &mut device,
        );
        if status == NO_ERR {
            Ok(Device::new(device))
        } else {
            Err(status)
        }
    }

    pub fn get_all_devices(&self) -> Result<Vec<Device>, OSStatus> {
        let address = get_property_address(Property::Devices, Scope::Global);

        let mut size = 0;
        let status =
            self.0
                .get_property_data_size(&address, 0, ptr::null_mut::<c_void>(), &mut size);
        if status != NO_ERR {
            return Err(status);
        }

        let element_size = mem::size_of::<AudioObjectID>();
        assert_eq!(size % element_size, 0);
        let elements = size / element_size;
        let mut buffer = vec![kAudioObjectUnknown; elements];

        let status = self.0.get_property_data(
            &address,
            0,
            ptr::null_mut::<c_void>(),
            &mut size,
            buffer.as_mut_ptr(),
        );
        if status == NO_ERR {
            Ok(buffer.into_iter().map(Device::new).collect())
        } else {
            Err(status)
        }
    }
}

impl Default for SystemDevice {
    fn default() -> Self {
        Self(AudioObject::new(kAudioObjectSystemObject))
    }
}

pub struct Device(AudioObject);

impl Device {
    fn new(id: AudioObjectID) -> Self {
        Self(AudioObject::new(id))
    }

    pub fn id(&self) -> AudioObjectID {
        self.0.id()
    }

    pub fn is_valid(&self) -> bool {
        self.id() != kAudioObjectUnknown
    }

    pub fn in_scope(&self, s: &Side) -> Result<bool, OSStatus> {
        let streams = self.number_of_streams(s)?;
        Ok(streams > 0)
    }

    pub fn channel_count(&self, s: &Side) -> Result<u32, OSStatus> {
        let buffers = self.stream_configuration(s)?;
        let mut count = 0;
        for buffer in buffers {
            count += buffer.mNumberChannels;
        }
        Ok(count)
    }

    pub fn source(&self, s: &Side) -> Result<u32, OSStatus> {
        let address = get_property_address(Property::DeviceSource, Scope::from(s));
        let mut source = 0u32;
        let mut size = mem::size_of::<u32>();
        let status = self.0.get_property_data(
            &address,
            0,
            ptr::null_mut::<c_void>(),
            &mut size,
            &mut source,
        );
        if status == NO_ERR {
            Ok(source)
        } else {
            Err(status)
        }
    }

    pub fn transport_type(&self, s: &Side) -> Result<u32, OSStatus> {
        let address = get_property_address(Property::TransportType, Scope::from(s));
        let mut transport = 0u32;
        let mut size = mem::size_of::<u32>();
        let status = self.0.get_property_data(
            &address,
            0,
            ptr::null_mut::<c_void>(),
            &mut size,
            &mut transport,
        );
        if status == NO_ERR {
            Ok(transport)
        } else {
            Err(status)
        }
    }

    fn stream_configuration(&self, s: &Side) -> Result<Vec<AudioBuffer>, OSStatus> {
        let address = get_property_address(Property::DeviceStreamConfiguration, Scope::from(s));

        let mut size = 0;
        let status =
            self.0
                .get_property_data_size(&address, 0, ptr::null_mut::<c_void>(), &mut size);
        if status != NO_ERR {
            return Err(status);
        }

        let mut buffer = vec![0u8; size];
        let status = self.0.get_property_data(
            &address,
            0,
            ptr::null_mut::<c_void>(),
            &mut size,
            buffer.as_mut_ptr(),
        );
        if status == NO_ERR {
            let list = unsafe { &*(buffer.as_mut_ptr() as *mut AudioBufferList) };
            let s = unsafe {
                slice::from_raw_parts(
                    list.mBuffers.as_ptr() as *const AudioBuffer,
                    list.mNumberBuffers as usize,
                )
            };
            Ok(s.to_vec())
        } else {
            Err(status)
        }
    }

    fn number_of_streams(&self, s: &Side) -> Result<usize, OSStatus> {
        let address = get_property_address(Property::DeviceStreams, Scope::from(s));

        let mut size = 0;
        let status =
            self.0
                .get_property_data_size(&address, 0, ptr::null_mut::<c_void>(), &mut size);
        if status == NO_ERR {
            Ok(size / mem::size_of::<AudioStreamID>())
        } else {
            Err(status)
        }
    }
}

#[test]
fn test_default_devices() {
    check_device_is_in_scope(Scope::Input);
    check_device_is_in_scope(Scope::Output);

    fn check_device_is_in_scope(scope: Scope) {
        use coreaudio_sys::kAudioHardwareBadObjectError;
        let system_device = SystemDevice::default();
        match system_device.get_default_device(&scope) {
            Ok(device) => {
                if device.is_valid() {
                    assert!(device.in_scope(&scope).unwrap());
                } else {
                    assert_eq!(
                        device.in_scope(&scope).unwrap_err(),
                        kAudioHardwareBadObjectError as OSStatus
                    );
                }
            }
            Err(e) => {
                println!("Failed to get default {} device. Error: {}", scope, e);
            }
        }
    }
}

#[test]
fn test_device_list() {
    let system_device = SystemDevice::default();
    let devices = system_device.get_all_devices().unwrap();
    let input = system_device.get_default_device(&Scope::Input);
    let output = system_device.get_default_device(&Scope::Output);
    assert_eq!(
        devices.is_empty(),
        (input.is_err() || !input.unwrap().is_valid())
            && (output.is_err() || !output.unwrap().is_valid())
    );
}
