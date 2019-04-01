use coreaudio_sys::{
    AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize, AudioObjectID,
    AudioObjectPropertyAddress, OSStatus, UInt32,
};
use std::mem;
use std::os::raw::c_void;
use std::ptr;

const NO_ERR: OSStatus = 0;

pub trait GetId {
    fn get_id(&self) -> AudioObjectID;
}

pub trait GetPropertyData {
    fn get_property_data<T>(
        &self,
        address: &AudioObjectPropertyAddress,
        size: *mut usize,
        data: *mut T,
    ) -> Result<(), OSStatus>
    where
        Self: GetId,
    {
        let status = audio_object_get_property_data(
            self.get_id(),
            address,
            0,
            ptr::null_mut::<c_void>(),
            size,
            data,
        );
        if status == NO_ERR {
            Ok(())
        } else {
            Err(status)
        }
    }
}

pub trait GetPropertyDataSize {
    fn get_property_data_size(
        &self,
        address: &AudioObjectPropertyAddress,
    ) -> Result<usize, OSStatus>
    where
        Self: GetId,
    {
        let mut size = 0;
        let status = audio_object_get_property_data_size(
            self.get_id(),
            address,
            0,
            ptr::null_mut::<c_void>(),
            &mut size,
        );
        if status == NO_ERR {
            Ok(size)
        } else {
            Err(status)
        }
    }
}

pub trait GetPropertyDataArray {
    fn get_property_data_array<T>(
        &self,
        address: &AudioObjectPropertyAddress,
    ) -> Result<Vec<T>, OSStatus>
    where
        Self: GetId,
    {
        let mut size = 0;
        let status = audio_object_get_property_data_size(
            self.get_id(),
            address,
            0,
            ptr::null_mut::<c_void>(),
            &mut size,
        );
        if status != NO_ERR {
            return Err(status);
        }
        let mut array: Vec<T> = allocate_array(size);
        let status = audio_object_get_property_data(
            self.get_id(),
            address,
            0,
            ptr::null_mut::<c_void>(),
            &mut size,
            array.as_mut_ptr(),
        );
        if status == NO_ERR {
            Ok(array)
        } else {
            Err(status)
        }
    }
}

pub fn allocate_array<T>(size: usize) -> Vec<T> {
    let element_size = mem::size_of::<T>();
    assert_eq!(size % element_size, 0);
    let elements = size / element_size;
    let mut buffer = Vec::<T>::with_capacity(elements);
    unsafe {
        buffer.set_len(elements);
    }
    buffer
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
