macro_rules! impl_basic {
    ($name:ident; $($generic:ident)?; $derefsto:ident) => {
        #[derive(Clone, ::ref_cast::RefCast)]
        #[repr(transparent)]
        pub struct $name $(<$generic>)? (JsValue, ::core::marker::PhantomData <($($generic,)?)> );

        impl $(<$generic>)? std::ops::Deref for $name $(<$generic>)? {
            type Target = JsValue;
            fn deref(&self) -> &JsValue {
                self.as_ref()
            }
        }

        impl_basic!(@one_conversion; $name; $($generic)?; $derefsto);

        impl $(<$generic>)? JsCast for $name $(<$generic>)? {
            fn unchecked_from_js(val: JsValue) -> Self {
                Self(val, ::core::marker::PhantomData)
            }
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                ::ref_cast::RefCast::ref_cast(val)
            }
        }

        impl $(<$generic>)? UseInJsCode for $name $(<$generic>)? {
            fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.serialize_to(buf)
            }
        }

        impl $(<$generic>)? AsRef<Self> for $name $(<$generic>)? {
            fn as_ref(&self) -> & Self {
                self
            }
        }
    };
    ($name:ident) => {
        impl_basic!($name;;JsValue);
    };
    (@one_conversion; $name: ident; $($generic:ident)?; $convertto:ident) => {
        impl $(<$generic>)? Into<$convertto> for $name $(<$generic>)? {
            fn into(self) -> $convertto {
                JsCast::unchecked_from_js(self.0)
            }
        }
        impl $(<$generic>)? AsRef<$convertto> for $name $(<$generic>)? {
            fn as_ref(&self) -> & $convertto {
                JsCast::unchecked_from_js_ref(&self.0)
            }
        }
    }
}