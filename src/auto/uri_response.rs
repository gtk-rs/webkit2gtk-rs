// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_sys;

glib_wrapper! {
    pub struct URIResponse(Object<webkit2_sys::WebKitURIResponse, webkit2_sys::WebKitURIResponseClass, URIResponseClass>);

    match fn {
        get_type => || webkit2_sys::webkit_uri_response_get_type(),
    }
}

pub const NONE_URI_RESPONSE: Option<&URIResponse> = None;

pub trait URIResponseExt: 'static {
    fn get_content_length(&self) -> u64;

    //#[cfg(any(feature = "v2_6", feature = "dox"))]
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders>;

    fn get_mime_type(&self) -> Option<GString>;

    fn get_status_code(&self) -> u32;

    fn get_suggested_filename(&self) -> Option<GString>;

    fn get_uri(&self) -> Option<GString>;

    fn connect_property_content_length_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn connect_property_http_headers_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_mime_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_status_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_suggested_filename_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<URIResponse>> URIResponseExt for O {
    fn get_content_length(&self) -> u64 {
        unsafe {
            webkit2_sys::webkit_uri_response_get_content_length(self.as_ref().to_glib_none().0)
        }
    }

    //#[cfg(any(feature = "v2_6", feature = "dox"))]
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders> {
    //    unsafe { TODO: call webkit2_sys:webkit_uri_response_get_http_headers() }
    //}

    fn get_mime_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_uri_response_get_mime_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_status_code(&self) -> u32 {
        unsafe { webkit2_sys::webkit_uri_response_get_status_code(self.as_ref().to_glib_none().0) }
    }

    fn get_suggested_filename(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_uri_response_get_suggested_filename(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_uri_response_get_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_property_content_length_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitURIResponse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<URIResponse>,
        {
            let f: &F = &*(f as *const F);
            f(&URIResponse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content-length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    fn connect_property_http_headers_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_http_headers_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitURIResponse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<URIResponse>,
        {
            let f: &F = &*(f as *const F);
            f(&URIResponse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::http-headers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_http_headers_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mime_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mime_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitURIResponse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<URIResponse>,
        {
            let f: &F = &*(f as *const F);
            f(&URIResponse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mime-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mime_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_status_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_status_code_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitURIResponse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<URIResponse>,
        {
            let f: &F = &*(f as *const F);
            f(&URIResponse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::status-code\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_status_code_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_suggested_filename_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_suggested_filename_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitURIResponse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<URIResponse>,
        {
            let f: &F = &*(f as *const F);
            f(&URIResponse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::suggested-filename\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_suggested_filename_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitURIResponse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<URIResponse>,
        {
            let f: &F = &*(f as *const F);
            f(&URIResponse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_uri_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for URIResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "URIResponse")
    }
}
