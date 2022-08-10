// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__Image__init(msg: *mut Image) -> bool;
    fn sensor_msgs__msg__Image__fini(msg: *mut Image);
    fn sensor_msgs__msg__Image__are_equal(lhs: *const Image, rhs: *const Image) -> bool;
    fn sensor_msgs__msg__Image__Sequence__init(msg: *mut ImageSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__Image__Sequence__fini(msg: *mut ImageSeqRaw);
    fn sensor_msgs__msg__Image__Sequence__are_equal(lhs: *const ImageSeqRaw, rhs: *const ImageSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Image() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Image {
    pub header: std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub encoding: crate::msg::RosString<0>,
    pub is_bigendian: u8,
    pub step: u32,
    pub data: crate::msg::U8Seq<0>,
}

impl Image {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Image__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__Image__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ImageSeqRaw {
    data: *mut Image,
    size: usize,
    capacity: usize,
}

/// Sequence of Image.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ImageSeq<const N: usize> {
    data: *mut Image,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ImageSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ImageSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Image__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[Image] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Image] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }
}

impl<const N: usize> Drop for ImageSeq<N> {
    fn drop(&mut self) {
        let mut msg = ImageSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__Image__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ImageSeq<N> {}
unsafe impl<const N: usize> Sync for ImageSeq<N> {}


impl TopicMsg for Image {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Image()
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sensor_msgs__msg__Image__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for ImageSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = ImageSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = ImageSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            sensor_msgs__msg__Image__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

