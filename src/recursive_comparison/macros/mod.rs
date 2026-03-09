/// Construct a [`Value`] from a Rust-like constructor expression.
///
/// With this macro it is possible to construct values that have the same
/// structure as actual types like structs, enums, tuples, or even primitive
/// types. It is not necessary to declare the types in advance.
///
/// # Syntax
///
/// The syntax is more or less the same as Rust's constructor expressions for
/// structs, enums, and tuples. The name of structs can be omitted. Enum
/// variants must be written in the form `Foo::Bar` (specifying only the
/// variant is not supported, even if the variant is imported).
///
/// Literals for bool, char, number, and strings can
/// be written similar to Rust literals with some minor differences: a `&str`
/// does not have to be converted to a `String` (this is done automatically),
/// and brackets are used for sequences, not arrays. Each number literal should
/// contain the type, e.g., `42_u64`, `1.2_f32`, etc. This is necessary because the
/// macro cannot infer the type of number literals.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let value = value!({
///     foo: 2.3_f64,
///     bar: {
///         baz: "alpha"
///         qux: 123_i16,
///         corge: true,
///     },
///     grault: Sample::Two("beta", 456_u32, 'b'),
///     waldo: (123_u8, 234_u8, 56_u8)
///     fred: ['a', 'b', 'c'],
///     thud: Named(0.8_f32),
/// });
/// ```
///
/// # Limitations
///
/// ## No Field Init Shorthand
///
/// When initializing a struct field with the value of a variable, the field
/// init shorthand is not supported. Even if the field has the same name as the
/// variable, the variable must be repeated after the colon.
///
/// Instead of using the field init shorthand, which does not compile:
///
/// ```compile_fail
/// use asserting::prelude::*;
///
/// let bar = "alpha";
///
/// let _value = value!(Foo { bar });
/// ```
///
/// use the normal (verbose) syntax:
///
/// ```
/// use asserting::prelude::*;
///
/// let bar = "alpha";
///
/// let value = value!(Foo { bar: bar });
///
/// assert_eq!(value, value!(Foo { bar: "alpha" }));
/// ```
///
/// ## No Unit Structs
///
/// This macro does not support unit structs. Unit structs interfere with
/// identifiers captured from the environment. We decided that capturing
/// variables from the environment of the macro is more valuable than unit
/// structs.
///
/// [`Value`]: crate::recursive_comparison::value::Value
#[macro_export]
macro_rules! value {
    ////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the values of a seq [...].
    //
    // Must be invoked as: value!(@seq [] ($($tt)*))
    // It returns a 'Vec<Value>`.
    ////////////////////////////////////////////////////////////////////////

    // Done with a trailing comma.
    (@seq [$($elems:expr,)*] ()) => {
        $crate::__private::vec![$($elems,)*]
    };

    // Done without a trailing comma.
    (@seq [$($elems:expr),*] ()) => {
        $crate::__private::vec![$($elems),*]
    };

    // The next element is a seq.
    (@seq [$($elems:expr,)*] ([ $($val:tt)* ] $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!([$($val)*]),] ($($rest)*))
    };

    // The next element is an anonymous struct.
    (@seq [$($elems:expr,)*] ({ $($val:tt)* } $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!({$($val)*}),] ($($rest)*))
    };

    // The next element is a named struct.
    (@seq [$($elems:expr,)*] ($name:ident { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!($name {$($val)*}),] ($($rest)*))
    };

    // The next element is a tuple struct.
    (@seq [$($elems:expr,)*] ($name:ident ( $($val:tt)* ) $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!($name ($($val)*)),] ($($rest)*))
    };

    // The next element is a struct variant.
    (@seq [$($elems:expr,)*] ($name:ident :: $variant:ident { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!($name :: $variant {$($val)*}),] ($($rest)*))
    };

    // The next element is a tuple variant.
    (@seq [$($elems:expr,)*] ($name:ident :: $variant:ident ( $($val:tt)* ) $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!($name :: $variant ($($val)*)),] ($($rest)*))
    };

    // The next element is a unit variant.
    (@seq [$($elems:expr,)*] ($name:ident :: $variant:ident $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!($name :: $variant),] ($($rest)*))
    };

    // The next element is an expression followed by a comma.
    (@seq [$($elems:expr,)*] ($next:expr , $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!($next),] ($($rest)*))
    };

    // The last element is an expression with no trailing comma.
    (@seq [$($elems:expr,)*] ($last:expr)) => {
        $crate::value!(@seq [$($elems,)* $crate::value!($last),] ())
    };

    // Comma after the most recent element.
    (@seq [$($elems:expr),*] (, $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems),*] ($($rest)*))
    };

    // Comma after the most recent element.
    (@seq [$($elems:expr,)*] (, $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)*] ($($rest)*))
    };

    ////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the fields of a struct {...}.
    //
    // Must be invoked as: value!(@fields [] ($($tt)*))
    // It returns a 'Vec<Field>'.
    ////////////////////////////////////////////////////////////////////////

    // Done with a trailing comma.
    (@fields [$($fields:expr,)*] ()) => {
        $crate::std::vec![$($fields,)*]
    };

    // Done without a trailing comma.
    (@fields [$($fields:expr),*] ()) => {
        $crate::std::vec![$($fields),*]
    };

    // The next value is a seq.
    (@fields [$($fields:expr,)*] ($key:ident : [ $($val:tt)* ] $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!([$($val)*]),
            },
        ] ($($rest)*))
    };

    // The nex value is a named struct.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!($name { $($val)* }),
            },
        ] ($($rest)*))
    };

    // The next value is an anonymous struct.
    (@fields [$($fields:expr,)*] ($key:ident : { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!({$($val)*}),
            },
        ] ($($rest)*))
    };

    // The next value is a tuple struct.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident ( $($val:tt)* ) $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!($name ($($val)*)),
            },
        ] ($($rest)*))
    };

    // The next value is a struct variant.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident :: $variant:ident { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!($name :: $variant {$($val)*}),
            },
        ] ($($rest)*))
    };

    // The next value is a tuple variant.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident :: $variant:ident ( $($val:tt)* ) $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!($name :: $variant ($($val)*)),
            }
        ] ($($rest)*))
    };

    // The next value is a unit variant.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident :: $variant:ident $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!($name :: $variant),
            },
        ] ($($rest)*))
    };

    // The next field followed by a comma.
    (@fields [$($fields:expr,)*] ($key:ident : $val:expr , $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!($val)
            },
        ] ($($rest)*))
    };

    // The last field without a trailing comma.
    (@fields [$($fields:expr,)*] ($key:ident : $val:expr)) => {
        $crate::value!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value!($val),
            },
        ] ())
    };

    // Comma after the most recent field.
    (@fields [$($fields:expr),*] (, $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields),*] ($($rest)*))
    };

    // Comma after the most recent field.
    (@fields [$($fields:expr,)*] (, $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*] ($($rest)*))
    };

    ////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: value!($($tt)+)
    ////////////////////////////////////////////////////////////////////////

    // Booleans
    (false) => {
        $crate::recursive_comparison::value::Value::Bool(false)
    };
    (true) => {
        $crate::recursive_comparison::value::Value::Bool(true)
    };

    // Empty Seq: [ ]
    ([ ]) => {
        $crate::recursive_comparison::value::Value::Seq($crate::__private::vec![])
    };

    // Seq: [ 1, 2, 3 ]
    ([ $($tt:tt)+ ]) => {
        $crate::recursive_comparison::value::Value::Seq($crate::value!(@seq [] ($($tt)+)))
    };

    // Empty named struct: Foo {}
    ($name:ident { }) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: stringify!($name).into(),
            fields: $crate::__private::vec![],
        }
    };

    // Named struct: Foo { a: 1, b: 2 }
    ($name:ident { $($tt:tt)+ }) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: stringify!($name).into(),
            fields: $crate::value!(@fields [] ($($tt)+)),
        }
    };

    // Empty anonymous struct: { }
    ({ }) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: "".into(),
            fields: $crate::__private::vec![],
        }
    };

    // Anonymous struct: { a: 1, b: 2 }
    ({ $($tt:tt)+ }) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: "".into(),
            fields: $crate::value!(@fields [] ($($tt)+)),
        }
    };

    // Empty struct variant: Foo::Bar { }
    ($name:ident :: $variant:ident { }) => {
        $crate::recursive_comparison::value::Value::StructVariant {
            type_name: stringify!($name).into(),
            variant: stringify!($variant).into(),
            fields: $crate::__private::vec![],
        }
    };

    // Struct variant: Foo::Bar { a: 1, b: 2 }
    ($name:ident :: $variant:ident { $($tt:tt)+ }) => {
        $crate::recursive_comparison::value::Value::StructVariant {
            type_name: stringify!($name).into(),
            variant: stringify!($variant).into(),
            fields: $crate::value!(@fields [] ($($tt)+)),
        }
    };

    // Tuple Variant: Foo::Bar(1, 2)
    ($name:ident :: $variant:ident ( $($tt:tt)+ )) => {
        $crate::recursive_comparison::value::tuple_variant(
            stringify!($name),
            stringify!($variant),
            $crate::value!(@seq [] ($($tt)+))
        )
    };

    // Unit Variant: Foo::Bar
    ($name:ident :: $variant:ident) => {
        $crate::recursive_comparison::value::Value::UnitVariant {
            type_name: stringify!($name).into(),
            variant: stringify!($variant).into(),
        }
    };

    // Empty tuple struct: Foo()
    ($name:ident ( )) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: stringify!($name).into(),
            fields: $crate::__private::vec![],
        }
    };

    // Tuple struct: Foo(1, 2)
    ($name:ident ( $($tt:tt)+ )) => {
        $crate::recursive_comparison::value::tuple_struct(
            stringify!($name),
            $crate::value!(@seq [] ($($tt)+))
        )
    };

    // Unit Struct: Foo
    ($name:ident) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: stringify!($name).into(),
            fields: $crate::__private::vec![],
        }
    };

    // Empty Tuple or Unit: ()
    (()) => {
        $crate::recursive_comparison::value::Value::Unit
    };

    // Tuple: (1, 2)
    (( $($tt:tt)+ )) => {
        $crate::recursive_comparison::value::tuple($crate::value!(@seq [] ($($tt)+)))
    };

    // Any Serialize type: numbers, strings, chars, variables, etc.
    // Must be below every other rule!
    ($val:expr) => {
        $crate::recursive_comparison::serialize::to_recursive_value(&$val)
            .unwrap_or_else(|err| panic!("failed to serialize expression: {err}"))
    };
}

#[cfg(test)]
mod tests;
