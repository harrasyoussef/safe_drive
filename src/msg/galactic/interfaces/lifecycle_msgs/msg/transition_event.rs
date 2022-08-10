// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn lifecycle_msgs__msg__TransitionEvent__init(msg: *mut TransitionEvent) -> bool;
    fn lifecycle_msgs__msg__TransitionEvent__fini(msg: *mut TransitionEvent);
    fn lifecycle_msgs__msg__TransitionEvent__are_equal(lhs: *const TransitionEvent, rhs: *const TransitionEvent) -> bool;
    fn lifecycle_msgs__msg__TransitionEvent__Sequence__init(msg: *mut TransitionEventSeqRaw, size: usize) -> bool;
    fn lifecycle_msgs__msg__TransitionEvent__Sequence__fini(msg: *mut TransitionEventSeqRaw);
    fn lifecycle_msgs__msg__TransitionEvent__Sequence__are_equal(lhs: *const TransitionEventSeqRaw, rhs: *const TransitionEventSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__msg__TransitionEvent() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct TransitionEvent {
    pub timestamp: u64,
    pub transition: Transition,
    pub start_state: State,
    pub goal_state: State,
}

impl TransitionEvent {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__msg__TransitionEvent__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TransitionEvent {
    fn drop(&mut self) {
        unsafe { lifecycle_msgs__msg__TransitionEvent__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct TransitionEventSeqRaw {
    data: *mut TransitionEvent,
    size: usize,
    capacity: usize,
}

/// Sequence of TransitionEvent.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct TransitionEventSeq<const N: usize> {
    data: *mut TransitionEvent,
    size: usize,
    capacity: usize,
}

impl<const N: usize> TransitionEventSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: TransitionEventSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__msg__TransitionEvent__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[TransitionEvent] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [TransitionEvent] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }
}

impl<const N: usize> Drop for TransitionEventSeq<N> {
    fn drop(&mut self) {
        let mut msg = TransitionEventSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { lifecycle_msgs__msg__TransitionEvent__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for TransitionEventSeq<N> {}
unsafe impl<const N: usize> Sync for TransitionEventSeq<N> {}


impl TopicMsg for TransitionEvent {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__msg__TransitionEvent()
        }
    }
}

impl PartialEq for TransitionEvent {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            lifecycle_msgs__msg__TransitionEvent__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for TransitionEventSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = TransitionEventSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = TransitionEventSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            lifecycle_msgs__msg__TransitionEvent__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

