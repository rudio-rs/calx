use coreaudio_sys::{
    kAudioDevicePropertyStreamConfiguration, kAudioDevicePropertyStreams,
    kAudioHardwarePropertyDefaultInputDevice, kAudioHardwarePropertyDefaultOutputDevice,
    kAudioHardwarePropertyDevices, kAudioObjectPropertyElementMaster,
    kAudioObjectPropertyScopeGlobal, kAudioObjectPropertyScopeInput,
    kAudioObjectPropertyScopeOutput, AudioObjectPropertyAddress, AudioObjectPropertyScope,
    AudioObjectPropertySelector,
};

#[derive(Debug)]
pub enum Property {
    DefaultInputDevice,
    DefaultOutputDevice,
    Devices,
    DeviceStreamConfiguration,
    DeviceStreams,
}

impl From<Property> for AudioObjectPropertySelector {
    fn from(p: Property) -> Self {
        match p {
            Property::DefaultInputDevice => kAudioHardwarePropertyDefaultInputDevice,
            Property::DefaultOutputDevice => kAudioHardwarePropertyDefaultOutputDevice,
            Property::Devices => kAudioHardwarePropertyDevices,
            Property::DeviceStreamConfiguration => kAudioDevicePropertyStreamConfiguration,
            Property::DeviceStreams => kAudioDevicePropertyStreams,
        }
    }
}

#[derive(Debug)]
pub enum PropertyScope {
    Global,
    Input,
    Output,
}

impl From<PropertyScope> for AudioObjectPropertyScope {
    fn from(scope: PropertyScope) -> Self {
        match scope {
            PropertyScope::Global => kAudioObjectPropertyScopeGlobal,
            PropertyScope::Input => kAudioObjectPropertyScopeInput,
            PropertyScope::Output => kAudioObjectPropertyScopeOutput,
        }
    }
}

pub fn get_property_address(
    property: Property,
    scope: PropertyScope,
) -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: AudioObjectPropertySelector::from(property),
        mScope: AudioObjectPropertyScope::from(scope),
        mElement: kAudioObjectPropertyElementMaster,
    }
}
