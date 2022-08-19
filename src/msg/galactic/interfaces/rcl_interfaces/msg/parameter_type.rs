// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;
pub const PARAMETER_NOT_SET: u8 = 0;
pub const PARAMETER_BOOL: u8 = 1;
pub const PARAMETER_INTEGER: u8 = 2;
pub const PARAMETER_DOUBLE: u8 = 3;
pub const PARAMETER_STRING: u8 = 4;
pub const PARAMETER_BYTE_ARRAY: u8 = 5;
pub const PARAMETER_BOOL_ARRAY: u8 = 6;
pub const PARAMETER_INTEGER_ARRAY: u8 = 7;
pub const PARAMETER_DOUBLE_ARRAY: u8 = 8;
pub const PARAMETER_STRING_ARRAY: u8 = 9;

extern "C" {
    fn rcl_interfaces__msg__ParameterType__init(msg: *mut ParameterType) -> bool;
    fn rcl_interfaces__msg__ParameterType__fini(msg: *mut ParameterType);
    fn rcl_interfaces__msg__ParameterType__are_equal(
        lhs: *const ParameterType,
        rhs: *const ParameterType,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterType__Sequence__init(
        msg: *mut ParameterTypeSeqRaw,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterType__Sequence__fini(msg: *mut ParameterTypeSeqRaw);
    fn rcl_interfaces__msg__ParameterType__Sequence__are_equal(
        lhs: *const ParameterTypeSeqRaw,
        rhs: *const ParameterTypeSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterType(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct ParameterType {
    _unused: u8,
}

impl ParameterType {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__ParameterType__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ParameterType {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__msg__ParameterType__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ParameterTypeSeqRaw {
    data: *mut ParameterType,
    size: usize,
    capacity: usize,
}

/// Sequence of ParameterType.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ParameterTypeSeq<const N: usize> {
    data: *mut ParameterType,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ParameterTypeSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ParameterTypeSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__ParameterType__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[ParameterType] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ParameterType] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ParameterType> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ParameterType> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for ParameterTypeSeq<N> {
    fn drop(&mut self) {
        let mut msg = ParameterTypeSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { rcl_interfaces__msg__ParameterType__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ParameterTypeSeq<N> {}
unsafe impl<const N: usize> Sync for ParameterTypeSeq<N> {}

impl TopicMsg for ParameterType {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterType()
        }
    }
}

impl PartialEq for ParameterType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { rcl_interfaces__msg__ParameterType__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for ParameterTypeSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = ParameterTypeSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = ParameterTypeSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            rcl_interfaces__msg__ParameterType__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
