// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__CompressedImage__init(msg: *mut CompressedImage) -> bool;
    fn sensor_msgs__msg__CompressedImage__fini(msg: *mut CompressedImage);
    fn sensor_msgs__msg__CompressedImage__are_equal(
        lhs: *const CompressedImage,
        rhs: *const CompressedImage,
    ) -> bool;
    fn sensor_msgs__msg__CompressedImage__Sequence__init(
        msg: *mut CompressedImageSeqRaw,
        size: usize,
    ) -> bool;
    fn sensor_msgs__msg__CompressedImage__Sequence__fini(msg: *mut CompressedImageSeqRaw);
    fn sensor_msgs__msg__CompressedImage__Sequence__are_equal(
        lhs: *const CompressedImageSeqRaw,
        rhs: *const CompressedImageSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__CompressedImage(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct CompressedImage {
    pub header: std_msgs::msg::Header,
    pub format: crate::msg::RosString<0>,
    pub data: crate::msg::U8Seq<0>,
}

impl CompressedImage {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__CompressedImage__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for CompressedImage {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__CompressedImage__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct CompressedImageSeqRaw {
    data: *mut CompressedImage,
    size: usize,
    capacity: usize,
}

/// Sequence of CompressedImage.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct CompressedImageSeq<const N: usize> {
    data: *mut CompressedImage,
    size: usize,
    capacity: usize,
}

impl<const N: usize> CompressedImageSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: CompressedImageSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__CompressedImage__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[CompressedImage] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [CompressedImage] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, CompressedImage> {
        self.as_slice().iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, CompressedImage> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<const N: usize> Drop for CompressedImageSeq<N> {
    fn drop(&mut self) {
        let mut msg = CompressedImageSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { sensor_msgs__msg__CompressedImage__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for CompressedImageSeq<N> {}
unsafe impl<const N: usize> Sync for CompressedImageSeq<N> {}

impl TopicMsg for CompressedImage {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__CompressedImage(
            )
        }
    }
}

impl PartialEq for CompressedImage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sensor_msgs__msg__CompressedImage__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for CompressedImageSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = CompressedImageSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = CompressedImageSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            sensor_msgs__msg__CompressedImage__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
