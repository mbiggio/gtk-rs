// Take a look at the license at the top of the repository in the LICENSE file.

//! Module for registering shared types for Rust types.

use crate::translate::*;

pub trait SharedType: Clone + Sized + 'static {
    const NAME: &'static str;
    fn reference(this: *const Self) -> *const Self;
    fn unreference(this: *const Self);
}

pub trait SharedTypeExt {
    fn get_type() -> crate::Type;
}

pub fn register_shared_type<T: SharedType>() -> crate::Type {
    unsafe {
        use std::ffi::CString;
        unsafe extern "C" fn boxed_copy<T: SharedType>(
            v: ffi::gpointer,
        ) -> ffi::gpointer {
            T::reference(v as *const T) as ffi::gpointer
        }
        unsafe extern "C" fn boxed_free<T: SharedType>(v: ffi::gpointer) {
            T::unreference(v as *const T)
        }

        let type_name = CString::new(T::NAME).unwrap();
        if gobject_ffi::g_type_from_name(type_name.as_ptr()) != gobject_ffi::G_TYPE_INVALID {
            panic!(
                "Type {} has already been registered",
                type_name.to_str().unwrap()
            );
        }

        from_glib(gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(boxed_copy::<T>),
            Some(boxed_free::<T>),
        ))
    }
}

impl<T> SharedTypeExt for T
where
    T: SharedType,
{
    fn get_type() -> crate::Type {
        static mut TYPE_: crate::Type = crate::Type::Invalid;
        static ONCE: ::std::sync::Once = ::std::sync::Once::new();

        ONCE.call_once(|| {
            let type_ = register_shared_type::<T>();
            unsafe {
                TYPE_ = type_;
            }
        });

        unsafe { TYPE_ }
    }
}
