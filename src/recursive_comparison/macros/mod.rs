/// Construct a [`Value`] from a Rust-like constructor expression.
///
/// With this macro it is possible to construct values that have the same
/// structure as actual types like structs, enums, tuples, or even primitive
/// types. It is not necessary to declare the type in advance.
///
/// [`Value`]: crate::recursive_comparison::value::Value
#[macro_export]
macro_rules! value {
    // Booleans
    (false) => {
        $crate::recursive_comparison::value::Value::Bool(false)
    };
    (true) => {
        $crate::recursive_comparison::value::Value::Bool(true)
    };

    // Named struct: Foo { a: 1, b: 2 }
    ($name:ident { $($field:ident : $val:tt),* $(,)? }) => {
        $crate::recursive_comparison::value::struct_(
            stringify!($name),
            [$( $crate::recursive_comparison::value::Field::from((stringify!($field), $crate::value!($val))) ),*]
        )
    };

    // Empty Ad-hoc struct: { }
    ({ }) => {
        {
            let fields: [$crate::recursive_comparison::value::Field;0] = [];
            $crate::recursive_comparison::value::struct_with_fields(fields)
        }
    };

    // Anonymous struct: { a: 1, b: 2 }
    ({ $($field:ident : $val:tt),+ $(,)? }) => {
        $crate::recursive_comparison::value::struct_with_fields(
            [$( $crate::recursive_comparison::value::Field::from((stringify!($field), $crate::value!($val))) ),+]
        )
    };

    // Tuple struct: Foo(1, 2)
    ($name:ident ( $($val:tt),* $(,)? )) => {
        $crate::recursive_comparison::value::tuple_struct(
            stringify!($name),
            [$( $crate::value!($val) ),*]
        )
    };

    // Empty Tuple or Unit: ()
    (()) => {
        $crate::recursive_comparison::value::Value::Unit
    };

    // Tuple: (1, 2)
    (( $($val:tt),+ $(,)? )) => {
        $crate::recursive_comparison::value::tuple(
            [$( $crate::value!($val) ),+]
        )
    };

    // Fallback for any other expression
    ($val:expr) => {
        $crate::recursive_comparison::serialize::to_recursive_value(&$val)
            .unwrap_or_else(|err| panic!("failed to serialize expression: {err}"))
    };
}

#[cfg(test)]
mod tests;
