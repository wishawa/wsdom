use crate::js::value::JsValue;

pub trait JsCast
where
    Self: AsRef<JsValue> + Into<JsValue>,
{
    // fn instanceof(val: &JsValue) -> bool;
    fn unchecked_from_js(val: JsValue) -> Self;
    fn unchecked_from_js_ref(val: &JsValue) -> &Self;

    // fn has_type<T>(&self) -> bool
    //    where T: JsCast { ... }
    // fn dyn_into<T>(self) -> Result<T, Self>
    //    where T: JsCast { ... }
    // fn dyn_ref<T>(&self) -> Option<&T>
    //    where T: JsCast { ... }
    fn unchecked_into<T>(self) -> T
    where
        T: JsCast,
    {
        T::unchecked_from_js(self.into())
    }
    fn unchecked_ref<T>(&self) -> &T
    where
        T: JsCast,
    {
        T::unchecked_from_js_ref(self.as_ref())
    }
    // fn is_instance_of<T>(&self) -> bool
    //    where T: JsCast { ... }
    // fn is_type_of(val: &JsValue) -> bool { ... }
}
