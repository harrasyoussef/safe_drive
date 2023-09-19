// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use safe_drive::msg::common_interfaces::*;
use safe_drive::msg::*;
use safe_drive::rcl;

extern "C" {
    fn example_msg__msg__Foo__init(msg: *mut Foo) -> bool;
    fn example_msg__msg__Foo__fini(msg: *mut Foo);
    fn example_msg__msg__Foo__are_equal(lhs: *const Foo, rhs: *const Foo) -> bool;
    fn example_msg__msg__Foo__Sequence__init(msg: *mut FooSeqRaw, size: usize) -> bool;
    fn example_msg__msg__Foo__Sequence__fini(msg: *mut FooSeqRaw);
    fn example_msg__msg__Foo__Sequence__are_equal(
        lhs: *const FooSeqRaw,
        rhs: *const FooSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__msg__Foo(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct Foo {
    pub a: safe_drive::msg::RosString<0>,
}

impl Foo {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__msg__Foo__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe { example_msg__msg__Foo__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct FooSeqRaw {
    data: *mut Foo,
    size: usize,
    capacity: usize,
}

/// Sequence of Foo.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct FooSeq<const N: usize> {
    data: *mut Foo,
    size: usize,
    capacity: usize,
}

impl<const N: usize> FooSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: FooSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__msg__Foo__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: FooSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[Foo] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Foo] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Foo> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Foo> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for FooSeq<N> {
    fn drop(&mut self) {
        let mut msg = FooSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__msg__Foo__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for FooSeq<N> {}
unsafe impl<const N: usize> Sync for FooSeq<N> {}

impl TypeSupport for Foo {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe { rosidl_typesupport_c__get_message_type_support_handle__example_msg__msg__Foo() }
    }
}

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__msg__Foo__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for FooSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = FooSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = FooSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__msg__Foo__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
