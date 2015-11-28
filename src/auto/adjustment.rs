// This file was generated by gir (b03ccb5) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct Adjustment(Object<ffi::GtkAdjustment>);

    match fn {
        get_type => || ffi::gtk_adjustment_get_type(),
    }
}

impl Adjustment {
    pub fn new(value: f64, lower: f64, upper: f64, step_increment: f64, page_increment: f64, page_size: f64) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_adjustment_new(value, lower, upper, step_increment, page_increment, page_size))
        }
    }

    pub fn changed(&self) {
        unsafe {
            ffi::gtk_adjustment_changed(self.to_glib_none().0);
        }
    }

    pub fn clamp_page(&self, lower: f64, upper: f64) {
        unsafe {
            ffi::gtk_adjustment_clamp_page(self.to_glib_none().0, lower, upper);
        }
    }

    pub fn configure(&self, value: f64, lower: f64, upper: f64, step_increment: f64, page_increment: f64, page_size: f64) {
        unsafe {
            ffi::gtk_adjustment_configure(self.to_glib_none().0, value, lower, upper, step_increment, page_increment, page_size);
        }
    }

    pub fn get_lower(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_lower(self.to_glib_none().0)
        }
    }

    pub fn get_minimum_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_minimum_increment(self.to_glib_none().0)
        }
    }

    pub fn get_page_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_increment(self.to_glib_none().0)
        }
    }

    pub fn get_page_size(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_size(self.to_glib_none().0)
        }
    }

    pub fn get_step_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_step_increment(self.to_glib_none().0)
        }
    }

    pub fn get_upper(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_upper(self.to_glib_none().0)
        }
    }

    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_value(self.to_glib_none().0)
        }
    }

    pub fn set_lower(&self, lower: f64) {
        unsafe {
            ffi::gtk_adjustment_set_lower(self.to_glib_none().0, lower);
        }
    }

    pub fn set_page_increment(&self, page_increment: f64) {
        unsafe {
            ffi::gtk_adjustment_set_page_increment(self.to_glib_none().0, page_increment);
        }
    }

    pub fn set_page_size(&self, page_size: f64) {
        unsafe {
            ffi::gtk_adjustment_set_page_size(self.to_glib_none().0, page_size);
        }
    }

    pub fn set_step_increment(&self, step_increment: f64) {
        unsafe {
            ffi::gtk_adjustment_set_step_increment(self.to_glib_none().0, step_increment);
        }
    }

    pub fn set_upper(&self, upper: f64) {
        unsafe {
            ffi::gtk_adjustment_set_upper(self.to_glib_none().0, upper);
        }
    }

    pub fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_adjustment_set_value(self.to_glib_none().0, value);
        }
    }

    pub fn value_changed(&self) {
        unsafe {
            ffi::gtk_adjustment_value_changed(self.to_glib_none().0);
        }
    }

}
