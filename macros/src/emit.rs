// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use super::{Tags, find_attr};

pub fn derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let (tags, tag_ty) = match Tags::new(&s) {
        Ok(v) => v,
        Err(err) => return err.into_compile_error(),
    };

    let se = quote::quote!(radiation);
    let mut body = quote::quote!();
    for t in tags {
        let (tag_val, variant) = match t {
            Ok(v) => v,
            Err(err) => return err.to_compile_error(),
        };

        let init = quote::quote! { buffer = #tag_ty::emit(&(#tag_val), buffer); };
        body.extend(variant.fold(init, |acc, binding| {
            let ast = &binding.ast();
            let as_str = find_attr(&ast.attrs, "as_str").is_some();
            let custom_emit = extract_attr!(&ast.attrs, "custom_emit");

            let i = &binding.binding;

            if as_str {
                quote::quote! {
                    #acc
                    buffer = #i.to_string().emit(buffer);
                }
            } else if let Some(custom_emit) = custom_emit {
                quote::quote! {
                    #acc
                    buffer = #custom_emit(#i, buffer);
                }
            } else {
                quote::quote! {
                    #acc
                    buffer = #i.emit(buffer);
                }
            }
        }))
    }

    let gen_impl = quote! {
        gen impl<W> #se::Emit<W> for @Self
        where
            W: Extend<u8> + #se::RadiationBuffer,
        {
            fn emit(&self, mut buffer: W) -> W {
                match self {
                    #body
                }
                buffer
            }
        }

    };

    s.gen_impl(gen_impl)
}
