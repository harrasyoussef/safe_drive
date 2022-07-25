// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__TwistWithCovariance__init(msg: *mut TwistWithCovariance) -> bool;
    fn geometry_msgs__msg__TwistWithCovariance__fini(msg: *mut TwistWithCovariance);
    fn geometry_msgs__msg__TwistWithCovariance__are_equal(lhs: *const TwistWithCovariance, rhs: *const TwistWithCovariance) -> bool;
    fn geometry_msgs__msg__TwistWithCovariance__Sequence__init(msg: *mut TwistWithCovarianceSeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__TwistWithCovariance__Sequence__fini(msg: *mut TwistWithCovarianceSeqRaw);
    fn geometry_msgs__msg__TwistWithCovariance__Sequence__are_equal(lhs: *const TwistWithCovarianceSeqRaw, rhs: *const TwistWithCovarianceSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__TwistWithCovariance() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct TwistWithCovariance {
    pub twist: Twist,
    pub covariance: [f64; 36],
}

impl TwistWithCovariance {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__TwistWithCovariance__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TwistWithCovariance {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__TwistWithCovariance__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct TwistWithCovarianceSeqRaw {
    data: *mut TwistWithCovariance,
    size: usize,
    capacity: usize,
}

/// Sequence of TwistWithCovariance.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct TwistWithCovarianceSeq<const N: usize> {
    data: *mut TwistWithCovariance,
    size: usize,
    capacity: usize,
}

impl<const N: usize> TwistWithCovarianceSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: TwistWithCovarianceSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__TwistWithCovariance__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[TwistWithCovariance]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [TwistWithCovariance]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for TwistWithCovarianceSeq<N> {
    fn drop(&mut self) {
        let mut msg = TwistWithCovarianceSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { geometry_msgs__msg__TwistWithCovariance__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for TwistWithCovarianceSeq<N> {}
unsafe impl<const N: usize> Sync for TwistWithCovarianceSeq<N> {}


impl TopicMsg for TwistWithCovariance {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__TwistWithCovariance()
        }
    }
}

impl PartialEq for TwistWithCovariance {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            geometry_msgs__msg__TwistWithCovariance__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for TwistWithCovarianceSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = TwistWithCovarianceSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = TwistWithCovarianceSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            geometry_msgs__msg__TwistWithCovariance__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

