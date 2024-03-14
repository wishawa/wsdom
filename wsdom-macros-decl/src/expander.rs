#[macro_export]
macro_rules! expand_class_def {
    ($generics_for_phantom:ty, [$($impl_generics:tt)*], $name_without_bound:ty, $name:ident, [$($decl_generics:tt)*], $first_extend:ty, $($all_extends:ty,)*) => {
        #[derive(::core::clone::Clone, __wsdom_load_ts_macro::RefCast)]
        #[repr(transparent)]
        pub struct $name $($decl_generics)* (__wsdom_load_ts_macro::JsValue, ::core::marker::PhantomData<$generics_for_phantom> );
        impl $($impl_generics)* __wsdom_load_ts_macro::JsCast for $name_without_bound
        {
            fn unchecked_from_js(val: __wsdom_load_ts_macro::JsValue) -> Self {
                Self(val, ::core::marker::PhantomData)
            }
            fn unchecked_from_js_ref(val: &__wsdom_load_ts_macro::JsValue) -> &Self {
                __wsdom_load_ts_macro::RefCast::ref_cast(val)
            }
        }
        impl $($impl_generics)* __wsdom_load_ts_macro::UseInJsCode for $name_without_bound
        {
            fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.serialize_to(buf)
            }
        }

        impl $($impl_generics)* ::core::convert::AsRef<Self> for $name_without_bound
        {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl $($impl_generics)* std::ops::Deref for $name_without_bound
        {
            type Target = $first_extend;
            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }
        __wsdom_load_ts_macro::expand_class_def!(extend @ [$($impl_generics)*], $name_without_bound, $($all_extends,)*);
    };
    (extend @ [$($impl_generics:tt)*], $name_without_bound:ty, $extend:ty, $($extends_rest:ty,)*) => {
        impl $($impl_generics)* ::core::convert::AsRef<$extend> for $name_without_bound
        {
            fn as_ref(&self) -> &$extend {
                __wsdom_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
            }
        }
        impl $($impl_generics)* ::core::convert::Into<$extend> for $name_without_bound
        {
            fn into(self) -> $extend {
                __wsdom_load_ts_macro::JsCast::unchecked_from_js(self.0)
            }
        }
        __wsdom_load_ts_macro::expand_class_def!(extend @ [$($impl_generics)*], $name_without_bound, $($extends_rest,)*);
    };
    (extend @ [$($impl_generics:tt)*], $name_without_bound:ty, ) => {

    };
}

#[macro_export]
macro_rules! expand_field_getter_setter {
    (self @ $getter_name:ident, $getter_ty:ty, $setter_name:ident, $setter_ty:ty, $field_name:literal) => {
        pub fn $getter_name(&self) -> $getter_ty {
            __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                __wsdom_load_ts_macro::JsObject::js_get_field(self.as_ref(), &$field_name),
            )
        }
        pub fn $setter_name(&self, value: $setter_ty) {
            __wsdom_load_ts_macro::JsObject::js_set_field(
                self.as_ref(),
                &$field_name,
                __wsdom_load_ts_macro::UpcastWorkaround::new(value).cast(),
            )
        }
    };
    (browser @ $getter_name:ident, $getter_ty:ty, $setter_name:ident, $setter_ty:ty, $field_name:literal, $iface_name:literal) => {
        pub fn $getter_name(browser: &__wsdom_load_ts_macro::Browser) -> $getter_ty {
            __wsdom_load_ts_macro::JsCast::unchecked_from_js(browser.get_field(
                &__wsdom_load_ts_macro::RawCodeImmediate($iface_name),
                &$field_name,
            ))
        }
        pub fn $setter_name(browser: &__wsdom_load_ts_macro::Browser, value: $setter_ty) {
            browser.set_field(
                &__wsdom_load_ts_macro::RawCodeImmediate($iface_name),
                &$field_name,
                __wsdom_load_ts_macro::UpcastWorkaround::new(value).cast(),
            )
        }
    };
}

#[macro_export]
macro_rules! expand_method {
    (self @ $method_name:ident, [$($generics:tt)*], [$($arg_names:ident : $arg_types:ty,)*], $ret:ty, $js_name:literal, $last_variadic:literal) => {
        pub fn $method_name $($generics)* (&self, $($arg_names: $arg_types,)*) -> $ret {
            __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                __wsdom_load_ts_macro::JsObject::js_call_method(self.as_ref(), $js_name, [
                    $(  __wsdom_load_ts_macro::UpcastWorkaround::new( $arg_names ).cast(), )*
                ], $last_variadic)
            )
        }
    };
    (constructor @ $method_name:ident, [$($generics:tt)*], [$($arg_names:ident : $arg_types:ty,)*], $ret:ty, $interface_name:literal, $last_variadic:literal) => {
        pub fn $method_name $($generics)* (browser: &__wsdom_load_ts_macro::Browser, $($arg_names: $arg_types,)*) -> $ret {
            __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                browser.call_constructor($interface_name, [
                    $(  __wsdom_load_ts_macro::UpcastWorkaround::new( $arg_names ).cast(), )*
                ], $last_variadic)
            )
        }
    };
    (free @ $method_name:ident, [$($generics:tt)*], [$($arg_names:ident : $arg_types:ty,)*], $ret:ty, $function_name:literal, $last_variadic:literal) => {
        pub fn $method_name $($generics)* (browser: &__wsdom_load_ts_macro::Browser, $($arg_names: $arg_types,)*) -> $ret {
            __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                browser.call_function($function_name, [
                    $(  __wsdom_load_ts_macro::UpcastWorkaround::new( $arg_names ).cast(), )*
                ], $last_variadic)
            )
        }
    };
}
