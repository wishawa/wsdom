use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::util::{iter_dedupe_all, iter_dedupe_consecutive},
    parser::{
        comment::WithComment,
        field::{Field, FieldName},
        interface::Interface,
        member::Member,
        ts_type::TsType,
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
                let extends = arg.extends.clone().map(|t| {
                    let t = self.convert_type(t);
                    quote! {
                        : AsRef<#t>
                    }
                });
                let default = arg.default.clone().map(|t| {
                    let t = self.convert_type(t);
                    quote! {
                        = #t
                    }
                });
                (
                    quote! {
                        #name #extends
                    },
                    quote! {
                        #name #extends #default
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
            #[derive(::core::clone::Clone, __wrmi_load_ts_macro::RefCast)]
            #[repr(transparent)]
            pub struct #name #generics_with_default (__wrmi_load_ts_macro::JsValue, ::core::marker::PhantomData #generics_for_phantom );
        };

        let extends = interface
            .extends
            .iter()
            .map(ToOwned::to_owned)
            .chain([known_types::UNKNOWN, known_types::OBJECT].into_iter())
            .map(|iface| self.convert_type(iface.to_owned()));
        let first_extend = self.convert_type(
            interface
                .extends
                .first()
                .unwrap_or(&known_types::OBJECT)
                .to_owned(),
        );

        let tokens = quote! {
            #tokens

            impl #generics_with_bound __wrmi_load_ts_macro::JsCast for #name #generics_without_bound {
                fn unchecked_from_js(val: __wrmi_load_ts_macro::JsValue) -> Self {
                    Self(val, ::core::marker::PhantomData)
                }
                fn unchecked_from_js_ref(val: &__wrmi_load_ts_macro::JsValue) -> &Self {
                    __wrmi_load_ts_macro::RefCast::ref_cast(val)
                }
            }
            impl #generics_with_bound __wrmi_load_ts_macro::UseInJsCode for #name #generics_without_bound {
                fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.serialize_to(buf)
                }
            }

            #(
                impl #generics_with_bound ::core::convert::AsRef<#extends> for #name #generics_without_bound {
                    fn as_ref(&self) -> &#extends {
                        __wrmi_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
                    }
                }
                impl #generics_with_bound ::core::convert::Into<#extends> for #name #generics_without_bound {
                    fn into(self) -> #extends {
                        __wrmi_load_ts_macro::JsCast::unchecked_from_js(self.0)
                    }
                }
            )*

            impl #generics_with_bound std::ops::Deref for #name #generics_without_bound {
                type Target = #first_extend;
                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }

            // impl #generics_with_bound __wrmi_load_ts_macro::ToJs< #name #generics_with_bound > for #name #generics_with_bound {}
        };

        let tokens = {
            let member_code = iter_dedupe_all(
                iter_dedupe_consecutive(
                    decl_members
                        .iter()
                        .map(|member| (member, false))
                        .chain(interface.members.iter().map(|member| (member, true))),
                    |(member, _)| match &member.data {
                        Member::Method(m) => Some(&m.name),
                        _ => None,
                    },
                ),
                |(member, _)| match &member.data {
                    Member::Field(Field {
                        name: FieldName::Name(name),
                        ..
                    }) => Some(*name),
                    _ => None,
                },
            )
            .filter_map(|(member, on_instance)| {
                self.make_member_code(interface.name, &member.data, on_instance)
            });

            quote! {
                #tokens
                impl #generics_with_bound #name #generics_without_bound {
                    #(#member_code)*
                }
            }
        };

        tokens
    }
    fn make_member_code(
        &self,
        interface_name: &'_ str,
        member: &Member<'_>,
        on_instance: bool,
    ) -> Option<TokenStream> {
        match member {
            Member::Method(method) => {
                let is_constructor = !on_instance
                    && matches!(method.name, crate::parser::method::MethodName::Constructor);
                let method_name_str = match method.name {
                    crate::parser::method::MethodName::Nothing => "call_self",
                    crate::parser::method::MethodName::Constructor => "new",
                    crate::parser::method::MethodName::Iterator => return None,
                    crate::parser::method::MethodName::Name(name) => name,
                };
                let method_name_ident = new_ident_safe(&to_snake_case(method_name_str));
                let arg_names_sig = method.args.iter().map(|arg| new_ident_safe(arg.name));
                let arg_names_body = arg_names_sig.clone();
                let arg_types = method.args.iter().map(|arg| {
                    let arg_type = self.convert_type(arg.ty.to_owned());
                    quote! {&impl __wrmi_load_ts_macro::ToJs<#arg_type>}
                });
                let last_arg_variadic = method.args.iter().any(|arg| arg.variadic);
                let ret = self.convert_type(method.ret.to_owned());
                let method_generics = method.generics.args.iter().map(|gen| {
                    let name = new_ident_safe(gen.name);
                    let extends = gen.extends.clone().map(|ty| self.convert_type(ty)).into_iter();
                    quote! {
                        #name #(: ::core::convert::AsRef<#extends> + ::core::convert::Into<#extends>)*
                    }
                });
                let method_generics = if method.generics.args.is_empty() {
                    None
                } else {
                    Some(quote! {
                        <#(#method_generics,)*>
                    })
                };
                if on_instance {
                    Some(quote! {
                        pub fn #method_name_ident #method_generics (&self, #(#arg_names_sig: #arg_types,)*) -> #ret {
                            __wrmi_load_ts_macro::JsCast::unchecked_from_js(
                                __wrmi_load_ts_macro::JsObject::js_call_method(self.as_ref(), #method_name_str, [
                                    #( #arg_names_body as &dyn __wrmi_load_ts_macro::UseInJsCode, )*
                                ], #last_arg_variadic)
                            )
                        }
                    })
                } else {
                    if is_constructor {
                        let function = format!("new {}", interface_name);
                        Some(quote! {
                            pub fn #method_name_ident (browser: &__wrmi_load_ts_macro::Browser, #(#arg_names_sig: #arg_types,)*) -> #ret {
                                __wrmi_load_ts_macro::JsCast::unchecked_from_js(
                                    browser.call_function(#function, [
                                        #( #arg_names_body as &dyn __wrmi_load_ts_macro::UseInJsCode,)*
                                    ], #last_arg_variadic)
                                )
                            }
                        })
                    } else {
                        let function = format!("{}.{}", interface_name, method_name_str);
                        Some(quote! {
                            pub fn #method_name_ident #method_generics (browser: &__wrmi_load_ts_macro::Browser, #(#arg_names_sig: #arg_types,)*) -> #ret {
                                __wrmi_load_ts_macro::JsCast::unchecked_from_js(
                                    browser.call_function(#function, [
                                        #( #arg_names_body as &dyn __wrmi_load_ts_macro::UseInJsCode,)*
                                    ], #last_arg_variadic)
                                )
                            }
                        })
                    }
                }
            }
            Member::Field(field) => {
                let field_name_str = match &field.name {
                    crate::parser::field::FieldName::Name(name) => *name,
                    crate::parser::field::FieldName::Wildcard { .. } => return None,
                };
                let mut ty = field.ty.to_owned();
                if field.optional {
                    ty = TsType::Union {
                        pair: Box::new((ty, known_types::NULL)),
                    };
                }
                let ty = self.convert_type(ty);
                let field_name_snake_case = to_snake_case(field_name_str);
                let getter = {
                    let getter_name_ident = new_ident_safe(&format!("get_{field_name_snake_case}"));
                    if on_instance {
                        quote! {
                            pub fn #getter_name_ident (&self) -> #ty {
                                __wrmi_load_ts_macro::JsCast::unchecked_from_js(
                                    __wrmi_load_ts_macro::JsObject::js_get_field(self.as_ref(), &#field_name_str)
                                )
                            }
                        }
                    } else {
                        quote! {
                            pub fn #getter_name_ident (browser: &__wrmi_load_ts_macro::Browser) -> #ty {
                                __wrmi_load_ts_macro::JsCast::unchecked_from_js(
                                    browser.get_field(&__wrmi_load_ts_macro::RawCodeImmediate(#interface_name), &#field_name_str)
                                )
                            }
                        }
                    }
                };
                let setter = (!field.readonly).then(|| {
                    let setter_name_ident = new_ident_safe(&format!("set_{field_name_snake_case}"));
                    if on_instance {
                        quote!{
                            pub fn #setter_name_ident (&self, value: &impl __wrmi_load_ts_macro::ToJs<#ty>) {
                                __wrmi_load_ts_macro::JsObject::js_set_field(self.as_ref(), &#field_name_str, value)
                            }
                        }
                    }
                    else {
                        quote!{
                            pub fn #setter_name_ident (browser: &__wrmi_load_ts_macro::Browser, value: &impl __wrmi_load_ts_macro::ToJs<#ty>) {
                                browser.set_field(&__wrmi_load_ts_macro::RawCodeImmediate(#interface_name), &#field_name_str, value)
                            }
                        }
                    }
                });
                Some(quote! {
                    #getter
                    #setter
                })
            }
            Member::Getter(getter) => {
                if !on_instance {
                    todo!("getter {} on constructor {}", getter.name, interface_name);
                }
                let field_name_str = getter.name;
                let getter_name_ident =
                    new_ident_safe(&format!("get_{}", to_snake_case(field_name_str)));
                let ret = self.convert_type(getter.ret.to_owned());
                Some(quote! {
                    pub fn #getter_name_ident (&self) -> #ret {
                        __wrmi_load_ts_macro::JsCast::unchecked_from_js(
                            __wrmi_load_ts_macro::JsObject::js_get_field(self.as_ref(), &#field_name_str)
                        )
                    }
                })
            }
            Member::Setter(setter) => {
                if !on_instance {
                    todo!("setter {} on constructor {}", setter.name, interface_name);
                }
                let field_name_str = setter.name;
                let setter_name_ident =
                    new_ident_safe(&format!("set_{}", to_snake_case(field_name_str)));
                let ty = self.convert_type(setter.arg_ty.to_owned());
                Some(quote! {
                    pub fn #setter_name_ident (&self, value: #ty) {
                        __wrmi_load_ts_macro::JsObject::js_set_field(self.as_ref(), &#field_name_str, &value)
                    }
                })
            }
        }
    }
}
