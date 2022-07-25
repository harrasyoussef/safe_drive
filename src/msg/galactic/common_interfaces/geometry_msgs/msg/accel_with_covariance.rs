// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__AccelWithCovariance__init(msg: *mut AccelWithCovariance) -> bool;
    fn geometry_msgs__msg__AccelWithCovariance__fini(msg: *mut AccelWithCovariance);
    fn geometry_msgs__msg__AccelWithCovariance__are_equal(lhs: *const AccelWithCovariance, rhs: *const AccelWithCovariance) -> bool;
    fn geometry_msgs__msg__AccelWithCovariance__Sequence__init(msg: *mut AccelWithCovarianceSeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__AccelWithCovariance__Sequence__fini(msg: *mut AccelWithCovarianceSeqRaw);
    fn geometry_msgs__msg__AccelWithCovariance__Sequence__are_equal(lhs: *const AccelWithCovarianceSeqRaw, rhs: *const AccelWithCovarianceSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__AccelWithCovariance() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct AccelWithCovariance {
    pub accel: Accel,
    pub covariance: [f64; 36],
}

impl AccelWithCovariance {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__AccelWithCovariance__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for AccelWithCovariance {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__AccelWithCovariance__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct AccelWithCovarianceSeqRaw {
    data: *mut AccelWithCovariance,
    size: usize,
    capacity: usize,
}

/// Sequence of AccelWithCovariance.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct AccelWithCovarianceSeq<const N: usize> {
    data: *mut AccelWithCovariance,
    size: usize,
    capacity: usize,
}

impl<const N: usize> AccelWithCovarianceSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: AccelWithCovarianceSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__AccelWithCovariance__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[AccelWithCovariance]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [AccelWithCovariance]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for AccelWithCovarianceSeq<N> {
    fn drop(&mut self) {
        let mut msg = AccelWithCovarianceSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { geometry_msgs__msg__AccelWithCovariance__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for AccelWithCovarianceSeq<N> {}
unsafe impl<const N: usize> Sync for AccelWithCovarianceSeq<N> {}


impl TopicMsg for AccelWithCovariance {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__AccelWithCovariance()
        }
    }
}

impl PartialEq for AccelWithCovariance {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            geometry_msgs__msg__AccelWithCovariance__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for AccelWithCovarianceSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = AccelWithCovarianceSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = AccelWithCovarianceSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            geometry_msgs__msg__AccelWithCovariance__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

