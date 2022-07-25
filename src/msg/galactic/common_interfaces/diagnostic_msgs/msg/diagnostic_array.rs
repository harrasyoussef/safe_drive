// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn diagnostic_msgs__msg__DiagnosticArray__init(msg: *mut DiagnosticArray) -> bool;
    fn diagnostic_msgs__msg__DiagnosticArray__fini(msg: *mut DiagnosticArray);
    fn diagnostic_msgs__msg__DiagnosticArray__are_equal(lhs: *const DiagnosticArray, rhs: *const DiagnosticArray) -> bool;
    fn diagnostic_msgs__msg__DiagnosticArray__Sequence__init(msg: *mut DiagnosticArraySeqRaw, size: usize) -> bool;
    fn diagnostic_msgs__msg__DiagnosticArray__Sequence__fini(msg: *mut DiagnosticArraySeqRaw);
    fn diagnostic_msgs__msg__DiagnosticArray__Sequence__are_equal(lhs: *const DiagnosticArraySeqRaw, rhs: *const DiagnosticArraySeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__DiagnosticArray() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticArray {
    pub header: std_msgs::msg::Header,
    pub status: DiagnosticStatusSeq<0>,
}

impl DiagnosticArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__DiagnosticArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for DiagnosticArray {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__msg__DiagnosticArray__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct DiagnosticArraySeqRaw {
    data: *mut DiagnosticArray,
    size: usize,
    capacity: usize,
}

/// Sequence of DiagnosticArray.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticArraySeq<const N: usize> {
    data: *mut DiagnosticArray,
    size: usize,
    capacity: usize,
}

impl<const N: usize> DiagnosticArraySeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: DiagnosticArraySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__DiagnosticArray__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[DiagnosticArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [DiagnosticArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for DiagnosticArraySeq<N> {
    fn drop(&mut self) {
        let mut msg = DiagnosticArraySeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { diagnostic_msgs__msg__DiagnosticArray__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for DiagnosticArraySeq<N> {}
unsafe impl<const N: usize> Sync for DiagnosticArraySeq<N> {}


impl TopicMsg for DiagnosticArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__DiagnosticArray()
        }
    }
}

impl PartialEq for DiagnosticArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            diagnostic_msgs__msg__DiagnosticArray__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for DiagnosticArraySeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = DiagnosticArraySeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = DiagnosticArraySeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            diagnostic_msgs__msg__DiagnosticArray__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

