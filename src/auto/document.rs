// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Document(Interface<atk_sys::AtkDocument>);

    match fn {
        get_type => || atk_sys::atk_document_get_type(),
    }
}

pub const NONE_DOCUMENT: Option<&Document> = None;

pub trait DocumentExt: 'static {
    fn get_attribute_value(&self, attribute_name: &str) -> Option<GString>;

    //fn get_attributes(&self) -> /*Ignored*/Option<AttributeSet>;

    fn get_current_page_number(&self) -> i32;

    //fn get_document(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    fn get_document_type(&self) -> Option<GString>;

    fn get_page_count(&self) -> i32;

    fn set_attribute_value(&self, attribute_name: &str, attribute_value: &str) -> bool;

    fn connect_load_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_load_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_page_changed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Document>> DocumentExt for O {
    fn get_attribute_value(&self, attribute_name: &str) -> Option<GString> {
        unsafe {
            from_glib_none(atk_sys::atk_document_get_attribute_value(
                self.as_ref().to_glib_none().0,
                attribute_name.to_glib_none().0,
            ))
        }
    }

    //fn get_attributes(&self) -> /*Ignored*/Option<AttributeSet> {
    //    unsafe { TODO: call atk_sys:atk_document_get_attributes() }
    //}

    fn get_current_page_number(&self) -> i32 {
        unsafe { atk_sys::atk_document_get_current_page_number(self.as_ref().to_glib_none().0) }
    }

    //fn get_document(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call atk_sys:atk_document_get_document() }
    //}

    fn get_document_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(atk_sys::atk_document_get_document_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_page_count(&self) -> i32 {
        unsafe { atk_sys::atk_document_get_page_count(self.as_ref().to_glib_none().0) }
    }

    fn set_attribute_value(&self, attribute_name: &str, attribute_value: &str) -> bool {
        unsafe {
            from_glib(atk_sys::atk_document_set_attribute_value(
                self.as_ref().to_glib_none().0,
                attribute_name.to_glib_none().0,
                attribute_value.to_glib_none().0,
            ))
        }
    }

    fn connect_load_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn load_complete_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut atk_sys::AtkDocument,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Document>,
        {
            let f: &F = &*(f as *const F);
            f(&Document::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"load-complete\0".as_ptr() as *const _,
                Some(transmute(load_complete_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_load_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn load_stopped_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut atk_sys::AtkDocument,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Document>,
        {
            let f: &F = &*(f as *const F);
            f(&Document::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"load-stopped\0".as_ptr() as *const _,
                Some(transmute(load_stopped_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_page_changed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_changed_trampoline<P, F: Fn(&P, i32) + 'static>(
            this: *mut atk_sys::AtkDocument,
            page_number: libc::c_int,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Document>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Document::from_glib_borrow(this).unsafe_cast_ref(),
                page_number,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-changed\0".as_ptr() as *const _,
                Some(transmute(page_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reload<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn reload_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut atk_sys::AtkDocument,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Document>,
        {
            let f: &F = &*(f as *const F);
            f(&Document::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"reload\0".as_ptr() as *const _,
                Some(transmute(reload_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Document")
    }
}
