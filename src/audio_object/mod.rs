use coreaudio_sys::{
    kAudioObjectSystemObject, kAudioObjectUnknown, AudioBuffer, AudioBufferList, AudioObjectID,
    AudioStreamID, OSStatus,
};
use std::mem;
use std::slice;

mod property_address;
mod utils;

use property_address::{get_property_address, Property, PropertyScope};
use utils::{allocate_array, GetId, GetPropertyData, GetPropertyDataArray, GetPropertyDataSize};

#[derive(Clone, Debug)]
pub enum Scope {
    Input,
    Output,
}

impl From<Scope> for PropertyScope {
    fn from(scope: Scope) -> Self {
        match scope {
            Scope::Input => PropertyScope::Input,
            Scope::Output => PropertyScope::Output,
        }
    }
}

// AudioSystemObject
// ------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct AudioSystemObject(AudioObjectID);
impl AudioSystemObject {
    pub fn get_default_device(&self, scope: Scope) -> Result<AudioObject, OSStatus> {
        let address = get_property_address(
            match scope {
                Scope::Input => Property::DefaultInputDevice,
                Scope::Output => Property::DefaultOutputDevice,
            },
            PropertyScope::Global,
        );
        let mut device = AudioObject::default();
        let mut size = mem::size_of::<AudioObject>();
        self.get_property_data(&address, &mut size, &mut device)?;
        Ok(device)
    }

    pub fn get_all_devices(&self) -> Result<Vec<AudioObject>, OSStatus> {
        let address = get_property_address(Property::Devices, PropertyScope::Global);
        let devices = self.get_property_data_array(&address)?;
        Ok(devices)
    }
}

impl Default for AudioSystemObject {
    fn default() -> Self {
        Self(kAudioObjectSystemObject)
    }
}

impl GetId for AudioSystemObject {
    fn get_id(&self) -> AudioObjectID {
        self.0
    }
}

impl GetPropertyData for AudioSystemObject {}

impl GetPropertyDataArray for AudioSystemObject {}

// AudioObject
// ------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct AudioObject(AudioObjectID);
impl AudioObject {
    pub fn new(id: AudioObjectID) -> Self {
        Self(id)
    }

    pub fn is_valid(&self) -> bool {
        self.0 != kAudioObjectUnknown
    }

    pub fn in_scope(&self, scope: Scope) -> Result<bool, OSStatus> {
        let streams = self.number_of_streams(scope)?;
        Ok(streams > 0)
    }

    #[allow(clippy::cast_ptr_alignment)] // Cast *mut u8 to *mut AudioBufferList
    pub fn get_channel_count(&self, scope: Scope) -> Result<u32, OSStatus> {
        let address =
            get_property_address(Property::StreamConfiguration, PropertyScope::from(scope));
        let mut size = self.get_property_data_size(&address)?;
        let mut data = allocate_array::<u8>(size);
        let data_ptr = data.as_mut_ptr();
        self.get_property_data(&address, &mut size, data_ptr)?;
        let list = unsafe { &*(data_ptr as *mut AudioBufferList) };
        let buffers = unsafe {
            slice::from_raw_parts(
                list.mBuffers.as_ptr() as *const AudioBuffer,
                list.mNumberBuffers as usize,
            )
        };
        let mut count = 0;
        for buffer in buffers {
            count += buffer.mNumberChannels;
        }
        Ok(count)
    }

    fn number_of_streams(&self, scope: Scope) -> Result<usize, OSStatus> {
        let address = get_property_address(Property::DeviceStreams, PropertyScope::from(scope));
        let size = self.get_property_data_size(&address)?;
        Ok(size / mem::size_of::<AudioStream>())
    }
}

impl Default for AudioObject {
    fn default() -> Self {
        Self::new(kAudioObjectUnknown)
    }
}

impl GetId for AudioObject {
    fn get_id(&self) -> AudioObjectID {
        self.0
    }
}

impl GetPropertyData for AudioObject {}

impl GetPropertyDataSize for AudioObject {}

// AudioStream
// ------------------------------------------------------------------------------------------------
struct AudioStream(AudioStreamID);
