#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(deref_nullptr)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::too_many_arguments)]

mod galactic;

pub(crate) use galactic::*;
pub use galactic::{rosidl_message_type_support_t, rosidl_service_type_support_t};

use crate::error::{ret_val_to_err, RCLResult};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub(crate) static MT_UNSAFE_FN: Lazy<Mutex<MTUnsafeFn>> =
    Lazy::new(|| Mutex::new(MTUnsafeFn::new()));

pub(crate) static MT_UNSAFE_LOG_FN: Lazy<Mutex<MTUnsafeLogFn>> =
    Lazy::new(|| Mutex::new(MTUnsafeLogFn::new()));

pub(crate) struct MTUnsafeFn;
pub(crate) struct MTUnsafeLogFn;

impl MTUnsafeFn {
    fn new() -> Self {
        Self
    }

    pub fn rcl_init(
        &self,
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
        options: *const rcl_init_options_t,
        context: *mut rcl_context_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_init(argc, argv, options, context) })
    }

    pub fn rcl_context_fini(&self, context: *mut rcl_context_t) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_context_fini(context) })
    }

    pub fn rcl_init_options_init(
        &self,
        init_options: *mut rcl_init_options_t,
        allocator: rcl_allocator_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_init_options_init(init_options, allocator) })
    }

    pub fn rcl_init_options_fini(&self, init_options: *mut rcl_init_options_t) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_init_options_fini(init_options) })
    }

    pub fn rcl_node_init(
        &self,
        node: *mut rcl_node_t,
        name: *const ::std::os::raw::c_char,
        namespace_: *const ::std::os::raw::c_char,
        context: *mut rcl_context_t,
        options: *const rcl_node_options_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_node_init(node, name, namespace_, context, options) })
    }

    pub fn rcl_node_fini(&self, node: *mut rcl_node_t) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_node_fini(node) })
    }

    pub fn rcl_node_options_fini(&self, options: *mut rcl_node_options_t) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_node_options_fini(options) })
    }

    pub fn rcl_publisher_init(
        &self,
        publisher: *mut rcl_publisher_t,
        node: *const rcl_node_t,
        type_support: *const rosidl_message_type_support_t,
        topic_name: *const ::std::os::raw::c_char,
        options: *const rcl_publisher_options_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_publisher_init(publisher, node, type_support, topic_name, options)
        })
    }

    pub fn rcl_publisher_fini(
        &self,
        publisher: *mut rcl_publisher_t,
        node: *mut rcl_node_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_publisher_fini(publisher, node) })
    }

    pub fn rcl_subscription_fini(
        &self,
        subscription: *mut rcl_subscription_t,
        node: *mut rcl_node_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_subscription_fini(subscription, node) })
    }

    pub fn rcl_subscription_init(
        &self,
        subscription: *mut rcl_subscription_t,
        node: *const rcl_node_t,
        type_support: *const rosidl_message_type_support_t,
        topic_name: *const ::std::os::raw::c_char,
        options: *const rcl_subscription_options_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_subscription_init(subscription, node, type_support, topic_name, options)
        })
    }

    pub fn rcl_take(
        &self,
        subscription: *const rcl_subscription_t,
        ros_message: *mut ::std::os::raw::c_void,
        message_info: *mut rmw_message_info_t,
        allocation: *mut rmw_subscription_allocation_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_take(subscription, ros_message, message_info, allocation)
        })
    }

    pub fn rcl_wait_set_init(
        &self,
        wait_set: *mut rcl_wait_set_t,
        number_of_subscriptions: size_t,
        number_of_guard_conditions: size_t,
        number_of_timers: size_t,
        number_of_clients: size_t,
        number_of_services: size_t,
        number_of_events: size_t,
        context: *mut rcl_context_t,
        allocator: rcl_allocator_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_wait_set_init(
                wait_set,
                number_of_subscriptions,
                number_of_guard_conditions,
                number_of_timers,
                number_of_clients,
                number_of_services,
                number_of_events,
                context,
                allocator,
            )
        })
    }

    pub fn rcl_wait_set_clear(&self, wait_set: *mut rcl_wait_set_t) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_wait_set_clear(wait_set) })
    }

    pub fn rcl_wait_set_resize(
        &self,
        wait_set: *mut rcl_wait_set_t,
        subscriptions_size: size_t,
        guard_conditions_size: size_t,
        timers_size: size_t,
        clients_size: size_t,
        services_size: size_t,
        events_size: size_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_wait_set_resize(
                wait_set,
                subscriptions_size,
                guard_conditions_size,
                timers_size,
                clients_size,
                services_size,
                events_size,
            )
        })
    }

    pub fn rcl_wait_set_add_subscription(
        &self,
        wait_set: *mut rcl_wait_set_t,
        subscription: *const rcl_subscription_t,
        index: *mut size_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_wait_set_add_subscription(wait_set, subscription, index)
        })
    }

    pub fn rcl_wait_set_fini(&self, wait_set: *mut rcl_wait_set_t) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_wait_set_fini(wait_set) })
    }

    pub fn rcl_guard_condition_init(
        &self,
        guard_condition: *mut rcl_guard_condition_t,
        context: *mut rcl_context_t,
        options: rcl_guard_condition_options_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_guard_condition_init(guard_condition, context, options) })
    }

    pub fn rcl_trigger_guard_condition(
        &self,
        guard_condition: *mut rcl_guard_condition_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_trigger_guard_condition(guard_condition) })
    }

    pub fn rcl_guard_condition_fini(
        &self,
        guard_condition: *mut rcl_guard_condition_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_guard_condition_fini(guard_condition) })
    }

    pub fn rcl_wait_set_add_guard_condition(
        &self,
        wait_set: *mut rcl_wait_set_t,
        guard_condition: *const rcl_guard_condition_t,
        index: *mut size_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_wait_set_add_guard_condition(wait_set, guard_condition, index)
        })
    }

    pub fn rcl_service_init(
        &self,
        service: *mut rcl_service_t,
        node: *const rcl_node_t,
        type_support: *const rosidl_service_type_support_t,
        service_name: *const ::std::os::raw::c_char,
        options: *const rcl_service_options_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_service_init(service, node, type_support, service_name, options)
        })
    }

    pub fn rcl_service_fini(
        &self,
        service: *mut rcl_service_t,
        node: *mut rcl_node_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_service_fini(service, node) })
    }

    pub fn rcl_take_request_with_info(
        &self,
        service: *const rcl_service_t,
        request_header: *mut rmw_service_info_t,
        ros_request: *mut ::std::os::raw::c_void,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_take_request_with_info(service, request_header, ros_request)
        })
    }

    pub fn rcl_client_init(
        &self,
        client: *mut rcl_client_t,
        node: *const rcl_node_t,
        type_support: *const rosidl_service_type_support_t,
        service_name: *const ::std::os::raw::c_char,
        options: *const rcl_client_options_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_client_init(client, node, type_support, service_name, options)
        })
    }

    pub fn rcl_client_fini(
        &self,
        client: *mut rcl_client_t,
        node: *mut rcl_node_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_client_fini(client, node) })
    }

    pub fn rcl_take_response_with_info(
        &self,
        client: *const rcl_client_t,
        request_header: *mut rmw_service_info_t,
        ros_response: *mut ::std::os::raw::c_void,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe {
            self::rcl_take_response_with_info(client, request_header, ros_response)
        })
    }

    pub fn rcl_wait_set_add_client(
        &self,
        wait_set: *mut rcl_wait_set_t,
        client: *const rcl_client_t,
        index: *mut size_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_wait_set_add_client(wait_set, client, index) })
    }

    pub fn rcl_wait_set_add_service(
        &self,
        wait_set: *mut rcl_wait_set_t,
        service: *const rcl_service_t,
        index: *mut size_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_wait_set_add_service(wait_set, service, index) })
    }
}

impl MTUnsafeLogFn {
    fn new() -> Self {
        Self
    }

    pub fn rcutils_logging_initialize(&self) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcutils_logging_initialize() })
    }

    pub fn rcutils_logging_logger_is_enabled_for(&self, name: *const i8, severity: i32) -> bool {
        unsafe { self::rcutils_logging_logger_is_enabled_for(name, severity) }
    }

    pub fn rcutils_log(
        &self,
        location: *const rcutils_log_location_t,
        severity: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        format: *const ::std::os::raw::c_char,
    ) {
        unsafe { self::rcutils_log(location, severity, name, format) }
    }
}

pub(crate) struct MTSafeFn;

impl MTSafeFn {
    pub fn rcl_get_zero_initialized_context() -> rcl_context_t {
        unsafe { self::rcl_get_zero_initialized_context() }
    }

    pub fn rcl_shutdown(context: *mut rcl_context_t) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_shutdown(context) })
    }

    pub fn rcutils_get_default_allocator() -> rcutils_allocator_t {
        unsafe { self::rcutils_get_default_allocator() }
    }

    pub fn rcl_get_zero_initialized_init_options() -> rcl_init_options_t {
        unsafe { self::rcl_get_zero_initialized_init_options() }
    }

    pub fn rcl_get_zero_initialized_node() -> rcl_node_t {
        unsafe { self::rcl_get_zero_initialized_node() }
    }

    pub fn rcl_node_get_default_options() -> rcl_node_options_t {
        unsafe { self::rcl_node_get_default_options() }
    }

    pub fn rcl_get_zero_initialized_publisher() -> rcl_publisher_t {
        unsafe { self::rcl_get_zero_initialized_publisher() }
    }

    pub fn rcl_publish(
        publisher: *const rcl_publisher_t,
        ros_message: *const ::std::os::raw::c_void,
        allocation: *mut rmw_publisher_allocation_t,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_publish(publisher, ros_message, allocation) })
    }

    pub fn rmw_get_default_publisher_options() -> rmw_publisher_options_t {
        unsafe { self::rmw_get_default_publisher_options() }
    }

    pub fn rcl_get_zero_initialized_subscription() -> rcl_subscription_t {
        unsafe { self::rcl_get_zero_initialized_subscription() }
    }

    pub fn rmw_get_default_subscription_options() -> rmw_subscription_options_t {
        unsafe { self::rmw_get_default_subscription_options() }
    }

    pub fn rcl_get_zero_initialized_wait_set() -> self::rcl_wait_set_t {
        unsafe { self::rcl_get_zero_initialized_wait_set() }
    }

    pub fn rcl_wait(wait_set: *mut rcl_wait_set_t, timeout: i64) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_wait(wait_set, timeout) })
    }

    pub fn rcl_get_zero_initialized_guard_condition() -> rcl_guard_condition_t {
        unsafe { self::rcl_get_zero_initialized_guard_condition() }
    }

    pub fn rcl_get_zero_initialized_service() -> rcl_service_t {
        unsafe { self::rcl_get_zero_initialized_service() }
    }

    pub fn rcl_send_response(
        service: *const rcl_service_t,
        response_header: *mut rmw_request_id_t,
        ros_response: *mut ::std::os::raw::c_void,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_send_response(service, response_header, ros_response) })
    }

    pub fn rcl_get_zero_initialized_client() -> rcl_client_t {
        unsafe { self::rcl_get_zero_initialized_client() }
    }

    pub fn rcl_send_request(
        client: *const rcl_client_t,
        ros_request: *const ::std::os::raw::c_void,
        sequence_number: *mut i64,
    ) -> RCLResult<()> {
        ret_val_to_err(unsafe { self::rcl_send_request(client, ros_request, sequence_number) })
    }
}
