use super::primitives::{JsBoolean, JsNumber, JsString};
use super::value::JsValue;
use crate::link::Browser;
use crate::serialize::{UseInJsCode, UseInJsCodeWriter};
use crate::{JsCast, ToJs};

#[rustfmt::skip]
impl Browser {
    /// The JavaScript `+` operator
    pub fn op_add           (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsValue    {self.value_from_raw_code(format_args!("(({}) + ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `-` operator
    pub fn op_sub           (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) - ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `*` operator
    pub fn op_mul           (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) * ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `/` operator
    pub fn op_div           (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) / ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `%` operator
    pub fn op_remainder     (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) % ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `**` operator
    pub fn op_exponent      (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) ** ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `&` operator
    pub fn op_bitwise_and   (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) & ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `|` operator
    pub fn op_bitwise_or    (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) | ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `^` operator
    pub fn op_bitwise_xor   (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) ^ ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `<<` operator
    pub fn op_lshift2       (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) << ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `>>` operator
    pub fn op_rshift2       (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) >> ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `>>>` operator
    pub fn op_rshift3       (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsNumber   {self.value_from_raw_code(format_args!("(({}) >>> ({}))",   UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `&&` operator
    pub fn op_logical_and   (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsValue    {self.value_from_raw_code(format_args!("(({}) && ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `||` operator
    pub fn op_logical_or    (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsValue    {self.value_from_raw_code(format_args!("(({}) || ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `==` operator
    pub fn op_eq2           (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsBoolean  {self.value_from_raw_code(format_args!("(({}) == ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `!=` operator
    pub fn op_neq1          (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsBoolean  {self.value_from_raw_code(format_args!("(({}) != ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `===` operator
    pub fn op_eq3           (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsBoolean  {self.value_from_raw_code(format_args!("(({}) === ({}))",   UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `!==` operator
    pub fn op_neq2          (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsBoolean  {self.value_from_raw_code(format_args!("(({}) !== ({}))",   UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `>` operator
    pub fn op_gt            (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsBoolean  {self.value_from_raw_code(format_args!("(({}) > ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `>=` operator
    pub fn op_gte           (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsBoolean  {self.value_from_raw_code(format_args!("(({}) >= ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `<` operator
    pub fn op_lt            (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsBoolean  {self.value_from_raw_code(format_args!("(({}) < ({}))",     UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}
    /// The JavaScript `<=` operator
    pub fn op_lte           (&self, a: &dyn UseInJsCode, b: &dyn UseInJsCode) -> JsBoolean  {self.value_from_raw_code(format_args!("(({}) <= ({}))",    UseInJsCodeWriter(a), UseInJsCodeWriter(b))).unchecked_into()}

    /// The JavaScript `-` operator
    pub fn op_neg           (&self, a: &dyn UseInJsCode) -> JsNumber    {self.value_from_raw_code(format_args!("(-({}))", UseInJsCodeWriter(a))).unchecked_into()}
    /// The JavaScript `~` operator
    pub fn op_bitwise_not   (&self, a: &dyn UseInJsCode) -> JsNumber    {self.value_from_raw_code(format_args!("(~({}))", UseInJsCodeWriter(a))).unchecked_into()}
    /// The JavaScript `!` operator
    pub fn op_logical_not   (&self, a: &dyn UseInJsCode) -> JsBoolean   {self.value_from_raw_code(format_args!("(!({}))", UseInJsCodeWriter(a))).unchecked_into()}
}

macro_rules! impl_number_op {
    ($trait_name:ident, $method_name:ident, $wsdom_name:ident) => {
        impl<'a, T: ToJs<JsNumber>> $trait_name<&'a T> for &'a JsNumber {
            type Output = JsNumber;

            fn $method_name(self, rhs: &'a T) -> Self::Output {
                self.browser.$wsdom_name(self, rhs).unchecked_into()
            }
        }
    };
}

use std::ops::*;
impl_number_op!(Add, add, op_add);
impl_number_op!(Sub, sub, op_sub);
impl_number_op!(Mul, mul, op_mul);
impl_number_op!(Div, div, op_div);
impl_number_op!(Rem, rem, op_remainder);
impl_number_op!(Shl, shl, op_lshift2);
impl_number_op!(Shr, shr, op_rshift2);
impl_number_op!(BitAnd, bitand, op_bitwise_and);
impl_number_op!(BitOr, bitor, op_bitwise_or);
impl_number_op!(BitXor, bitxor, op_bitwise_xor);

impl<'a, T: ToJs<JsString>> Add<&'a T> for &'a JsString {
    type Output = JsString;

    fn add(self, rhs: &'a T) -> Self::Output {
        self.browser.op_add(self, rhs).unchecked_into()
    }
}
impl<'a> Neg for &'a JsNumber {
    type Output = JsNumber;

    fn neg(self) -> Self::Output {
        self.browser.op_neg(self)
    }
}
