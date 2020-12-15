// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use super::fields::{derive_downgrade_fields, DowngradeStructParts};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Generics, Ident};

/// This function derives a weak type for a given strong struct and
/// implementations of `Downgrade` and `Upgrade` traits.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(glib::Downgrade)]
/// struct Unnamed(X, Y);
///
/// #[derive(glib::Downgrade)]
/// struct Named {
///     x: X,
///     y: Y,
/// }
/// ```
///
/// Here is what will be derived:
///
/// ```rust,ignore
/// pub struct UnnamedWeak(<X as Downgrade>::Weak, <Y as Downgrade>::Weak);
///
/// impl glib::clone::Downgrade for Unnamed {
///     type Weak = UnnamedWeak;
///
///     fn downgrade(&self) -> Self::Weak {
///         let Self (ref _0, ref _1) = self;
///         UnnamedWeak (
///             glib::clone::Downgrade::downgrade(_0),
///             glib::clone::Downgrade::downgrade(_1),
///         )
///     }
/// }
///
/// impl glib::clone::Upgrade for UnnamedWeak {
///     type Strong = Unnamed;
///
///     fn upgrade(&self) -> Option<Self::Strong> {
///         let Self (ref _0, ref _1) = self;
///         Some(Unnamed (
///             glib::clone::Upgrade::upgrade(_0)?,
///             glib::clone::Upgrade::upgrade(_1)?,
///         ))
///     }
/// }
///
/// pub struct NamedWeak {
///     x: <X as Downgrade>::Weak,
///     y: <Y as Downgrade>::Weak,
/// }
///
/// impl glib::clone::Downgrade for Named {
///     type Weak = NamedWeak;
///
///     fn downgrade(&self) -> Self::Weak {
///         let Self { ref x, ref y } = self;
///         NamedWeak {
///             glib::clone::Downgrade::downgrade(x),
///             glib::clone::Downgrade::downgrade(y),
///         }
///     }
/// }
///
/// impl glib::clone::Upgrade for NamedWeak {
///     type Strong = Named;
///
///     fn upgrade(&self) -> Option<Self::Strong> {
///         let Self { ref x, ref y } = self;
///         Some(Named {
///             glib::clone::Upgrade::upgrade(x)?,
///             glib::clone::Upgrade::upgrade(y)?,
///         })
///     }
/// }
/// ```
pub fn derive_downgrade_for_struct(
    ident: Ident,
    generics: Generics,
    data_struct: syn::DataStruct,
) -> TokenStream {
    let weak_type = format_ident!("{}Weak", ident);

    let DowngradeStructParts {
        weak_fields,
        end_of_struct,
        destruct,
        downgrade,
        upgrade,
    } = derive_downgrade_fields(data_struct.fields);

    let derived = quote! {
        pub struct #weak_type #generics #weak_fields #end_of_struct

        impl #generics glib::clone::Downgrade for #ident #generics {
            type Weak = #weak_type #generics;

            fn downgrade(&self) -> Self::Weak {
                let Self #destruct = self;
                #weak_type #downgrade
            }
        }

        impl #generics glib::clone::Upgrade for #weak_type #generics {
            type Strong = #ident #generics;

            fn upgrade(&self) -> Option<Self::Strong> {
                let Self #destruct = self;
                Some(#ident #upgrade)
            }
        }
    };

    derived.into()
}