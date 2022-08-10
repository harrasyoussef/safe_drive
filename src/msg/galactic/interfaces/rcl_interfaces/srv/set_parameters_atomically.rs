// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn rcl_interfaces__srv__SetParametersAtomically_Request__init(msg: *mut SetParametersAtomicallyRequest) -> bool;
    fn rcl_interfaces__srv__SetParametersAtomically_Request__fini(msg: *mut SetParametersAtomicallyRequest);
    fn rcl_interfaces__srv__SetParametersAtomically_Request__Sequence__init(msg: *mut SetParametersAtomicallyRequestSeqRaw, size: usize) -> bool;
    fn rcl_interfaces__srv__SetParametersAtomically_Request__Sequence__fini(msg: *mut SetParametersAtomicallyRequestSeqRaw);
    fn rcl_interfaces__srv__SetParametersAtomically_Response__init(msg: *mut SetParametersAtomicallyResponse) -> bool;
    fn rcl_interfaces__srv__SetParametersAtomically_Response__fini(msg: *mut SetParametersAtomicallyResponse);
    fn rcl_interfaces__srv__SetParametersAtomically_Response__Sequence__init(msg: *mut SetParametersAtomicallyResponseSeqRaw, size: usize) -> bool;
    fn rcl_interfaces__srv__SetParametersAtomically_Response__Sequence__fini(msg: *mut SetParametersAtomicallyResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetParametersAtomically() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct SetParametersAtomicallyRequest {
    pub parameters: ParameterSeq<0>,
}

#[repr(C)]
#[derive(Debug)]
pub struct SetParametersAtomicallyResponse {
    pub result: SetParametersResult,
}

impl SetParametersAtomicallyRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__SetParametersAtomically_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetParametersAtomicallyRequest {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__srv__SetParametersAtomically_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetParametersAtomicallyRequestSeqRaw {
    data: *mut SetParametersAtomicallyRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of SetParametersAtomicallyRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetParametersAtomicallyRequestSeq<const N: usize> {
    data: *mut SetParametersAtomicallyRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SetParametersAtomicallyRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetParametersAtomicallyRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__SetParametersAtomically_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[SetParametersAtomicallyRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetParametersAtomicallyRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }
}

impl<const N: usize> Drop for SetParametersAtomicallyRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetParametersAtomicallyRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { rcl_interfaces__srv__SetParametersAtomically_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetParametersAtomicallyRequestSeq<N> {}
unsafe impl<const N: usize> Sync for SetParametersAtomicallyRequestSeq<N> {}


impl SetParametersAtomicallyResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__SetParametersAtomically_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetParametersAtomicallyResponse {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__srv__SetParametersAtomically_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetParametersAtomicallyResponseSeqRaw {
    data: *mut SetParametersAtomicallyResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of SetParametersAtomicallyResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetParametersAtomicallyResponseSeq<const N: usize> {
    data: *mut SetParametersAtomicallyResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SetParametersAtomicallyResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetParametersAtomicallyResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__SetParametersAtomically_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[SetParametersAtomicallyResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetParametersAtomicallyResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }
}

impl<const N: usize> Drop for SetParametersAtomicallyResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetParametersAtomicallyResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { rcl_interfaces__srv__SetParametersAtomically_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetParametersAtomicallyResponseSeq<N> {}
unsafe impl<const N: usize> Sync for SetParametersAtomicallyResponseSeq<N> {}


pub struct SetParametersAtomically;

impl ServiceMsg for SetParametersAtomically {
    type Request = SetParametersAtomicallyRequest;
    type Response = SetParametersAtomicallyResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetParametersAtomically()
        }
    }
}

