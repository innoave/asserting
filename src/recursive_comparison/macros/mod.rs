/// Construct a [`Value`] from a Rust-like constructor expression.
///
/// With this macro it is possible to construct values that have the same
/// structure as actual types like structs, enums, tuples, or even primitive
/// types. It is not necessary to declare the type in advance.
///
/// # Limitations
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
        $crate::std::vec![$($elems,)*]
    };

    // Done without a trailing comma.
    (@seq [$($elems:expr),*] ()) => {
        $crate::std::vec![$($elems),*]
    };

    // Ignore leading commas.
    (@seq [$($elems:expr,)*] (, $($rest:tt)*)) => {
        $crate::value!(@seq [$($elems,)*] ($($rest)*))
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

    // Ignore leading commas.
    (@fields [$($fields:expr,)*] (, $($rest:tt)*)) => {
        $crate::value!(@fields [$($fields,)*] ($($rest)*))
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

    ////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: value!($($json)+)
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
        $crate::recursive_comparison::value::Value::Seq($crate::std::vec![])
    };

    // Seq: [ 1, 2, 3 ]
    ([ $($tt:tt)+ ]) => {
        $crate::recursive_comparison::value::Value::Seq($crate::value!(@seq [] ($($tt)+)))
    };

    // Empty named struct: Foo {}
    ($name:ident { }) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: stringify!($name).into(),
            fields: $crate::std::vec![],
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
            fields: $crate::std::vec![],
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
            fields: $crate::std::vec![],
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

    // Empty tuple struct: Foo()
    ($name:ident ( )) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: stringify!($name).into(),
            fields: $crate::std::vec![],
        }
    };

    // Tuple struct: Foo(1, 2)
    ($name:ident ( $($tt:tt)+ )) => {
        $crate::recursive_comparison::value::tuple_struct(
            stringify!($name),
            $crate::value!(@seq [] ($($tt)+))
        )
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

    // Unit Struct: Foo
    ($name:ident) => {
        $crate::recursive_comparison::value::Value::Struct {
            type_name: stringify!($name).into(),
            fields: $crate::std::vec![],
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
