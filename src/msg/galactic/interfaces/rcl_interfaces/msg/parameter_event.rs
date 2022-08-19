// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn rcl_interfaces__msg__ParameterEvent__init(msg: *mut ParameterEvent) -> bool;
    fn rcl_interfaces__msg__ParameterEvent__fini(msg: *mut ParameterEvent);
    fn rcl_interfaces__msg__ParameterEvent__are_equal(
        lhs: *const ParameterEvent,
        rhs: *const ParameterEvent,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterEvent__Sequence__init(
        msg: *mut ParameterEventSeqRaw,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterEvent__Sequence__fini(msg: *mut ParameterEventSeqRaw);
    fn rcl_interfaces__msg__ParameterEvent__Sequence__are_equal(
        lhs: *const ParameterEventSeqRaw,
        rhs: *const ParameterEventSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEvent(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct ParameterEvent {
    pub stamp: builtin_interfaces::UnsafeTime,
    pub node: crate::msg::RosString<0>,
    pub new_parameters: ParameterSeq<0>,
    pub changed_parameters: ParameterSeq<0>,
    pub deleted_parameters: ParameterSeq<0>,
}

impl ParameterEvent {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__ParameterEvent__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ParameterEvent {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__msg__ParameterEvent__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ParameterEventSeqRaw {
    data: *mut ParameterEvent,
    size: usize,
    capacity: usize,
}

/// Sequence of ParameterEvent.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ParameterEventSeq<const N: usize> {
    data: *mut ParameterEvent,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ParameterEventSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ParameterEventSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__ParameterEvent__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[ParameterEvent] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ParameterEvent] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ParameterEvent> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ParameterEvent> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for ParameterEventSeq<N> {
    fn drop(&mut self) {
        let mut msg = ParameterEventSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { rcl_interfaces__msg__ParameterEvent__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ParameterEventSeq<N> {}
unsafe impl<const N: usize> Sync for ParameterEventSeq<N> {}

impl TopicMsg for ParameterEvent {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEvent()
        }
    }
}

impl PartialEq for ParameterEvent {
    fn eq(&self, other: &Self) -> bool {
        unsafe { rcl_interfaces__msg__ParameterEvent__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for ParameterEventSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = ParameterEventSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = ParameterEventSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            rcl_interfaces__msg__ParameterEvent__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
