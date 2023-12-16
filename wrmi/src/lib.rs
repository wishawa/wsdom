pub use wrmi_macros::load_ts;

pub mod __wrmi_load_ts_macro {
    pub use ref_cast::RefCast;
    pub use wrmi_core::{js_types::*, Browser, JsCast, ToJs, UseInJsCode};
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {
        mod res {
            use super::super::__wrmi_load_ts_macro;
            #[derive(::core::clone::Clone, __wrmi_load_ts_macro::RefCast)]
            #[repr(transparent)]
            struct EventListenerOptions(
                __wrmi_load_ts_macro::JsValue,
                ::core::marker::PhantomData<()>,
            );

            impl __wrmi_load_ts_macro::JsCast for EventListenerOptions {
                fn unchecked_from_js(val: __wrmi_load_ts_macro::JsValue) -> Self {
                    Self(val, ::core::marker::PhantomData)
                }
                fn unchecked_from_js_ref(val: &__wrmi_load_ts_macro::JsValue) -> &Self {
                    __wrmi_load_ts_macro::RefCast::ref_cast(val)
                }
            }
            impl __wrmi_load_ts_macro::UseInJsCode for EventListenerOptions {
                fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.serialize_to(buf)
                }
            }
            impl ::core::convert::AsRef<__wrmi_load_ts_macro::JsValue> for EventListenerOptions {
                fn as_ref(&self) -> &__wrmi_load_ts_macro::JsValue {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
                }
            }
            impl ::core::convert::Into<__wrmi_load_ts_macro::JsValue> for EventListenerOptions {
                fn into(self) -> __wrmi_load_ts_macro::JsValue {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js(self.0)
                }
            }
            impl ::core::convert::AsRef<__wrmi_load_ts_macro::JsObject> for EventListenerOptions {
                fn as_ref(&self) -> &__wrmi_load_ts_macro::JsObject {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
                }
            }
            impl ::core::convert::Into<__wrmi_load_ts_macro::JsObject> for EventListenerOptions {
                fn into(self) -> __wrmi_load_ts_macro::JsObject {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js(self.0)
                }
            }
            impl std::ops::Deref for EventListenerOptions {
                type Target = __wrmi_load_ts_macro::JsObject;
                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }
            impl EventListenerOptions {}

            #[derive(::core::clone::Clone, __wrmi_load_ts_macro::RefCast)]
            #[repr(transparent)]
            struct AddEventListenerOptions(
                __wrmi_load_ts_macro::JsValue,
                ::core::marker::PhantomData<()>,
            );

            impl __wrmi_load_ts_macro::JsCast for AddEventListenerOptions {
                fn unchecked_from_js(val: __wrmi_load_ts_macro::JsValue) -> Self {
                    Self(val, ::core::marker::PhantomData)
                }
                fn unchecked_from_js_ref(val: &__wrmi_load_ts_macro::JsValue) -> &Self {
                    __wrmi_load_ts_macro::RefCast::ref_cast(val)
                }
            }
            impl __wrmi_load_ts_macro::UseInJsCode for AddEventListenerOptions {
                fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.serialize_to(buf)
                }
            }
            impl ::core::convert::AsRef<EventListenerOptions> for AddEventListenerOptions {
                fn as_ref(&self) -> &EventListenerOptions {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
                }
            }
            impl ::core::convert::Into<EventListenerOptions> for AddEventListenerOptions {
                fn into(self) -> EventListenerOptions {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js(self.0)
                }
            }
            impl ::core::convert::AsRef<__wrmi_load_ts_macro::JsValue> for AddEventListenerOptions {
                fn as_ref(&self) -> &__wrmi_load_ts_macro::JsValue {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
                }
            }
            impl ::core::convert::Into<__wrmi_load_ts_macro::JsValue> for AddEventListenerOptions {
                fn into(self) -> __wrmi_load_ts_macro::JsValue {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js(self.0)
                }
            }
            impl ::core::convert::AsRef<__wrmi_load_ts_macro::JsObject> for AddEventListenerOptions {
                fn as_ref(&self) -> &__wrmi_load_ts_macro::JsObject {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
                }
            }
            impl ::core::convert::Into<__wrmi_load_ts_macro::JsObject> for AddEventListenerOptions {
                fn into(self) -> __wrmi_load_ts_macro::JsObject {
                    __wrmi_load_ts_macro::JsCast::unchecked_from_js(self.0)
                }
            }
            impl std::ops::Deref for AddEventListenerOptions {
                type Target = EventListenerOptions;
                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }
            impl AddEventListenerOptions {}
        }
    }
}
