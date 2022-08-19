// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;
pub const OK: u8 = 0;
pub const WARN: u8 = 1;
pub const ERROR: u8 = 2;
pub const STALE: u8 = 3;

extern "C" {
    fn diagnostic_msgs__msg__DiagnosticStatus__init(msg: *mut DiagnosticStatus) -> bool;
    fn diagnostic_msgs__msg__DiagnosticStatus__fini(msg: *mut DiagnosticStatus);
    fn diagnostic_msgs__msg__DiagnosticStatus__are_equal(
        lhs: *const DiagnosticStatus,
        rhs: *const DiagnosticStatus,
    ) -> bool;
    fn diagnostic_msgs__msg__DiagnosticStatus__Sequence__init(
        msg: *mut DiagnosticStatusSeqRaw,
        size: usize,
    ) -> bool;
    fn diagnostic_msgs__msg__DiagnosticStatus__Sequence__fini(msg: *mut DiagnosticStatusSeqRaw);
    fn diagnostic_msgs__msg__DiagnosticStatus__Sequence__are_equal(
        lhs: *const DiagnosticStatusSeqRaw,
        rhs: *const DiagnosticStatusSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__DiagnosticStatus(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticStatus {
    pub level: u8,
    pub name: crate::msg::RosString<0>,
    pub message: crate::msg::RosString<0>,
    pub hardware_id: crate::msg::RosString<0>,
    pub values: KeyValueSeq<0>,
}

impl DiagnosticStatus {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__DiagnosticStatus__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for DiagnosticStatus {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__msg__DiagnosticStatus__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct DiagnosticStatusSeqRaw {
    data: *mut DiagnosticStatus,
    size: usize,
    capacity: usize,
}

/// Sequence of DiagnosticStatus.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticStatusSeq<const N: usize> {
    data: *mut DiagnosticStatus,
    size: usize,
    capacity: usize,
}

impl<const N: usize> DiagnosticStatusSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: DiagnosticStatusSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__DiagnosticStatus__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[DiagnosticStatus] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [DiagnosticStatus] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, DiagnosticStatus> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, DiagnosticStatus> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for DiagnosticStatusSeq<N> {
    fn drop(&mut self) {
        let mut msg = DiagnosticStatusSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { diagnostic_msgs__msg__DiagnosticStatus__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for DiagnosticStatusSeq<N> {}
unsafe impl<const N: usize> Sync for DiagnosticStatusSeq<N> {}

impl TopicMsg for DiagnosticStatus {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__DiagnosticStatus()
        }
    }
}

impl PartialEq for DiagnosticStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe { diagnostic_msgs__msg__DiagnosticStatus__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for DiagnosticStatusSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = DiagnosticStatusSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = DiagnosticStatusSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            diagnostic_msgs__msg__DiagnosticStatus__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
