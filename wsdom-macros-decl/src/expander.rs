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
