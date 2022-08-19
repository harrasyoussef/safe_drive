// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Transform__init(msg: *mut Transform) -> bool;
    fn geometry_msgs__msg__Transform__fini(msg: *mut Transform);
    fn geometry_msgs__msg__Transform__are_equal(
        lhs: *const Transform,
        rhs: *const Transform,
    ) -> bool;
    fn geometry_msgs__msg__Transform__Sequence__init(
        msg: *mut TransformSeqRaw,
        size: usize,
    ) -> bool;
    fn geometry_msgs__msg__Transform__Sequence__fini(msg: *mut TransformSeqRaw);
    fn geometry_msgs__msg__Transform__Sequence__are_equal(
        lhs: *const TransformSeqRaw,
        rhs: *const TransformSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Transform(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

impl Transform {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Transform__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Transform {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Transform__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct TransformSeqRaw {
    data: *mut Transform,
    size: usize,
    capacity: usize,
}

/// Sequence of Transform.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct TransformSeq<const N: usize> {
    data: *mut Transform,
    size: usize,
    capacity: usize,
}

impl<const N: usize> TransformSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: TransformSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Transform__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[Transform] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Transform] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Transform> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Transform> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for TransformSeq<N> {
    fn drop(&mut self) {
        let mut msg = TransformSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { geometry_msgs__msg__Transform__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for TransformSeq<N> {}
unsafe impl<const N: usize> Sync for TransformSeq<N> {}

impl TopicMsg for Transform {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Transform()
        }
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        unsafe { geometry_msgs__msg__Transform__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for TransformSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = TransformSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = TransformSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            geometry_msgs__msg__Transform__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
