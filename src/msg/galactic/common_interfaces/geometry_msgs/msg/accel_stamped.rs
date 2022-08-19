// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__AccelStamped__init(msg: *mut AccelStamped) -> bool;
    fn geometry_msgs__msg__AccelStamped__fini(msg: *mut AccelStamped);
    fn geometry_msgs__msg__AccelStamped__are_equal(
        lhs: *const AccelStamped,
        rhs: *const AccelStamped,
    ) -> bool;
    fn geometry_msgs__msg__AccelStamped__Sequence__init(
        msg: *mut AccelStampedSeqRaw,
        size: usize,
    ) -> bool;
    fn geometry_msgs__msg__AccelStamped__Sequence__fini(msg: *mut AccelStampedSeqRaw);
    fn geometry_msgs__msg__AccelStamped__Sequence__are_equal(
        lhs: *const AccelStampedSeqRaw,
        rhs: *const AccelStampedSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__AccelStamped(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct AccelStamped {
    pub header: std_msgs::msg::Header,
    pub accel: Accel,
}

impl AccelStamped {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__AccelStamped__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for AccelStamped {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__AccelStamped__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct AccelStampedSeqRaw {
    data: *mut AccelStamped,
    size: usize,
    capacity: usize,
}

/// Sequence of AccelStamped.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct AccelStampedSeq<const N: usize> {
    data: *mut AccelStamped,
    size: usize,
    capacity: usize,
}

impl<const N: usize> AccelStampedSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: AccelStampedSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__AccelStamped__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[AccelStamped] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [AccelStamped] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, AccelStamped> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, AccelStamped> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for AccelStampedSeq<N> {
    fn drop(&mut self) {
        let mut msg = AccelStampedSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { geometry_msgs__msg__AccelStamped__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for AccelStampedSeq<N> {}
unsafe impl<const N: usize> Sync for AccelStampedSeq<N> {}

impl TopicMsg for AccelStamped {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__AccelStamped(
            )
        }
    }
}

impl PartialEq for AccelStamped {
    fn eq(&self, other: &Self) -> bool {
        unsafe { geometry_msgs__msg__AccelStamped__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for AccelStampedSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = AccelStampedSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = AccelStampedSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            geometry_msgs__msg__AccelStamped__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
