// This file was generated by gir (0d368d6) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct FormSubmissionRequest(Object<ffi::WebKitFormSubmissionRequest>);

    match fn {
        get_type => || ffi::webkit_form_submission_request_get_type(),
    }
}

impl FormSubmissionRequest {
    //pub fn get_text_fields(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 } {
    //    unsafe { TODO: call ffi::webkit_form_submission_request_get_text_fields() }
    //}

    pub fn submit(&self) {
        unsafe {
            ffi::webkit_form_submission_request_submit(self.to_glib_none().0);
        }
    }
}
