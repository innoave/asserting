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
/// Literals for bool, char, number, and strings can be written similar to Rust
/// literals with some minor differences: a `&str` does not have to be converted
/// to a `String` (this is done automatically), and brackets are used for
/// sequences, not arrays. Each number literal should contain the type, e.g.,
/// `42_u64`, `1.2_f32`, etc. This is necessary because the macro cannot infer
/// the type of number literals.
///
/// The following example gives an overview of the syntax, with elements of
/// various types.
///
/// ```
/// use asserting::prelude::*;
///
/// let value = value!({                          // a struct
///     foo: 2.3_f64,                             // a float literal
///     bar: {                                    // an embedded struct
///         baz: "alpha",                         // a string literal
///         qux: 123_i16,                         // an integer literal
///         corge: true,                          // a boolean literal
///     },
///     grault: Sample::Two("beta", -456_i64),    // a tuple variant
///     waldo: (123_u8, 234_u8, 56_u8),           // a tuple
///     fred: ["alpha", "beta", "gamma"],         // a sequence
///     corge: #{ 'a' => 1, 'b' => 2, 'c' => 3},  // a map
///     thud: Named(0.8_f32),                     // a tuple struct
/// });
/// ```
///
/// Variables in scope can be referenced inside the `value!`-macro.
///
/// ```
/// use asserting::prelude::*;
///
/// let one = 1;
/// let two = 2;
/// let three = 3;
///
/// let value = value!([one, two, three]);
///
/// assert_eq!(format!("{value:?}"), "[1, 2, 3]");
/// ```
///
/// Expressions can be used inside the `value!`-macro as well:
///
/// ```
/// use asserting::prelude::*;
///
/// let value = value!(Sum(13_i16 + 17_i16));
///
/// assert_eq!(format!("{value:?}"), "Sum(30)");
/// ```
///
/// ## Structs
///
/// Structs can be constructed on the fly, without prior declaration of a type.
/// In `asserting` they are called "anonymous structs". The name of a struct
/// can be omitted.
///
/// ```
/// # use asserting::prelude::*;
/// #
/// let value = value!({
///     name: "Silvia",
///     age: 25_u8,
/// });
/// ```
///
/// The name of a struct to be constructed can be specified as by the usual
/// syntax in Rust.
///
/// ```
/// # use asserting::prelude::*;
/// #
/// let value = value!(Person {
///     name: "Silvia",
///     age: 25_u8,
/// });
/// ```
///
/// Note: The name of a struct is not compared in the field-by-field recursive
/// comparison mode.
///
/// ## Tuples
///
/// A tuple is constructed using parenthesis as in plain Rust.
///
/// ```
/// # use asserting::prelude::*;
/// #
/// let value = value!((42_u64, "alpha", true));
/// ```
///
/// ## Enums-Variants
///
/// Example for constructing a value of unit variant:
///
/// ```
/// # use asserting::prelude::*;
/// #
/// let value = value!(Foo::Bar);
/// ```
///
/// Example for constructing a value of tuple variant:
///
/// ```
/// # use asserting::prelude::*;
/// #
/// let value = value!(Foo::Bar(-1.3_f32));
/// ```
///
/// Example for constructing a value of struct variant:
///
/// ```
/// # use asserting::prelude::*;
/// #
/// let value = value!(Foo::Bar { left: "alpha", right: -123_i16 });
/// ```
///
/// ## Sequences
///
/// A sequence is constructed by enclosing a list of values inside brackets.
/// In the following example we construct a sequence of chars.
///
/// ```
/// # use asserting::prelude::*;
/// #
/// let value = value!(['a', 'b', 'c']);
/// ```
///
/// ## Maps
///
/// A map starts with `#{` and ends with `}`. An association between a key and
/// a value is separated by `=>`. Multiple key/value-pairs are separated by `,`.
///
/// ```
/// # use asserting::prelude::*;
/// #
/// let value = value!(#{
///     'a' => 1,
///     'b' => 2,
///     'c' => 3,
/// });
/// ```
///
/// ## Primitive types
///
/// | Type     | Example           |
/// |----------|-------------------|
/// | `bool`   | `true` or `false` |
/// | `char`   | `'a'`             |
/// | `f32`    | `1.2_f32`         |
/// | `f64`    | `1.2_f64`         |
/// | `str`    | `"alpha"`         |
/// | `String` | `"alpha"`         |
/// | `i8`     | `-12_i8`          |
/// | `i16`    | `-12_i16`         |
/// | `i32`    | `-12_i32`         |
/// | `i64`    | `-12_i64`         |
/// | `i128`   | `-12_i128`        |
/// | `u8`     | `12_i8`           |
/// | `u16`    | `12_i16`          |
/// | `u32`    | `12_i32`          |
/// | `u64`    | `12_i64`          |
/// | `u128`   | `12_i128`         |
///
/// Note: `isize` and `usize` are not supported by `serde`. Therefore, `isize`
/// values are converted to i64 or i128 and `usize` values are converted to
/// `u64` or `u128`.
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
/// using the normal (verbose) syntax works:
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
    // hide distracting implementation details from the generated rustdoc.
    ($($value:tt)+) => {
        $crate::value_impl!($($value)+)
    };
}

/// DO NOT RELY ON THIS MACRO AS IT MAY CHANGE WITHOUT NOTICE!
#[macro_export]
#[doc(hidden)]
macro_rules! value_impl {
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
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!([$($val)*]),] ($($rest)*))
    };

    // The next element is a map.
    (@seq [$($elems:expr,)*] (#{ $($val:tt)* } $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!(#{$($val)*}),] ($($rest)*))
    };

    // The next element is an anonymous struct.
    (@seq [$($elems:expr,)*] ({ $($val:tt)* } $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!({$($val)*}),] ($($rest)*))
    };

    // The next element is a named struct.
    (@seq [$($elems:expr,)*] ($name:ident { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!($name {$($val)*}),] ($($rest)*))
    };

    // The next element is a tuple struct.
    (@seq [$($elems:expr,)*] ($name:ident ( $($val:tt)* ) $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!($name ($($val)*)),] ($($rest)*))
    };

    // The next element is a struct variant.
    (@seq [$($elems:expr,)*] ($name:ident :: $variant:ident { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!($name :: $variant {$($val)*}),] ($($rest)*))
    };

    // The next element is a tuple variant.
    (@seq [$($elems:expr,)*] ($name:ident :: $variant:ident ( $($val:tt)* ) $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!($name :: $variant ($($val)*)),] ($($rest)*))
    };

    // The next element is a unit variant.
    (@seq [$($elems:expr,)*] ($name:ident :: $variant:ident $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!($name :: $variant),] ($($rest)*))
    };

    // The next element is an expression followed by a comma.
    (@seq [$($elems:expr,)*] ($next:expr , $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!($next),] ($($rest)*))
    };

    // The last element is an expression with no trailing comma.
    (@seq [$($elems:expr,)*] ($last:expr)) => {
        $crate::value_impl!(@seq [$($elems,)* $crate::value_impl!($last),] ())
    };

    // Comma after the most recent element.
    (@seq [$($elems:expr,)*] (, $($rest:tt)*)) => {
        $crate::value_impl!(@seq [$($elems,)*] ($($rest)*))
    };

    ////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the fields of a struct {...}.
    //
    // Must be invoked as: value!(@fields [] ($($tt)*))
    // It returns a 'Vec<Field>'.
    ////////////////////////////////////////////////////////////////////////

    // Done with a trailing comma.
    (@fields [$($fields:expr,)*] ()) => {
        $crate::__private::vec![$($fields,)*]
    };

    // Done without a trailing comma.
    (@fields [$($fields:expr),*] ()) => {
        $crate::__private::vec![$($fields),*]
    };

    // The next value is a seq.
    (@fields [$($fields:expr,)*] ($key:ident : [ $($val:tt)* ] $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!([$($val)*]),
            },
        ] ($($rest)*))
    };

    // The next value is a map.
    (@fields [$($fields:expr,)*] ($key:ident : #{ $($val:tt)* } $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!(#{$($val)*}),
            },
        ] ($($rest)*))
    };

    // The next value is a named struct.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!($name { $($val)* }),
            },
        ] ($($rest)*))
    };

    // The next value is an anonymous struct.
    (@fields [$($fields:expr,)*] ($key:ident : { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!({$($val)*}),
            },
        ] ($($rest)*))
    };

    // The next value is a tuple struct.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident ( $($val:tt)* ) $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!($name ($($val)*)),
            },
        ] ($($rest)*))
    };

    // The next value is a struct variant.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident :: $variant:ident { $($val:tt)* } $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!($name :: $variant {$($val)*}),
            },
        ] ($($rest)*))
    };

    // The next value is a tuple variant.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident :: $variant:ident ( $($val:tt)* ) $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!($name :: $variant ($($val)*)),
            },
        ] ($($rest)*))
    };

    // The next value is a unit variant.
    (@fields [$($fields:expr,)*] ($key:ident : $name:ident :: $variant:ident $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!($name :: $variant),
            },
        ] ($($rest)*))
    };

    // The next field followed by a comma.
    (@fields [$($fields:expr,)*] ($key:ident : $val:expr , $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!($val)
            },
        ] ($($rest)*))
    };

    // The last field without a trailing comma.
    (@fields [$($fields:expr,)*] ($key:ident : $val:expr)) => {
        $crate::value_impl!(@fields [$($fields,)*
            $crate::recursive_comparison::value::Field {
                name: stringify!($key).into(),
                value: $crate::value_impl!($val),
            },
        ] ())
    };

    // Comma after the most recent field.
    (@fields [$($fields:expr,)*] (, $($rest:tt)*)) => {
        $crate::value_impl!(@fields [$($fields,)*] ($($rest)*))
    };

    ////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the values of a map #{...}.
    //
    // Must be invoked as: value!(@map [] () ($($tt)*))
    // It returns a 'Vec<(Value, Value)>`.
    ////////////////////////////////////////////////////////////////////////

    // Done
    (@map [$($pairs:expr,)*] () ()) => {
        $crate::__private::vec![$($pairs,)*]
    };

    // The key is finished, start parsing the value.
    (@map [$($pairs:expr,)*] ($($key:tt)+) (=> $($rest:tt)*)) => {
        $crate::value_impl!(@map_val [$($pairs,)*] ($($key)+) () ($($rest)*))
    };

    // Munch the next token for the key.
    (@map [$($pairs:expr,)*] ($($key:tt)*) ($next:tt $($rest:tt)*)) => {
        $crate::value_impl!(@map [$($pairs,)*] ($($key)* $next) ($($rest)*))
    };

    // The value is finished by a comma, start parsing the next entry.
    (@map_val [$($pairs:expr,)*] ($($key:tt)+) ($($val:tt)+) (, $($rest:tt)*)) => {
        $crate::value_impl!(@map [$($pairs,)* ($crate::value_impl!($($key)+), $crate::value_impl!($($val)+)),] () ($($rest)*))
    };

    // The value is finished (end of tokens).
    (@map_val [$($pairs:expr,)*] ($($key:tt)+) ($($val:tt)+) ()) => {
        $crate::value_impl!(@map [$($pairs,)* ($crate::value_impl!($($key)+), $crate::value_impl!($($val)+)),] () ())
    };

    // Munch the next token for the value.
    (@map_val [$($pairs:expr,)*] ($($key:tt)+) ($($val:tt)*) ($next:tt $($rest:tt)*)) => {
        $crate::value_impl!(@map_val [$($pairs,)*] ($($key)+) ($($val)* $next) ($($rest)*))
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
        $crate::recursive_comparison::value::Value::Seq($crate::value_impl!(@seq [] ($($tt)+)))
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
            fields: $crate::value_impl!(@fields [] ($($tt)+)),
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
            fields: $crate::value_impl!(@fields [] ($($tt)+)),
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
            fields: $crate::value_impl!(@fields [] ($($tt)+)),
        }
    };

    // Tuple Variant: Foo::Bar(1, 2)
    ($name:ident :: $variant:ident ( $($tt:tt)+ )) => {
        $crate::recursive_comparison::value::tuple_variant(
            stringify!($name),
            stringify!($variant),
            $crate::value_impl!(@seq [] ($($tt)+))
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
            $crate::value_impl!(@seq [] ($($tt)+))
        )
    };

    // Empty Map: #{ }
    (#{ }) => {
        $crate::recursive_comparison::value::Value::Map(
            $crate::recursive_comparison::value::Map::new()
        )
    };

    // Map: #{ a => 1, b => 2 }
    (#{ $($tt:tt)+ }) => {
        $crate::recursive_comparison::value::Value::Map(
            $crate::recursive_comparison::value::Map::from_iter(
                $crate::value_impl!(@map [] () ($($tt)+))
            )
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
        $crate::recursive_comparison::value::tuple($crate::value_impl!(@seq [] ($($tt)+)))
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
