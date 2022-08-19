// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__PointCloud__init(msg: *mut PointCloud) -> bool;
    fn sensor_msgs__msg__PointCloud__fini(msg: *mut PointCloud);
    fn sensor_msgs__msg__PointCloud__are_equal(
        lhs: *const PointCloud,
        rhs: *const PointCloud,
    ) -> bool;
    fn sensor_msgs__msg__PointCloud__Sequence__init(
        msg: *mut PointCloudSeqRaw,
        size: usize,
    ) -> bool;
    fn sensor_msgs__msg__PointCloud__Sequence__fini(msg: *mut PointCloudSeqRaw);
    fn sensor_msgs__msg__PointCloud__Sequence__are_equal(
        lhs: *const PointCloudSeqRaw,
        rhs: *const PointCloudSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointCloud(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct PointCloud {
    pub header: std_msgs::msg::Header,
    pub points: geometry_msgs::msg::Point32Seq<0>,
    pub channels: ChannelFloat32Seq<0>,
}

impl PointCloud {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__PointCloud__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for PointCloud {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__PointCloud__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct PointCloudSeqRaw {
    data: *mut PointCloud,
    size: usize,
    capacity: usize,
}

/// Sequence of PointCloud.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PointCloudSeq<const N: usize> {
    data: *mut PointCloud,
    size: usize,
    capacity: usize,
}

impl<const N: usize> PointCloudSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PointCloudSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__PointCloud__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[PointCloud] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [PointCloud] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, PointCloud> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, PointCloud> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for PointCloudSeq<N> {
    fn drop(&mut self) {
        let mut msg = PointCloudSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { sensor_msgs__msg__PointCloud__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PointCloudSeq<N> {}
unsafe impl<const N: usize> Sync for PointCloudSeq<N> {}

impl TopicMsg for PointCloud {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointCloud()
        }
    }
}

impl PartialEq for PointCloud {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sensor_msgs__msg__PointCloud__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for PointCloudSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = PointCloudSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = PointCloudSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            sensor_msgs__msg__PointCloud__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
