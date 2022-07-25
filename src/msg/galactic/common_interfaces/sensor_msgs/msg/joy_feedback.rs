// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const TYPE_LED: u8 = 0;
pub const TYPE_RUMBLE: u8 = 1;
pub const TYPE_BUZZER: u8 = 2;

extern "C" {
    fn sensor_msgs__msg__JoyFeedback__init(msg: *mut JoyFeedback) -> bool;
    fn sensor_msgs__msg__JoyFeedback__fini(msg: *mut JoyFeedback);
    fn sensor_msgs__msg__JoyFeedback__are_equal(lhs: *const JoyFeedback, rhs: *const JoyFeedback) -> bool;
    fn sensor_msgs__msg__JoyFeedback__Sequence__init(msg: *mut JoyFeedbackSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__JoyFeedback__Sequence__fini(msg: *mut JoyFeedbackSeqRaw);
    fn sensor_msgs__msg__JoyFeedback__Sequence__are_equal(lhs: *const JoyFeedbackSeqRaw, rhs: *const JoyFeedbackSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JoyFeedback() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct JoyFeedback {
    pub type_: u8,
    pub id: u8,
    pub intensity: f32,
}

impl JoyFeedback {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__JoyFeedback__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for JoyFeedback {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__JoyFeedback__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct JoyFeedbackSeqRaw {
    data: *mut JoyFeedback,
    size: usize,
    capacity: usize,
}

/// Sequence of JoyFeedback.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct JoyFeedbackSeq<const N: usize> {
    data: *mut JoyFeedback,
    size: usize,
    capacity: usize,
}

impl<const N: usize> JoyFeedbackSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: JoyFeedbackSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__JoyFeedback__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[JoyFeedback]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [JoyFeedback]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for JoyFeedbackSeq<N> {
    fn drop(&mut self) {
        let mut msg = JoyFeedbackSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__JoyFeedback__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for JoyFeedbackSeq<N> {}
unsafe impl<const N: usize> Sync for JoyFeedbackSeq<N> {}


impl TopicMsg for JoyFeedback {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JoyFeedback()
        }
    }
}

impl PartialEq for JoyFeedback {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sensor_msgs__msg__JoyFeedback__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for JoyFeedbackSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = JoyFeedbackSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = JoyFeedbackSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            sensor_msgs__msg__JoyFeedback__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

