use super::*;
use rust_ad_core_macros::{combined_derivative_macro, compose};

// Primitive procedures
// -------------------------------------------------------------------

// Forward derivative of [std::ops::Add].
combined_derivative_macro!(add_i64, "0i64", "1i64", "1i64");
// Forward derivative of [std::ops::Sub].
combined_derivative_macro!(sub_i64, "0i64", "1i64", "-1i64");
// Forward derivative of [std::ops::Mul].
combined_derivative_macro!(mul_i64, "0i64", "{1}", "{0}");
// Forward derivative of [std::ops::Div].
combined_derivative_macro!(div_i64, "0i64", "1i64/{1}", "-{0}/({1}*{1})");