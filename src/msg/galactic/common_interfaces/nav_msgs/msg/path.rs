// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn nav_msgs__msg__Path__init(msg: *mut Path) -> bool;
    fn nav_msgs__msg__Path__fini(msg: *mut Path);
    fn nav_msgs__msg__Path__are_equal(lhs: *const Path, rhs: *const Path) -> bool;
    fn nav_msgs__msg__Path__Sequence__init(msg: *mut PathSeqRaw, size: usize) -> bool;
    fn nav_msgs__msg__Path__Sequence__fini(msg: *mut PathSeqRaw);
    fn nav_msgs__msg__Path__Sequence__are_equal(
        lhs: *const PathSeqRaw,
        rhs: *const PathSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__Path(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct Path {
    pub header: std_msgs::msg::Header,
    pub poses: geometry_msgs::msg::PoseStampedSeq<0>,
}

impl Path {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__Path__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { nav_msgs__msg__Path__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct PathSeqRaw {
    data: *mut Path,
    size: usize,
    capacity: usize,
}

/// Sequence of Path.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PathSeq<const N: usize> {
    data: *mut Path,
    size: usize,
    capacity: usize,
}

impl<const N: usize> PathSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PathSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__Path__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[Path] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Path] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Path> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Path> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for PathSeq<N> {
    fn drop(&mut self) {
        let mut msg = PathSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { nav_msgs__msg__Path__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PathSeq<N> {}
unsafe impl<const N: usize> Sync for PathSeq<N> {}

impl TopicMsg for Path {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe { rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__Path() }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        unsafe { nav_msgs__msg__Path__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for PathSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = PathSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = PathSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            nav_msgs__msg__Path__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
