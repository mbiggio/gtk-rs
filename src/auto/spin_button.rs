// This file was generated by gir (b03ccb5) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Editable;
use Entry;
use Orientable;
use SpinButtonUpdatePolicy;
use SpinType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct SpinButton(Object<ffi::GtkSpinButton>): Widget, Entry, Buildable, Editable, Orientable;

    match fn {
        get_type => || ffi::gtk_spin_button_get_type(),
    }
}

impl SpinButton {
    pub fn new(adjustment: Option<&Adjustment>, climb_rate: f64, digits: u32) -> SpinButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_spin_button_new(adjustment.to_glib_none().0, climb_rate, digits)).downcast_unchecked()
        }
    }

    pub fn new_with_range(min: f64, max: f64, step: f64) -> SpinButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_spin_button_new_with_range(min, max, step)).downcast_unchecked()
        }
    }

    pub fn configure(&self, adjustment: Option<&Adjustment>, climb_rate: f64, digits: u32) {
        unsafe {
            ffi::gtk_spin_button_configure(self.to_glib_none().0, adjustment.to_glib_none().0, climb_rate, digits);
        }
    }

    pub fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_spin_button_get_adjustment(self.to_glib_none().0))
        }
    }

    pub fn get_digits(&self) -> u32 {
        unsafe {
            ffi::gtk_spin_button_get_digits(self.to_glib_none().0)
        }
    }

    pub fn get_increments(&self) -> (f64, f64) {
        unsafe {
            let mut step = mem::uninitialized();
            let mut page = mem::uninitialized();
            ffi::gtk_spin_button_get_increments(self.to_glib_none().0, &mut step, &mut page);
            (step, page)
        }
    }

    pub fn get_numeric(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_numeric(self.to_glib_none().0))
        }
    }

    pub fn get_range(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::uninitialized();
            let mut max = mem::uninitialized();
            ffi::gtk_spin_button_get_range(self.to_glib_none().0, &mut min, &mut max);
            (min, max)
        }
    }

    pub fn get_snap_to_ticks(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_snap_to_ticks(self.to_glib_none().0))
        }
    }

    pub fn get_update_policy(&self) -> SpinButtonUpdatePolicy {
        unsafe {
            ffi::gtk_spin_button_get_update_policy(self.to_glib_none().0)
        }
    }

    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_spin_button_get_value(self.to_glib_none().0)
        }
    }

    pub fn get_value_as_int(&self) -> i32 {
        unsafe {
            ffi::gtk_spin_button_get_value_as_int(self.to_glib_none().0)
        }
    }

    pub fn get_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_wrap(self.to_glib_none().0))
        }
    }

    pub fn set_adjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_spin_button_set_adjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    pub fn set_digits(&self, digits: u32) {
        unsafe {
            ffi::gtk_spin_button_set_digits(self.to_glib_none().0, digits);
        }
    }

    pub fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            ffi::gtk_spin_button_set_increments(self.to_glib_none().0, step, page);
        }
    }

    pub fn set_numeric(&self, numeric: bool) {
        unsafe {
            ffi::gtk_spin_button_set_numeric(self.to_glib_none().0, numeric.to_glib());
        }
    }

    pub fn set_range(&self, min: f64, max: f64) {
        unsafe {
            ffi::gtk_spin_button_set_range(self.to_glib_none().0, min, max);
        }
    }

    pub fn set_snap_to_ticks(&self, snap_to_ticks: bool) {
        unsafe {
            ffi::gtk_spin_button_set_snap_to_ticks(self.to_glib_none().0, snap_to_ticks.to_glib());
        }
    }

    pub fn set_update_policy(&self, policy: SpinButtonUpdatePolicy) {
        unsafe {
            ffi::gtk_spin_button_set_update_policy(self.to_glib_none().0, policy);
        }
    }

    pub fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_spin_button_set_value(self.to_glib_none().0, value);
        }
    }

    pub fn set_wrap(&self, wrap: bool) {
        unsafe {
            ffi::gtk_spin_button_set_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    pub fn spin(&self, direction: SpinType, increment: f64) {
        unsafe {
            ffi::gtk_spin_button_spin(self.to_glib_none().0, direction, increment);
        }
    }

    pub fn update(&self) {
        unsafe {
            ffi::gtk_spin_button_update(self.to_glib_none().0);
        }
    }

}
