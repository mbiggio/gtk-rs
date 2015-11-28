// This file was generated by gir (b03ccb5) from gir-files (11e0e6d)
// DO NOT EDIT

use TextMark;
use TextTagTable;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TextBuffer(Object<ffi::GtkTextBuffer>);

    match fn {
        get_type => || ffi::gtk_text_buffer_get_type(),
    }
}

impl TextBuffer {
    pub fn new(table: Option<&TextTagTable>) -> TextBuffer {
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_new(table.to_glib_none().0))
        }
    }

    //pub fn add_mark(&self, mark: &TextMark, where_: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_add_mark() }
    //}

    //pub fn add_selection_clipboard(&self, clipboard: /*Ignored*/&Clipboard) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_add_selection_clipboard() }
    //}

    //pub fn apply_tag(&self, tag: &TextTag, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_apply_tag() }
    //}

    //pub fn apply_tag_by_name(&self, name: &str, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_apply_tag_by_name() }
    //}

    //pub fn backspace(&self, iter: /*Ignored*/&TextIter, interactive: bool, default_editable: bool) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_backspace() }
    //}

    pub fn begin_user_action(&self) {
        unsafe {
            ffi::gtk_text_buffer_begin_user_action(self.to_glib_none().0);
        }
    }

    //pub fn copy_clipboard(&self, clipboard: /*Ignored*/&Clipboard) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_copy_clipboard() }
    //}

    //pub fn create_child_anchor(&self, iter: /*Ignored*/&TextIter) -> Option<TextChildAnchor> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_create_child_anchor() }
    //}

    //pub fn create_mark(&self, mark_name: Option<&str>, where_: /*Ignored*/&TextIter, left_gravity: bool) -> Option<TextMark> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_create_mark() }
    //}

    //pub fn create_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*/Fundamental: VarArgs) -> Option<TextTag> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_create_tag() }
    //}

    //pub fn cut_clipboard(&self, clipboard: /*Ignored*/&Clipboard, default_editable: bool) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_cut_clipboard() }
    //}

    //pub fn delete(&self, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_delete() }
    //}

    //pub fn delete_interactive(&self, start_iter: /*Ignored*/&TextIter, end_iter: /*Ignored*/&TextIter, default_editable: bool) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_delete_interactive() }
    //}

    pub fn delete_mark(&self, mark: &TextMark) {
        unsafe {
            ffi::gtk_text_buffer_delete_mark(self.to_glib_none().0, mark.to_glib_none().0);
        }
    }

    pub fn delete_mark_by_name(&self, name: &str) {
        unsafe {
            ffi::gtk_text_buffer_delete_mark_by_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn delete_selection(&self, interactive: bool, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_delete_selection(self.to_glib_none().0, interactive.to_glib(), default_editable.to_glib()))
        }
    }

    //pub fn deserialize(&self, content_buffer: &TextBuffer, format: &Atom, iter: /*Ignored*/&TextIter, data: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 3 }", length: Fundamental: Size, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize() }
    //}

    //pub fn deserialize_get_can_create_tags(&self, format: &Atom) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize_get_can_create_tags() }
    //}

    //pub fn deserialize_set_can_create_tags(&self, format: &Atom, can_create_tags: bool) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize_set_can_create_tags() }
    //}

    pub fn end_user_action(&self) {
        unsafe {
            ffi::gtk_text_buffer_end_user_action(self.to_glib_none().0);
        }
    }

    //pub fn get_bounds(&self, start: /*Ignored*/TextIter, end: /*Ignored*/TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_bounds() }
    //}

    pub fn get_char_count(&self) -> i32 {
        unsafe {
            ffi::gtk_text_buffer_get_char_count(self.to_glib_none().0)
        }
    }

    //pub fn get_copy_target_list(&self) -> /*Ignored*/TargetList {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_copy_target_list() }
    //}

    //pub fn get_deserialize_formats(&self, n_formats: &mut i32) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 10, id: 5 }" {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_deserialize_formats() }
    //}

    //pub fn get_end_iter(&self, iter: /*Ignored*/TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_end_iter() }
    //}

    pub fn get_has_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_get_has_selection(self.to_glib_none().0))
        }
    }

    pub fn get_insert(&self) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_insert(self.to_glib_none().0))
        }
    }

    //pub fn get_iter_at_child_anchor(&self, iter: /*Ignored*/TextIter, anchor: &TextChildAnchor) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_iter_at_child_anchor() }
    //}

    //pub fn get_iter_at_line(&self, iter: /*Ignored*/TextIter, line_number: i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_iter_at_line() }
    //}

    //pub fn get_iter_at_line_index(&self, iter: /*Ignored*/TextIter, line_number: i32, byte_index: i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_iter_at_line_index() }
    //}

    //pub fn get_iter_at_line_offset(&self, iter: /*Ignored*/TextIter, line_number: i32, char_offset: i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_iter_at_line_offset() }
    //}

    //pub fn get_iter_at_mark(&self, iter: /*Ignored*/TextIter, mark: &TextMark) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_iter_at_mark() }
    //}

    //pub fn get_iter_at_offset(&self, iter: /*Ignored*/TextIter, char_offset: i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_iter_at_offset() }
    //}

    pub fn get_line_count(&self) -> i32 {
        unsafe {
            ffi::gtk_text_buffer_get_line_count(self.to_glib_none().0)
        }
    }

    pub fn get_mark(&self, name: &str) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_mark(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn get_modified(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_get_modified(self.to_glib_none().0))
        }
    }

    //pub fn get_paste_target_list(&self) -> /*Ignored*/TargetList {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_paste_target_list() }
    //}

    pub fn get_selection_bound(&self) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_selection_bound(self.to_glib_none().0))
        }
    }

    //pub fn get_selection_bounds(&self, start: /*Ignored*/TextIter, end: /*Ignored*/TextIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_selection_bounds() }
    //}

    //pub fn get_serialize_formats(&self, n_formats: &mut i32) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 10, id: 5 }" {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_serialize_formats() }
    //}

    //pub fn get_slice(&self, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter, include_hidden_chars: bool) -> Option<String> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_slice() }
    //}

    //pub fn get_start_iter(&self, iter: /*Ignored*/TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_start_iter() }
    //}

    pub fn get_tag_table(&self) -> Option<TextTagTable> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_tag_table(self.to_glib_none().0))
        }
    }

    //pub fn get_text(&self, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter, include_hidden_chars: bool) -> Option<String> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_text() }
    //}

    //pub fn insert(&self, iter: /*Ignored*/&TextIter, text: &str, len: i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert() }
    //}

    pub fn insert_at_cursor(&self, text: &str, len: i32) {
        unsafe {
            ffi::gtk_text_buffer_insert_at_cursor(self.to_glib_none().0, text.to_glib_none().0, len);
        }
    }

    //pub fn insert_child_anchor(&self, iter: /*Ignored*/&TextIter, anchor: &TextChildAnchor) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_child_anchor() }
    //}

    //pub fn insert_interactive(&self, iter: /*Ignored*/&TextIter, text: &str, len: i32, default_editable: bool) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_interactive() }
    //}

    pub fn insert_interactive_at_cursor(&self, text: &str, len: i32, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_insert_interactive_at_cursor(self.to_glib_none().0, text.to_glib_none().0, len, default_editable.to_glib()))
        }
    }

    //#[cfg(gtk_3_16)]
    //pub fn insert_markup(&self, iter: /*Ignored*/&TextIter, markup: &str, len: i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_markup() }
    //}

    //pub fn insert_pixbuf(&self, iter: /*Ignored*/&TextIter, pixbuf: &gdk_pixbuf::Pixbuf) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_pixbuf() }
    //}

    //pub fn insert_range(&self, iter: /*Ignored*/&TextIter, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_range() }
    //}

    //pub fn insert_range_interactive(&self, iter: /*Ignored*/&TextIter, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter, default_editable: bool) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_range_interactive() }
    //}

    //pub fn insert_with_tags(&self, iter: /*Ignored*/&TextIter, text: &str, len: i32, first_tag: &TextTag, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_with_tags() }
    //}

    //pub fn insert_with_tags_by_name(&self, iter: /*Ignored*/&TextIter, text: &str, len: i32, first_tag_name: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_with_tags_by_name() }
    //}

    //pub fn move_mark(&self, mark: &TextMark, where_: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_move_mark() }
    //}

    //pub fn move_mark_by_name(&self, name: &str, where_: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_move_mark_by_name() }
    //}

    //pub fn paste_clipboard(&self, clipboard: /*Ignored*/&Clipboard, override_location: /*Ignored*/Option<&TextIter>, default_editable: bool) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_paste_clipboard() }
    //}

    //pub fn place_cursor(&self, where_: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_place_cursor() }
    //}

    //pub fn register_deserialize_format(&self, mime_type: &str, function: /*Unknown conversion*/Unknown rust type: "TextBufferDeserializeFunc", user_data: Fundamental: Pointer, user_data_destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") -> Atom {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_deserialize_format() }
    //}

    //pub fn register_deserialize_tagset(&self, tagset_name: Option<&str>) -> Atom {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_deserialize_tagset() }
    //}

    //pub fn register_serialize_format(&self, mime_type: &str, function: /*Unknown conversion*/Unknown rust type: "TextBufferSerializeFunc", user_data: Fundamental: Pointer, user_data_destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") -> Atom {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_serialize_format() }
    //}

    //pub fn register_serialize_tagset(&self, tagset_name: Option<&str>) -> Atom {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_serialize_tagset() }
    //}

    //pub fn remove_all_tags(&self, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_remove_all_tags() }
    //}

    //pub fn remove_selection_clipboard(&self, clipboard: /*Ignored*/&Clipboard) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_remove_selection_clipboard() }
    //}

    //pub fn remove_tag(&self, tag: &TextTag, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_remove_tag() }
    //}

    //pub fn remove_tag_by_name(&self, name: &str, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_remove_tag_by_name() }
    //}

    //pub fn select_range(&self, ins: /*Ignored*/&TextIter, bound: /*Ignored*/&TextIter) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_select_range() }
    //}

    //pub fn serialize(&self, content_buffer: &TextBuffer, format: &Atom, start: /*Ignored*/&TextIter, end: /*Ignored*/&TextIter, length: Fundamental: Size) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 3 }" {
    //    unsafe { TODO: call ffi::gtk_text_buffer_serialize() }
    //}

    pub fn set_modified(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_buffer_set_modified(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_text(&self, text: &str, len: i32) {
        unsafe {
            ffi::gtk_text_buffer_set_text(self.to_glib_none().0, text.to_glib_none().0, len);
        }
    }

    //pub fn unregister_deserialize_format(&self, format: &Atom) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_unregister_deserialize_format() }
    //}

    //pub fn unregister_serialize_format(&self, format: &Atom) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_unregister_serialize_format() }
    //}

}
