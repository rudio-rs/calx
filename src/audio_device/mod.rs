mod audio_object;
mod property_address;

use super::string::StringRef;
use audio_object::AudioObject;
use core_foundation_sys::string::CFStringRef;
use coreaudio_sys::{
    kAudioObjectSystemObject, kAudioObjectUnknown, noErr, AudioBuffer, AudioBufferList,
    AudioObjectID, AudioStreamID, AudioValueRange, AudioValueTranslation, OSStatus,
};
use property_address::{get_property_address, Property, Scope};
use std::fmt;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::slice;

const NO_ERR: OSStatus = noErr as OSStatus;

pub enum Side {
    Input,
    Output,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut device);
        if status == NO_ERR {
            Ok(Device::new(device))
        } else {
            Err(status)
        }
    }

    pub fn get_all_devices(&self) -> Result<Vec<Device>, OSStatus> {
        let address = get_property_address(Property::Devices, Scope::Global);

        let mut size = 0;
        let status = self
            .0
            .get_property_data_size_without_qualifier(&address, &mut size);
        if status != NO_ERR {
            return Err(status);
        }

        let element_size = mem::size_of::<AudioObjectID>();
        assert_eq!(size % element_size, 0);
        let elements = size / element_size;
        let mut buffer = vec![kAudioObjectUnknown; elements];

        let status =
            self.0
                .get_property_data_without_qualifier(&address, &mut size, buffer.as_mut_ptr());
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

#[derive(PartialEq)]
pub struct DeviceId(AudioObjectID);

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Device(AudioObject);

impl Device {
    fn new(id: AudioObjectID) -> Self {
        Self(AudioObject::new(id))
    }

    pub fn id(&self) -> DeviceId {
        DeviceId(self.0.id())
    }

    pub fn is_valid(&self) -> bool {
        self.id() != DeviceId(kAudioObjectUnknown)
    }

    pub fn in_scope(&self, s: &Side) -> Result<bool, OSStatus> {
        let streams = self.streams(s)?;
        Ok(!streams.is_empty())
    }

    pub fn buffer_frame_size_range(&self, s: &Side) -> Result<(f64, f64), OSStatus> {
        let address = get_property_address(Property::DeviceBufferFrameSizeRange, Scope::from(s));
        let mut range = AudioValueRange::default();
        let mut size = mem::size_of::<AudioValueRange>();
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut range);
        if status == NO_ERR {
            Ok((range.mMinimum, range.mMaximum))
        } else {
            Err(status)
        }
    }

    pub fn channel_count(&self, s: &Side) -> Result<u32, OSStatus> {
        let buffers = self.stream_configuration(s)?;
        let mut count = 0;
        for buffer in buffers {
            count += buffer.mNumberChannels;
        }
        Ok(count)
    }

    pub fn clock_domain(&self, s: &Side) -> Result<u32, OSStatus> {
        let address = get_property_address(Property::ClockDomain, Scope::from(s));
        let mut domain = 0u32;
        let mut size = mem::size_of::<u32>();
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut domain);
        if status == NO_ERR {
            Ok(domain)
        } else {
            Err(status)
        }
    }

    pub fn latency(&self, s: &Side) -> Result<u32, OSStatus> {
        let address = get_property_address(Property::DeviceLatency, Scope::from(s));
        let mut latency = 0u32;
        let mut size = mem::size_of::<u32>();
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut latency);
        if status == NO_ERR {
            Ok(latency)
        } else {
            Err(status)
        }
    }

    pub fn model_uid(&self, s: Option<&Side>) -> Result<String, OSStatus> {
        let address = get_property_address(
            Property::DeviceModelUID,
            if let Some(side) = s {
                Scope::from(side)
            } else {
                Scope::Global
            },
        );

        let mut size = mem::size_of::<CFStringRef>();
        let mut uid: CFStringRef = ptr::null();
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut uid);
        if status == NO_ERR {
            let s = StringRef::new(uid);
            let utf8 = s.to_utf8();
            Ok(String::from_utf8_lossy(&utf8).to_string())
        } else {
            Err(status)
        }
    }

    pub fn name(&self, s: Option<&Side>) -> Result<String, OSStatus> {
        let address = get_property_address(
            Property::DeviceName,
            if let Some(side) = s {
                Scope::from(side)
            } else {
                Scope::Global
            },
        );

        let mut size = mem::size_of::<CFStringRef>();
        let mut uid: CFStringRef = ptr::null();
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut uid);
        if status == NO_ERR {
            let s = StringRef::new(uid);
            let utf8 = s.to_utf8();
            Ok(String::from_utf8_lossy(&utf8).to_string())
        } else {
            Err(status)
        }
    }

    pub fn sample_rate(&self, s: &Side) -> Result<f64, OSStatus> {
        let address = get_property_address(Property::DeviceSampleRate, Scope::from(s));
        let mut rate = 0f64;
        let mut size = mem::size_of::<f64>();
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut rate);
        if status == NO_ERR {
            Ok(rate)
        } else {
            Err(status)
        }
    }

    pub fn sample_rate_ranges(&self, s: &Side) -> Result<Vec<(f64, f64)>, OSStatus> {
        let address = get_property_address(Property::DeviceSampleRates, Scope::from(s));

        let mut size = 0;
        let status = self
            .0
            .get_property_data_size_without_qualifier(&address, &mut size);
        if status != NO_ERR {
            return Err(status);
        }

        let element_size = mem::size_of::<AudioValueRange>();
        assert_eq!(size % element_size, 0);
        let elements = size / element_size;
        let mut buffer = vec![AudioValueRange::default(); elements];

        let status =
            self.0
                .get_property_data_without_qualifier(&address, &mut size, buffer.as_mut_ptr());
        if status == NO_ERR {
            Ok(buffer
                .into_iter()
                .map(|r| (r.mMinimum, r.mMaximum))
                .collect())
        } else {
            Err(status)
        }
    }

    pub fn source(&self, s: &Side) -> Result<u32, OSStatus> {
        let address = get_property_address(Property::DeviceSource, Scope::from(s));
        let mut source = 0u32;
        let mut size = mem::size_of::<u32>();
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut source);
        if status == NO_ERR {
            Ok(source)
        } else {
            Err(status)
        }
    }

    pub fn source_name(&self, s: &Side) -> Result<String, OSStatus> {
        let mut source = self.source(s)?;
        let address = get_property_address(Property::DeviceSourceName, Scope::from(s));
        let mut size = mem::size_of::<AudioValueTranslation>();
        let mut name: CFStringRef = ptr::null();
        let mut trl = AudioValueTranslation {
            mInputData: &mut source as *mut u32 as *mut c_void,
            mInputDataSize: mem::size_of::<u32>() as u32,
            mOutputData: &mut name as *mut CFStringRef as *mut c_void,
            mOutputDataSize: mem::size_of::<CFStringRef>() as u32,
        };
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut trl);
        if status == NO_ERR {
            let s = StringRef::new(name);
            let utf8 = s.to_utf8();
            Ok(String::from_utf8_lossy(&utf8).to_string())
        } else {
            Err(status)
        }
    }

    pub fn transport_type(&self, s: &Side) -> Result<u32, OSStatus> {
        let address = get_property_address(Property::TransportType, Scope::from(s));
        let mut transport = 0u32;
        let mut size = mem::size_of::<u32>();
        let status =
            self.0
                .get_property_data_without_qualifier(&address, &mut size, &mut transport);
        if status == NO_ERR {
            Ok(transport)
        } else {
            Err(status)
        }
    }

    pub fn uid(&self, s: Option<&Side>) -> Result<String, OSStatus> {
        let address = get_property_address(
            Property::DeviceUID,
            if let Some(side) = s {
                Scope::from(side)
            } else {
                Scope::Global
            },
        );

        let mut size = mem::size_of::<CFStringRef>();
        let mut uid: CFStringRef = ptr::null();
        let status = self
            .0
            .get_property_data_without_qualifier(&address, &mut size, &mut uid);
        if status == NO_ERR {
            let s = StringRef::new(uid);
            let utf8 = s.to_utf8();
            Ok(String::from_utf8_lossy(&utf8).to_string())
        } else {
            Err(status)
        }
    }

    fn stream_configuration(&self, s: &Side) -> Result<Vec<AudioBuffer>, OSStatus> {
        let address = get_property_address(Property::DeviceStreamConfiguration, Scope::from(s));

        let mut size = 0;
        let status = self
            .0
            .get_property_data_size_without_qualifier(&address, &mut size);
        if status != NO_ERR {
            return Err(status);
        }

        let mut buffer = vec![0u8; size];
        let status =
            self.0
                .get_property_data_without_qualifier(&address, &mut size, buffer.as_mut_ptr());
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

    fn streams(&self, s: &Side) -> Result<Vec<AudioStreamID>, OSStatus> {
        let address = get_property_address(Property::DeviceStreams, Scope::from(s));

        let mut size = 0;
        let status = self
            .0
            .get_property_data_size_without_qualifier(&address, &mut size);
        if status != NO_ERR {
            return Err(status);
        }

        let element_size = mem::size_of::<AudioStreamID>();
        assert_eq!(size % element_size, 0);
        let elements = size / element_size;
        let mut buffer = vec![AudioStreamID::default(); elements];

        let status =
            self.0
                .get_property_data_without_qualifier(&address, &mut size, buffer.as_mut_ptr());
        if status == NO_ERR {
            Ok(buffer)
        } else {
            Err(status)
        }
    }
}

#[test]
fn test_default_devices() {
    check_device_is_in_scope(Side::Input);
    check_device_is_in_scope(Side::Output);

    fn check_device_is_in_scope(s: Side) {
        use coreaudio_sys::kAudioHardwareBadObjectError;
        let system_device = SystemDevice::default();
        match system_device.get_default_device(&s) {
            Ok(device) => {
                if device.is_valid() {
                    assert!(device.in_scope(&s).unwrap());
                } else {
                    assert_eq!(
                        device.in_scope(&s).unwrap_err(),
                        kAudioHardwareBadObjectError as OSStatus
                    );
                }
            }
            Err(e) => {
                println!("Failed to get default {} device. Error: {}", s, e);
            }
        }
    }
}

#[test]
fn test_device_list() {
    let system_device = SystemDevice::default();
    let devices = system_device.get_all_devices().unwrap();
    let input = system_device.get_default_device(&Side::Input);
    let output = system_device.get_default_device(&Side::Output);
    assert_eq!(
        devices.is_empty(),
        (input.is_err() || !input.unwrap().is_valid())
            && (output.is_err() || !output.unwrap().is_valid())
    );
}
