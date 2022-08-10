// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__MagneticField__init(msg: *mut MagneticField) -> bool;
    fn sensor_msgs__msg__MagneticField__fini(msg: *mut MagneticField);
    fn sensor_msgs__msg__MagneticField__are_equal(lhs: *const MagneticField, rhs: *const MagneticField) -> bool;
    fn sensor_msgs__msg__MagneticField__Sequence__init(msg: *mut MagneticFieldSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__MagneticField__Sequence__fini(msg: *mut MagneticFieldSeqRaw);
    fn sensor_msgs__msg__MagneticField__Sequence__are_equal(lhs: *const MagneticFieldSeqRaw, rhs: *const MagneticFieldSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MagneticField() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct MagneticField {
    pub header: std_msgs::msg::Header,
    pub magnetic_field: geometry_msgs::msg::Vector3,
    pub magnetic_field_covariance: [f64; 9],
}

impl MagneticField {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MagneticField__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MagneticField {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__MagneticField__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MagneticFieldSeqRaw {
    data: *mut MagneticField,
    size: usize,
    capacity: usize,
}

/// Sequence of MagneticField.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MagneticFieldSeq<const N: usize> {
    data: *mut MagneticField,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MagneticFieldSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: MagneticFieldSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MagneticField__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[MagneticField] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MagneticField] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }
}

impl<const N: usize> Drop for MagneticFieldSeq<N> {
    fn drop(&mut self) {
        let mut msg = MagneticFieldSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__MagneticField__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MagneticFieldSeq<N> {}
unsafe impl<const N: usize> Sync for MagneticFieldSeq<N> {}


impl TopicMsg for MagneticField {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MagneticField()
        }
    }
}

impl PartialEq for MagneticField {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sensor_msgs__msg__MagneticField__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for MagneticFieldSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MagneticFieldSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = MagneticFieldSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            sensor_msgs__msg__MagneticField__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

