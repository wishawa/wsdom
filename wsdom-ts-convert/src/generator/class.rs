use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::{
        types::SimplifiedType,
        util::{iter_dedupe_all, iter_dedupe_consecutive},
    },
    parser::{
        comment::WithComment,
        field::{Field, FieldName},
        interface::Interface,
        member::{Getter, Member, Setter},
        method::{Method, MethodName},
    },
};

use super::{
    types::known_types,
    util::{new_ident_safe, to_snake_case},
    Context,
};

impl<'a> Context<'a> {
    pub(super) fn make_class(
        &self,
        interface: &Interface<'_>,
        decl_members: &[WithComment<'_, Member<'_>>],
    ) -> TokenStream {
        let name = new_ident_safe(interface.name);

        let (
            generics_with_bound,
            generics_without_bound,
            generics_with_default,
            generics_for_phantom,
        ) = if interface.generics.args.is_empty() {
            (None, None, None, quote! {<()>})
        } else {
            let with_bounds_with_defaults = interface.generics.args.iter().map(|arg| {
                let name = new_ident_safe(arg.name);
                let bounds = arg
                    .extends
                    .clone()
                    .map(|t| {
                        let t = self.convert_type(self.simplify_type(t));
                        quote! {
                            ::core::convert::AsRef<#t> + ::core::convert::Into<#t>
                        }
                    })
                    .into_iter();
                let bounds_cloned = bounds.clone();
                let default = arg.default.clone().map(|t| {
                    let t = self.convert_type(self.simplify_type(t));
                    quote! {
                        = #t
                    }
                });
                (
                    quote! {
                        #name: __wsdom_load_ts_macro::JsCast #(+ #bounds)*
                    },
                    quote! {
                        #name: __wsdom_load_ts_macro::JsCast #(+ #bounds_cloned)* #default
                    },
                )
            });
            let with_bounds = with_bounds_with_defaults.clone().map(|(b, _d)| b);
            let with_defaults = with_bounds_with_defaults.map(|(_b, d)| d);
            let without_bounds = interface.generics.args.iter().map(|arg| {
                let name = new_ident_safe(arg.name);
                quote! {
                    #name
                }
            });
            let without_bounds_tokens = quote! {#(#without_bounds,)*};
            (
                Some(quote! { <#(#with_bounds,)*> }),
                Some(quote! { <#without_bounds_tokens> }),
                Some(quote! { <#(#with_defaults,)*> }),
                quote! {<(#without_bounds_tokens)>},
            )
        };

        let tokens = quote! {
            #[derive(::core::clone::Clone, __wsdom_load_ts_macro::RefCast)]
            #[repr(transparent)]
            pub struct #name #generics_with_default (__wsdom_load_ts_macro::JsValue, ::core::marker::PhantomData #generics_for_phantom );
        };

        let mut ancestors = Vec::new();

        static ALWAYS_EXTENDED: &[SimplifiedType] = &[known_types::UNKNOWN, known_types::OBJECT];
        interface
            .extends
            .iter()
            .map(|ty| self.simplify_type(ty.to_owned()))
            .chain(ALWAYS_EXTENDED.iter().cloned())
            .for_each(|ty| {
                self.visit_all_ancestors(&ty, &mut |ext| {
                    ancestors.push(ext.to_owned());
                    None::<()>
                });
                ancestors.push(ty);
            });
        ancestors.sort_by_key(|item| item.name);
        ancestors.dedup_by_key(|item| item.name);

        let first_extend = self.convert_type(
            interface
                .extends
                .first()
                .map(|ty| self.simplify_type(ty.to_owned()))
                .unwrap_or(known_types::OBJECT),
        );

        let extends = ancestors.into_iter().map(|anc| self.convert_type(anc));

        let tokens = quote! {
            #tokens

            impl #generics_with_bound __wsdom_load_ts_macro::JsCast for #name #generics_without_bound {
                fn unchecked_from_js(val: __wsdom_load_ts_macro::JsValue) -> Self {
                    Self(val, ::core::marker::PhantomData)
                }
                fn unchecked_from_js_ref(val: &__wsdom_load_ts_macro::JsValue) -> &Self {
                    __wsdom_load_ts_macro::RefCast::ref_cast(val)
                }
            }
            impl #generics_with_bound __wsdom_load_ts_macro::UseInJsCode for #name #generics_without_bound {
                fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.serialize_to(buf)
                }
            }

            impl #generics_with_bound ::core::convert::AsRef<Self> for #name #generics_without_bound {
                fn as_ref(&self) -> &Self {
                    self
                }
            }

            #(
                impl #generics_with_bound ::core::convert::AsRef<#extends> for #name #generics_without_bound {
                    fn as_ref(&self) -> &#extends {
                        __wsdom_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
                    }
                }
                impl #generics_with_bound ::core::convert::Into<#extends> for #name #generics_without_bound {
                    fn into(self) -> #extends {
                        __wsdom_load_ts_macro::JsCast::unchecked_from_js(self.0)
                    }
                }
            )*

            impl #generics_with_bound std::ops::Deref for #name #generics_without_bound {
                type Target = #first_extend;
                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }
        };

        let tokens = {
            let mut member_tokens = Vec::new();
            let all_members = decl_members
                .iter()
                .map(|member| (member, false))
                .chain(interface.members.iter().map(|member| (member, true)));

            let methods =
                all_members
                    .clone()
                    .filter_map(|(member, on_instance)| match &member.data {
                        Member::Method(x) => Some((x, on_instance)),
                        _ => None,
                    });
            let fields =
                all_members
                    .clone()
                    .filter_map(|(member, on_instance)| match &member.data {
                        Member::Field(x) => Some((x, on_instance)),
                        _ => None,
                    });
            let getters =
                all_members
                    .clone()
                    .filter_map(|(member, on_instance)| match &member.data {
                        Member::Getter(x) => Some((x, on_instance)),
                        _ => None,
                    });
            let setters =
                all_members
                    .clone()
                    .filter_map(|(member, on_instance)| match &member.data {
                        Member::Setter(x) => Some((x, on_instance)),
                        _ => None,
                    });
            {
                let methods = iter_dedupe_consecutive(methods, |(m, _)| (&m.name, m.args.len()));

                let mut last_method_name = MethodName::Name("");
                for (method, on_instance) in methods {
                    let is_overload = method.name == last_method_name;
                    last_method_name = method.name.clone();
                    member_tokens.push(self.make_method_code(
                        interface.name,
                        method,
                        on_instance,
                        is_overload,
                    ));
                }
            }
            {
                let fields = iter_dedupe_all(fields, |(f, _)| match &f.name {
                    FieldName::Name(s) => *s,
                    FieldName::Wildcard { .. } => "[]",
                });
                for (field, on_instance) in fields {
                    member_tokens.push(self.make_field_code(interface.name, field, on_instance));
                }
            }
            {
                for (getter, on_instance) in getters {
                    member_tokens.push(self.make_getter_code(interface.name, getter, on_instance));
                }
            }
            {
                for (setter, on_instance) in setters {
                    member_tokens.push(self.make_setter_code(interface.name, setter, on_instance));
                }
            }
            quote! {
                #tokens
                impl #generics_with_bound #name #generics_without_bound {
                    #(
                        #member_tokens
                    )*
                }
            }
        };

        tokens
    }
    fn make_method_code(
        &self,
        interface_name: &'_ str,
        method: &Method<'_>,
        on_instance: bool,
        is_overload: bool,
    ) -> Option<TokenStream> {
        let is_constructor =
            !on_instance && matches!(method.name, crate::parser::method::MethodName::Constructor);
        let method_name_str = match method.name {
            crate::parser::method::MethodName::Nothing => "call_self",
            crate::parser::method::MethodName::Constructor => "new",
            crate::parser::method::MethodName::Iterator => return None,
            crate::parser::method::MethodName::Name(name) => name,
        };
        let mut rust_name_str = to_snake_case(method_name_str);

        if is_overload {
            rust_name_str.push_str("_with");
            for arg in &method.args {
                rust_name_str.push('_');
                rust_name_str.push_str(arg.name);
            }
        }
        let method_name_ident = new_ident_safe(&rust_name_str);
        let (arg_types, arg_names_sig, last_arg_variadic) = self.make_sig_args(&method.args);
        let arg_names_body = arg_names_sig.clone();
        let ret = self.convert_type(self.simplify_type(method.ret.to_owned()));
        let method_generics = self.make_sig_generics(&method.generics.args);
        if on_instance {
            Some(quote! {
                pub fn #method_name_ident #method_generics (&self, #(#arg_names_sig: #arg_types,)*) -> #ret {
                    __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                        __wsdom_load_ts_macro::JsObject::js_call_method(self.as_ref(), #method_name_str, [
                            #( #arg_names_body as &dyn __wsdom_load_ts_macro::UseInJsCode, )*
                        ], #last_arg_variadic)
                    )
                }
            })
        } else {
            if is_constructor {
                Some(quote! {
                    pub fn #method_name_ident (browser: &__wsdom_load_ts_macro::Browser, #(#arg_names_sig: #arg_types,)*) -> #ret {
                        __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                            browser.call_constructor(#interface_name, [
                                #( #arg_names_body as &dyn __wsdom_load_ts_macro::UseInJsCode,)*
                            ], #last_arg_variadic)
                        )
                    }
                })
            } else {
                let function = format!("{}.{}", interface_name, method_name_str);
                Some(quote! {
                    pub fn #method_name_ident #method_generics (browser: &__wsdom_load_ts_macro::Browser, #(#arg_names_sig: #arg_types,)*) -> #ret {
                        __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                            browser.call_function(#function, [
                                #( #arg_names_body as &dyn __wsdom_load_ts_macro::UseInJsCode,)*
                            ], #last_arg_variadic)
                        )
                    }
                })
            }
        }
    }
    fn make_field_code(
        &self,
        interface_name: &'_ str,
        field: &Field<'_>,
        on_instance: bool,
    ) -> Option<TokenStream> {
        let field_name_str = match &field.name {
            crate::parser::field::FieldName::Name(name) => *name,
            crate::parser::field::FieldName::Wildcard { .. } => return None,
        };
        let mut ty = self.simplify_type(field.ty.to_owned());
        if field.optional {
            ty = SimplifiedType {
                name: "__translate_nullable",
                args: vec![ty],
            };
        }
        let ty_name = ty.name;
        let ty_tokens = self.convert_type(ty);
        let field_name_snake_case = to_snake_case(field_name_str);
        let getter = {
            let getter_name_ident = new_ident_safe(&format!("get_{field_name_snake_case}"));
            if on_instance {
                quote! {
                    pub fn #getter_name_ident (&self) -> #ty_tokens {
                        __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                            __wsdom_load_ts_macro::JsObject::js_get_field(self.as_ref(), &#field_name_str)
                        )
                    }
                }
            } else {
                quote! {
                    pub fn #getter_name_ident (browser: &__wsdom_load_ts_macro::Browser) -> #ty_tokens {
                        __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                            browser.get_field(&__wsdom_load_ts_macro::RawCodeImmediate( #interface_name ), &#field_name_str)
                        )
                    }
                }
            }
        };
        let setter = (!field.readonly).then(|| {
                    let setter_name_ident = new_ident_safe(&format!("set_{field_name_snake_case}"));
                    let ty_tokens = if self.classes.contains(ty_name) {
                        quote! {& #ty_tokens}
                    }
                    else {
                        quote! {&dyn __wsdom_load_ts_macro::ToJs< #ty_tokens >}
                    };
                    if on_instance {
                        quote!{
                            pub fn #setter_name_ident (&self, value: #ty_tokens) {
                                __wsdom_load_ts_macro::JsObject::js_set_field(self.as_ref(), &#field_name_str, value)
                            }
                        }
                    }
                    else {
                        quote!{
                            pub fn #setter_name_ident (browser: &__wsdom_load_ts_macro::Browser, value: #ty_tokens) {
                                browser.set_field(&__wsdom_load_ts_macro::RawCodeImmediate( #interface_name ), &#field_name_str, value)
                            }
                        }
                    }
                });
        Some(quote! {
            #getter
            #setter
        })
    }
    fn make_getter_code(
        &self,
        interface_name: &'_ str,
        getter: &Getter<'_>,
        on_instance: bool,
    ) -> Option<TokenStream> {
        if !on_instance {
            todo!("getter {} on constructor {}", getter.name, interface_name);
        }
        let field_name_str = getter.name;
        let getter_name_ident = new_ident_safe(&format!("get_{}", to_snake_case(field_name_str)));
        let ret = self.convert_type(self.simplify_type(getter.ret.to_owned()));
        Some(quote! {
            pub fn #getter_name_ident (&self) -> #ret {
                __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                    __wsdom_load_ts_macro::JsObject::js_get_field(self.as_ref(), &#field_name_str)
                )
            }
        })
    }
    fn make_setter_code(
        &self,
        interface_name: &'_ str,
        setter: &Setter<'_>,
        on_instance: bool,
    ) -> Option<TokenStream> {
        if !on_instance {
            todo!("setter {} on constructor {}", setter.name, interface_name);
        }
        let field_name_str = setter.name;
        let setter_name_ident = new_ident_safe(&format!("set_{}", to_snake_case(field_name_str)));
        let ty = self.simplify_type(setter.arg_ty.to_owned());
        let ty_name = ty.name;
        let ty_tokens = self.convert_type(ty);
        let ty_tokens = if self.classes.contains(ty_name) {
            quote! {& #ty_tokens}
        } else {
            quote! {&dyn __wsdom_load_ts_macro::ToJs< #ty_tokens >}
        };

        Some(quote! {
            pub fn #setter_name_ident (&self, value: #ty_tokens) {
                __wsdom_load_ts_macro::JsObject::js_set_field(self.as_ref(), &#field_name_str, value as &dyn __wsdom_load_ts_macro::UseInJsCode)
            }
        })
    }
}
