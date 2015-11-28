// This file was generated by gir (b03ccb5) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use SensitivityType;
use TreeModel;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ComboBox(Object<ffi::GtkComboBox>): Widget, Container, Bin, Buildable;

    match fn {
        get_type => || ffi::gtk_combo_box_get_type(),
    }
}

impl ComboBox {
    pub fn new() -> ComboBox {
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_area<T: Upcast</*Ignored*/CellArea>>(area: &T) -> ComboBox {
    //    unsafe { TODO: call ffi::gtk_combo_box_new_with_area() }
    //}

    //pub fn new_with_area_and_entry<T: Upcast</*Ignored*/CellArea>>(area: &T) -> ComboBox {
    //    unsafe { TODO: call ffi::gtk_combo_box_new_with_area_and_entry() }
    //}

    pub fn new_with_entry() -> ComboBox {
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_entry()).downcast_unchecked()
        }
    }

    pub fn new_with_model<T: Upcast<TreeModel>>(model: &T) -> ComboBox {
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_model_and_entry<T: Upcast<TreeModel>>(model: &T) -> ComboBox {
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model_and_entry(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_active(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_active(self.to_glib_none().0)
        }
    }

    pub fn get_active_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_active_id(self.to_glib_none().0))
        }
    }

    //pub fn get_active_iter(&self, iter: /*Ignored*/TreeIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_combo_box_get_active_iter() }
    //}

    pub fn get_add_tearoffs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_add_tearoffs(self.to_glib_none().0))
        }
    }

    pub fn get_button_sensitivity(&self) -> SensitivityType {
        unsafe {
            ffi::gtk_combo_box_get_button_sensitivity(self.to_glib_none().0)
        }
    }

    pub fn get_column_span_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_column_span_column(self.to_glib_none().0)
        }
    }

    pub fn get_entry_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_entry_text_column(self.to_glib_none().0)
        }
    }

    pub fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_focus_on_click(self.to_glib_none().0))
        }
    }

    pub fn get_has_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_has_entry(self.to_glib_none().0))
        }
    }

    pub fn get_id_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_id_column(self.to_glib_none().0)
        }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_model(self.to_glib_none().0))
        }
    }

    //pub fn get_popup_accessible(&self) -> /*Ignored*/Option<atk::Object> {
    //    unsafe { TODO: call ffi::gtk_combo_box_get_popup_accessible() }
    //}

    pub fn get_popup_fixed_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_popup_fixed_width(self.to_glib_none().0))
        }
    }

    //pub fn get_row_separator_func(&self) -> /*Unknown conversion*/Unknown rust type: "TreeViewRowSeparatorFunc" {
    //    unsafe { TODO: call ffi::gtk_combo_box_get_row_separator_func() }
    //}

    pub fn get_row_span_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_row_span_column(self.to_glib_none().0)
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_wrap_width(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_wrap_width(self.to_glib_none().0)
        }
    }

    pub fn popdown(&self) {
        unsafe {
            ffi::gtk_combo_box_popdown(self.to_glib_none().0);
        }
    }

    pub fn popup(&self) {
        unsafe {
            ffi::gtk_combo_box_popup(self.to_glib_none().0);
        }
    }

    //pub fn popup_for_device(&self, device: /*Ignored*/&gdk::Device) {
    //    unsafe { TODO: call ffi::gtk_combo_box_popup_for_device() }
    //}

    pub fn set_active(&self, index_: i32) {
        unsafe {
            ffi::gtk_combo_box_set_active(self.to_glib_none().0, index_);
        }
    }

    pub fn set_active_id(&self, active_id: Option<&str>) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_set_active_id(self.to_glib_none().0, active_id.to_glib_none().0))
        }
    }

    //pub fn set_active_iter(&self, iter: /*Ignored*/Option<&TreeIter>) {
    //    unsafe { TODO: call ffi::gtk_combo_box_set_active_iter() }
    //}

    pub fn set_add_tearoffs(&self, add_tearoffs: bool) {
        unsafe {
            ffi::gtk_combo_box_set_add_tearoffs(self.to_glib_none().0, add_tearoffs.to_glib());
        }
    }

    pub fn set_button_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_combo_box_set_button_sensitivity(self.to_glib_none().0, sensitivity);
        }
    }

    pub fn set_column_span_column(&self, column_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_column_span_column(self.to_glib_none().0, column_span);
        }
    }

    pub fn set_entry_text_column(&self, text_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_entry_text_column(self.to_glib_none().0, text_column);
        }
    }

    pub fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_combo_box_set_focus_on_click(self.to_glib_none().0, focus_on_click.to_glib());
        }
    }

    pub fn set_id_column(&self, id_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_id_column(self.to_glib_none().0, id_column);
        }
    }

    pub fn set_model<T: Upcast<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_combo_box_set_model(self.to_glib_none().0, model.to_glib_none().0);
        }
    }

    pub fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe {
            ffi::gtk_combo_box_set_popup_fixed_width(self.to_glib_none().0, fixed.to_glib());
        }
    }

    //pub fn set_row_separator_func(&self, func: /*Unknown conversion*/Unknown rust type: "TreeViewRowSeparatorFunc", data: Option<Fundamental: Pointer>, destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_combo_box_set_row_separator_func() }
    //}

    pub fn set_row_span_column(&self, row_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_row_span_column(self.to_glib_none().0, row_span);
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_combo_box_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_wrap_width(&self, width: i32) {
        unsafe {
            ffi::gtk_combo_box_set_wrap_width(self.to_glib_none().0, width);
        }
    }

}
