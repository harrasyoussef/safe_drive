// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn diagnostic_msgs__msg__KeyValue__init(msg: *mut KeyValue) -> bool;
    fn diagnostic_msgs__msg__KeyValue__fini(msg: *mut KeyValue);
    fn diagnostic_msgs__msg__KeyValue__are_equal(lhs: *const KeyValue, rhs: *const KeyValue) -> bool;
    fn diagnostic_msgs__msg__KeyValue__Sequence__init(msg: *mut KeyValueSeqRaw, size: usize) -> bool;
    fn diagnostic_msgs__msg__KeyValue__Sequence__fini(msg: *mut KeyValueSeqRaw);
    fn diagnostic_msgs__msg__KeyValue__Sequence__are_equal(lhs: *const KeyValueSeqRaw, rhs: *const KeyValueSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__KeyValue() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct KeyValue {
    pub key: crate::msg::RosString<0>,
    pub value: crate::msg::RosString<0>,
}

impl KeyValue {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__KeyValue__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for KeyValue {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__msg__KeyValue__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct KeyValueSeqRaw {
    data: *mut KeyValue,
    size: usize,
    capacity: usize,
}

/// Sequence of KeyValue.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct KeyValueSeq<const N: usize> {
    data: *mut KeyValue,
    size: usize,
    capacity: usize,
}

impl<const N: usize> KeyValueSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: KeyValueSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__KeyValue__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[KeyValue]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [KeyValue]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for KeyValueSeq<N> {
    fn drop(&mut self) {
        let mut msg = KeyValueSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { diagnostic_msgs__msg__KeyValue__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for KeyValueSeq<N> {}
unsafe impl<const N: usize> Sync for KeyValueSeq<N> {}


impl TopicMsg for KeyValue {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__KeyValue()
        }
    }
}

impl PartialEq for KeyValue {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            diagnostic_msgs__msg__KeyValue__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for KeyValueSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = KeyValueSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = KeyValueSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            diagnostic_msgs__msg__KeyValue__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

