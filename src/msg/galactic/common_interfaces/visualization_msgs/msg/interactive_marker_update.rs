// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;
pub const KEEP_ALIVE: u8 = 0;
pub const UPDATE: u8 = 1;

extern "C" {
    fn visualization_msgs__msg__InteractiveMarkerUpdate__init(
        msg: *mut InteractiveMarkerUpdate,
    ) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerUpdate__fini(msg: *mut InteractiveMarkerUpdate);
    fn visualization_msgs__msg__InteractiveMarkerUpdate__are_equal(
        lhs: *const InteractiveMarkerUpdate,
        rhs: *const InteractiveMarkerUpdate,
    ) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerUpdate__Sequence__init(
        msg: *mut InteractiveMarkerUpdateSeqRaw,
        size: usize,
    ) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerUpdate__Sequence__fini(
        msg: *mut InteractiveMarkerUpdateSeqRaw,
    );
    fn visualization_msgs__msg__InteractiveMarkerUpdate__Sequence__are_equal(
        lhs: *const InteractiveMarkerUpdateSeqRaw,
        rhs: *const InteractiveMarkerUpdateSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarkerUpdate(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerUpdate {
    pub server_id: crate::msg::RosString<0>,
    pub seq_num: u64,
    pub type_: u8,
    pub markers: InteractiveMarkerSeq<0>,
    pub poses: InteractiveMarkerPoseSeq<0>,
    pub erases: crate::msg::RosStringSeq<0, 0>,
}

impl InteractiveMarkerUpdate {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarkerUpdate__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for InteractiveMarkerUpdate {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__InteractiveMarkerUpdate__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct InteractiveMarkerUpdateSeqRaw {
    data: *mut InteractiveMarkerUpdate,
    size: usize,
    capacity: usize,
}

/// Sequence of InteractiveMarkerUpdate.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerUpdateSeq<const N: usize> {
    data: *mut InteractiveMarkerUpdate,
    size: usize,
    capacity: usize,
}

impl<const N: usize> InteractiveMarkerUpdateSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: InteractiveMarkerUpdateSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            visualization_msgs__msg__InteractiveMarkerUpdate__Sequence__init(&mut msg, size)
        } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[InteractiveMarkerUpdate] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [InteractiveMarkerUpdate] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, InteractiveMarkerUpdate> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, InteractiveMarkerUpdate> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for InteractiveMarkerUpdateSeq<N> {
    fn drop(&mut self) {
        let mut msg = InteractiveMarkerUpdateSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { visualization_msgs__msg__InteractiveMarkerUpdate__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for InteractiveMarkerUpdateSeq<N> {}
unsafe impl<const N: usize> Sync for InteractiveMarkerUpdateSeq<N> {}

impl TopicMsg for InteractiveMarkerUpdate {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarkerUpdate()
        }
    }
}

impl PartialEq for InteractiveMarkerUpdate {
    fn eq(&self, other: &Self) -> bool {
        unsafe { visualization_msgs__msg__InteractiveMarkerUpdate__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for InteractiveMarkerUpdateSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = InteractiveMarkerUpdateSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = InteractiveMarkerUpdateSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            visualization_msgs__msg__InteractiveMarkerUpdate__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
