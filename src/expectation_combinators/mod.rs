use crate::expectations::{All, IntoRec, Not, Rec};
use crate::spec::{DiffFormat, Expectation, Expression, Invertible};
use crate::std::string::String;

impl<S, E> Expectation<S> for Rec<E>
where
    E: Expectation<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        let result = self.expectation.test(subject);
        self.result = Some(result);
        result
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        if self.is_failure() {
            self.expectation
                .message(expression, actual, inverted, format)
                + "\n"
        } else {
            String::new()
        }
    }
}

impl<E> From<E> for Rec<E> {
    fn from(expectation: E) -> Self {
        Self::new(expectation)
    }
}

macro_rules! impl_into_rec_for_tuple {
    ( $( $tp_name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($tp_name: Into<Rec<$tp_name >>),+> IntoRec for ($($tp_name,)+) {
            type Output = ($(Rec<$tp_name>,)+);

            fn into_rec(self) -> Self::Output {
                let ($($tp_name,)+) = self;
                ($($tp_name.into(),)+)
            }
        }
    };
}

impl_into_rec_for_tuple! { A1 }
impl_into_rec_for_tuple! { A1 A2 }
impl_into_rec_for_tuple! { A1 A2 A3 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 A5 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 A5 A6 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 A5 A6 A7 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 A5 A6 A7 A8 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 A5 A6 A7 A8 A9 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 }
impl_into_rec_for_tuple! { A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 }

impl<S, E> Expectation<S> for Not<E>
where
    E: Invertible + Expectation<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        !self.0.test(subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        self.0.message(expression, actual, !inverted, format)
    }
}

macro_rules! impl_expectation_for_tuple_combinator {
    ( $combinator:ident: $( $tp_name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<S, $($tp_name: Expectation<S>),+> Expectation<S> for $combinator<($(Rec<$tp_name>,)+)> {
            fn test(&mut self, subject: &S) -> bool {
                let ($($tp_name,)+) = &mut self.0;
                $(
                    let $tp_name = $tp_name.test(subject);
                )+
                $( $tp_name )&&+
            }

            fn message(
                &self,
                expression: &Expression<'_>,
                actual: &S,
                inverted: bool,
                format: &DiffFormat,
            ) -> String {
                let ($($tp_name,)+) = &self.0;
                let mut message = String::new();
                $(
                    message.push_str(&$tp_name.message(expression, actual, inverted, format));
                )+
                message
            }
        }
    };
}

impl_expectation_for_tuple_combinator! { All: A1 }
impl_expectation_for_tuple_combinator! { All: A1 A2 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 A5 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 A5 A6 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 A5 A6 A7 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 A5 A6 A7 A8 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 A5 A6 A7 A8 A9 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 }
impl_expectation_for_tuple_combinator! { All: A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 }

#[cfg(test)]
mod tests;
