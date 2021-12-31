use super::*;
use rust_ad_core_macros::{combined_derivative_macro, compose};

// Primitive procedures
// -------------------------------------------------------------------

// Forward derivative of [std::ops::Add].
combined_derivative_macro!(add_i8, "0i8", "1i8", "1i8");
// Forward derivative of [std::ops::Sub].
combined_derivative_macro!(sub_i8, "0i8", "1i8", "-1i8");
// Forward derivative of [std::ops::Mul].
combined_derivative_macro!(mul_i8, "0i8", "{1}", "{0}");
// Forward derivative of [std::ops::Div].
combined_derivative_macro!(div_i8, "0i8", "1i8/{1}", "-{0}/({1}*{1})");