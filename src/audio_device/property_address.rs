use coreaudio_sys::{
    kAudioDevicePropertyAvailableNominalSampleRates, kAudioDevicePropertyBufferFrameSizeRange,
    kAudioDevicePropertyClockDomain, kAudioDevicePropertyDataSource,
    kAudioDevicePropertyDataSourceNameForIDCFString, kAudioDevicePropertyDeviceUID,
    kAudioDevicePropertyLatency, kAudioDevicePropertyModelUID,
    kAudioDevicePropertyNominalSampleRate, kAudioDevicePropertyStreamConfiguration,
    kAudioDevicePropertyStreams, kAudioDevicePropertyTransportType,
    kAudioHardwarePropertyDefaultInputDevice, kAudioHardwarePropertyDefaultOutputDevice,
    kAudioHardwarePropertyDevices, kAudioObjectPropertyElementMaster,
    kAudioObjectPropertyManufacturer, kAudioObjectPropertyName, kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyScopeInput, kAudioObjectPropertyScopeOutput, AudioObjectPropertyAddress,
    AudioObjectPropertyScope, AudioObjectPropertySelector,
};

pub enum Property {
    DefaultInputDevice,
    DefaultOutputDevice,
    Devices,
    DeviceManufacturer,
    DeviceName,
    ClockDomain,
    DeviceBufferFrameSizeRange,
    DeviceModelUID,
    DeviceLatency,
    DeviceSampleRate,
    DeviceSampleRates,
    DeviceSource,
    DeviceSourceName,
    DeviceStreamConfiguration,
    DeviceStreams,
    DeviceUID,
    TransportType,
}

impl From<Property> for AudioObjectPropertySelector {
    fn from(p: Property) -> Self {
        match p {
            // kAudioHardwareProperty*
            Property::DefaultInputDevice => kAudioHardwarePropertyDefaultInputDevice,
            Property::DefaultOutputDevice => kAudioHardwarePropertyDefaultOutputDevice,
            Property::Devices => kAudioHardwarePropertyDevices,
            // kAudioObject*
            Property::DeviceManufacturer => kAudioObjectPropertyManufacturer,
            Property::DeviceName => kAudioObjectPropertyName,
            // kAudioDeviceProperty*
            Property::ClockDomain => kAudioDevicePropertyClockDomain,
            Property::DeviceBufferFrameSizeRange => kAudioDevicePropertyBufferFrameSizeRange,
            Property::DeviceModelUID => kAudioDevicePropertyModelUID,
            Property::DeviceLatency => kAudioDevicePropertyLatency,
            Property::DeviceSampleRate => kAudioDevicePropertyNominalSampleRate,
            Property::DeviceSampleRates => kAudioDevicePropertyAvailableNominalSampleRates,
            Property::DeviceSource => kAudioDevicePropertyDataSource,
            Property::DeviceSourceName => kAudioDevicePropertyDataSourceNameForIDCFString,
            Property::DeviceStreamConfiguration => kAudioDevicePropertyStreamConfiguration,
            Property::DeviceStreams => kAudioDevicePropertyStreams,
            Property::DeviceUID => kAudioDevicePropertyDeviceUID,
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
