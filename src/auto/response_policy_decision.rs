// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use PolicyDecision;
use URIRequest;
use URIResponse;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ResponsePolicyDecision(Object<ffi::WebKitResponsePolicyDecision, ffi::WebKitResponsePolicyDecisionClass>): PolicyDecision;

    match fn {
        get_type => || ffi::webkit_response_policy_decision_get_type(),
    }
}

pub trait ResponsePolicyDecisionExt {
    fn get_request(&self) -> Option<URIRequest>;

    fn get_response(&self) -> Option<URIResponse>;

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn is_mime_type_supported(&self) -> bool;

    fn connect_property_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ResponsePolicyDecision> + IsA<glib::object::Object>> ResponsePolicyDecisionExt for O {
    fn get_request(&self) -> Option<URIRequest> {
        unsafe {
            from_glib_none(ffi::webkit_response_policy_decision_get_request(self.to_glib_none().0))
        }
    }

    fn get_response(&self) -> Option<URIResponse> {
        unsafe {
            from_glib_none(ffi::webkit_response_policy_decision_get_response(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn is_mime_type_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_response_policy_decision_is_mime_type_supported(self.to_glib_none().0))
        }
    }

    fn connect_property_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::request",
                transmute(notify_request_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::response",
                transmute(notify_response_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_request_trampoline<P>(this: *mut ffi::WebKitResponsePolicyDecision, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ResponsePolicyDecision> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ResponsePolicyDecision::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_response_trampoline<P>(this: *mut ffi::WebKitResponsePolicyDecision, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ResponsePolicyDecision> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ResponsePolicyDecision::from_glib_borrow(this).downcast_unchecked())
}
