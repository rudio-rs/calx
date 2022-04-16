use coreaudio_sys::{
    kAudioDevicePropertyStreamConfiguration, kAudioDevicePropertyStreams,
    kAudioDevicePropertyTransportType, kAudioHardwarePropertyDefaultInputDevice,
    kAudioHardwarePropertyDefaultOutputDevice, kAudioHardwarePropertyDevices,
    kAudioObjectPropertyElementMaster, kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyScopeInput, kAudioObjectPropertyScopeOutput, AudioObjectPropertyAddress,
    AudioObjectPropertyScope, AudioObjectPropertySelector,
};

#[derive(Debug)]
pub enum Property {
    DefaultInputDevice,
    DefaultOutputDevice,
    Devices,
    DeviceStreamConfiguration,
    DeviceStreams,
    TransportType,
}

impl From<Property> for AudioObjectPropertySelector {
    fn from(p: Property) -> Self {
        match p {
            Property::DefaultInputDevice => kAudioHardwarePropertyDefaultInputDevice,
            Property::DefaultOutputDevice => kAudioHardwarePropertyDefaultOutputDevice,
            Property::Devices => kAudioHardwarePropertyDevices,
            Property::DeviceStreamConfiguration => kAudioDevicePropertyStreamConfiguration,
            Property::DeviceStreams => kAudioDevicePropertyStreams,
            Property::TransportType => kAudioDevicePropertyTransportType,
        }
    }
}

pub enum Scope {
    Global,
    Input,
    Output,
}

impl From<Scope> for AudioObjectPropertyScope {
    fn from(scope: Scope) -> Self {
        match scope {
            Scope::Global => kAudioObjectPropertyScopeGlobal,
            Scope::Input => kAudioObjectPropertyScopeInput,
            Scope::Output => kAudioObjectPropertyScopeOutput,
        }
    }
}

pub fn get_property_address(property: Property, scope: Scope) -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: AudioObjectPropertySelector::from(property),
        mScope: AudioObjectPropertyScope::from(scope),
        mElement: kAudioObjectPropertyElementMaster,
    }
}
