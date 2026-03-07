/// Construct a [`Value`] from a Rust-like constructor expression.
///
/// With this macro it is possible to construct values that have the same
/// structure as actual types like structs, enums, tuples, or even primitive
/// types. It is not necessary to declare the type in advance.
///
/// [`Value`]: crate::recursive_comparison::value::Value
#[macro_export]
macro_rules! value {
    // Hide distracting implementation details from the generated rustdoc.
    ($($value:tt)+) => {
        $crate::value_internal!($($value)+)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! value_internal {
    () => {
    };
    ($head:tt, $($rest:tt)*) => {
        $crate::value_internal!($head), $crate::value_internal!($($rest)*)
    };
    ($name:ident { $($tt:tt)+ }) => {
        $crate::recursive_comparison::value::struct_(
            stringify!($name),
            [$crate::value_internal!($($tt)+),]
        )
    };
    ({$($tt:tt)+}) => {
        $crate::recursive_comparison::value::struct_with_fields(
            [$crate::value_internal!($($tt)+),]
        )
    };
    ($name:ident($($tt:tt)+)) => {
        $crate::recursive_comparison::value::tuple_struct(
            stringify!($name),
            [$crate::value_internal!($($tt)+),]
        )
    };
    (($($tt:tt)+)) => {
        $crate::recursive_comparison::value::tuple(
            [$crate::value_internal!($($tt)+),]
        )
    };
    ($name:ident: $val:tt) => {
        $crate::recursive_comparison::value::field(stringify!($name), value_internal!($val))
    };
    (false) => {
        $crate::recursive_comparison::value::Value::Bool(false)
    };
    (true) => {
        $crate::recursive_comparison::value::Value::Bool(true)
    };
    ( $val:expr) => {
        $crate::recursive_comparison::serialize::to_recursive_value( &$ val)
            .unwrap_or_else( | err | panic! ("failed to serialize expression: {err}"))
    };
}

#[cfg(test)]
mod tests;
