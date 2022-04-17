use coreaudio_sys::{
    noErr, AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize, AudioObjectID,
    AudioObjectPropertyAddress, OSStatus, UInt32,
};
use std::mem;
use std::os::raw::c_void;

pub struct AudioObject(AudioObjectID);
impl AudioObject {
    pub fn new(id: AudioObjectID) -> Self {
        Self(id)
    }

    pub fn id(&self) -> AudioObjectID {
        self.0
    }

    pub fn get_property_data<Q, D>(
        &self,
        address: &AudioObjectPropertyAddress,
        in_qualifier_data_size: usize,
        in_qualifier_data: *mut Q,
        io_data_size: *mut usize,
        out_data: *mut D,
    ) -> OSStatus {
        audio_object_get_property_data(
            self.0,
            address,
            in_qualifier_data_size,
            in_qualifier_data,
            io_data_size,
            out_data,
        )
    }

    pub fn get_property_data_without_qualifier<D>(
        &self,
        address: &AudioObjectPropertyAddress,
        io_data_size: *mut usize,
        out_data: *mut D,
    ) -> OSStatus {
        self.get_property_data(
            address,
            0,
            std::ptr::null_mut::<c_void>(),
            io_data_size,
            out_data,
        )
    }

    pub fn get_property_data_common<D: Default + Sized>(
        &self,
        address: &AudioObjectPropertyAddress,
    ) -> Result<D, OSStatus> {
        const NO_ERR: OSStatus = noErr as OSStatus;
        let mut data = D::default();
        let mut size = mem::size_of::<D>();
        let status = self.get_property_data_without_qualifier(address, &mut size, &mut data);
        if status == NO_ERR {
            Ok(data)
        } else {
            Err(status)
        }
    }

    pub fn get_property_data_size<Q>(
        &self,
        address: &AudioObjectPropertyAddress,
        in_qualifier_data_size: usize,
        in_qualifier_data: *mut Q,
        out_data_size: *mut usize,
    ) -> OSStatus {
        audio_object_get_property_data_size(
            self.0,
            address,
            in_qualifier_data_size,
            in_qualifier_data,
            out_data_size,
        )
    }

    pub fn get_property_data_size_without_qualifier(
        &self,
        address: &AudioObjectPropertyAddress,
        out_data_size: *mut usize,
    ) -> OSStatus {
        self.get_property_data_size(address, 0, std::ptr::null_mut::<c_void>(), out_data_size)
    }
}

fn audio_object_get_property_data<Q, D>(
    in_object_id: AudioObjectID,
    in_address: &AudioObjectPropertyAddress,
    in_qualifier_data_size: usize,
    in_qualifier_data: *mut Q,
    io_data_size: *mut usize,
    out_data: *mut D,
) -> OSStatus {
    assert!(
        (in_qualifier_data.is_null() && in_qualifier_data_size == 0)
            || (!in_qualifier_data.is_null() && in_qualifier_data_size >= mem::size_of::<Q>())
    );
    assert!(!io_data_size.is_null());
    assert!(!out_data.is_null());
    unsafe {
        assert!(*io_data_size == 0 || *io_data_size >= mem::size_of::<D>());
        AudioObjectGetPropertyData(
            in_object_id,
            in_address,
            in_qualifier_data_size as UInt32,
            in_qualifier_data as *mut c_void,
            io_data_size as *mut UInt32,
            out_data as *mut c_void,
        )
    }
}

fn audio_object_get_property_data_size<Q>(
    in_object_id: AudioObjectID,
    in_address: &AudioObjectPropertyAddress,
    in_qualifier_data_size: usize,
    in_qualifier_data: *mut Q,
    out_data_size: *mut usize,
) -> OSStatus {
    assert!(
        (in_qualifier_data.is_null() && in_qualifier_data_size == 0)
            || (!in_qualifier_data.is_null() && in_qualifier_data_size >= mem::size_of::<Q>())
    );
    unsafe {
        AudioObjectGetPropertyDataSize(
            in_object_id,
            in_address,
            in_qualifier_data_size as UInt32,
            in_qualifier_data as *mut c_void,
            out_data_size as *mut UInt32,
        )
    }
}
