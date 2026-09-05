//! Implementation of assertions for `Result` values.

use crate::assertions::{
    AssertHasError, AssertHasErrorMessage, AssertHasValue, AssertResult, AssertResultValue,
};
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    HasError, HasValue, IsErr, IsOk, has_error, has_value, is_equal_to, is_err, is_ok,
};
use crate::spec::{
    DebugRepresentation, DiffFormat, DisplayRepresentation, Expectation, Expecting, Expression,
    FailingStrategy, Invertible, Represent, Represented, RepresentedBy, Spec,
};
use crate::std::fmt::{Debug, Display};
use crate::std::{
    format,
    string::{String, ToString},
};

impl<T, E, D, R> AssertResult for Spec<'_, Result<T, E>, D, R>
where
    D: Represent<T> + Represent<E>,
    R: FailingStrategy,
{
    fn is_ok(self) -> Self {
        self.expecting(is_ok())
    }

    fn is_err(self) -> Self {
        self.expecting(is_err())
    }
}

impl<T, E, D, R> AssertResult for Spec<'_, &Result<T, E>, D, R>
where
    D: Represent<T> + Represent<E>,
    R: FailingStrategy,
{
    fn is_ok(self) -> Self {
        self.expecting(is_ok())
    }

    fn is_err(self) -> Self {
        self.expecting(is_err())
    }
}

impl<'a, T, E, D, R> AssertResultValue for Spec<'a, Result<T, E>, D, R>
where
    D: Represent<T> + Represent<E> + Clone,
{
    type Ok = Spec<'a, T, D, R>;
    type Err = Spec<'a, E, D, R>;

    fn ok(self) -> Self::Ok {
        let representation = self.representation().clone();
        self.mapping(|subject| match subject {
            Ok(value) => value,
            Err(error) => {
                let error = Represented::from((&error, &representation));
                panic!("expected the subject to be `Ok(_)`, but was `Err({error:?})`")
            },
        })
        .represented_by(representation)
    }

    fn err(self) -> Self::Err {
        let representation = self.representation().clone();
        self.mapping(|subject| match subject {
            Ok(value) => {
                let value = Represented::from((&value, &representation));
                panic!("expected the subject to be `Err(_)`, but was `Ok({value:?})`")
            },
            Err(error) => error,
        })
        .represented_by(representation)
    }
}

impl<'a, T, E, D, R> AssertResultValue for Spec<'a, &'a Result<T, E>, D, R>
where
    D: Represent<T> + Represent<E> + Clone,
{
    type Ok = Spec<'a, &'a T, D, R>;
    type Err = Spec<'a, &'a E, D, R>;

    fn ok(self) -> Self::Ok {
        let representation = self.representation().clone();
        self.mapping(|subject| match subject {
            Ok(value) => value,
            Err(error) => {
                let error = Represented::from((error, &representation));
                panic!("expected the subject to be `Ok(_)`, but was `Err({error:?})`")
            },
        })
        .represented_by(representation)
    }

    fn err(self) -> Self::Err {
        let representation = self.representation().clone();
        self.mapping(|subject| match subject {
            Ok(value) => {
                let value = Represented::from((value, &representation));
                panic!("expected the subject to be `Err(_)`, but was `Ok({value:?})`")
            },
            Err(error) => error,
        })
        .represented_by(representation)
    }
}

impl<T, E, X, D, R> AssertHasValue<X> for Spec<'_, Result<T, E>, D, R>
where
    T: PartialEq<X>,
    D: Represent<T> + Represent<E> + Represent<X>,
    R: FailingStrategy,
{
    fn has_value(self, expected: X) -> Self {
        self.expecting(has_value(expected))
    }
}

impl<T, E, X, D, R> AssertHasValue<X> for Spec<'_, &Result<T, E>, D, R>
where
    T: PartialEq<X>,
    D: Represent<T> + Represent<E> + Represent<X>,
    R: FailingStrategy,
{
    fn has_value(self, expected: X) -> Self {
        self.expecting(has_value(expected))
    }
}

impl<T, E, X, D, R> AssertHasError<X> for Spec<'_, Result<T, E>, D, R>
where
    E: PartialEq<X>,
    D: Represent<T> + Represent<E> + Represent<X>,
    R: FailingStrategy,
{
    fn has_error(self, expected: X) -> Self {
        self.expecting(has_error(expected))
    }
}

impl<T, E, X, D, R> AssertHasError<X> for Spec<'_, &Result<T, E>, D, R>
where
    E: PartialEq<X>,
    D: Represent<T> + Represent<E> + Represent<X>,
    R: FailingStrategy,
{
    fn has_error(self, expected: X) -> Self {
        self.expecting(has_error(expected))
    }
}

impl<'a, T, E, X, D, R> AssertHasErrorMessage<X> for Spec<'a, Result<T, E>, D, R>
where
    E: Display,
    X: Debug,
    String: PartialEq<X>,
    D: Represent<T> + Represent<E>,
    R: FailingStrategy,
{
    type ErrorMessage = Spec<'a, String, DebugRepresentation, R>;

    fn has_error_message(self, expected: X) -> Self::ErrorMessage {
        let subject = match self.subject() {
            Ok(value) => Ok(format!(
                "Ok({})",
                Represented::from((value, self.representation()))
            )),
            Err(error) => Err(error.to_string()),
        };
        self.mapping(|_result| match subject {
            Ok(value) => panic!(
                r"expected the subject to be `Err(_)` with message {expected:?}, but was `{value}`"
            ),
            Err(error) => error,
        })
        .expecting(is_equal_to(expected))
    }
}

impl<'a, T, E, X, D, R> AssertHasErrorMessage<X> for Spec<'a, &Result<T, E>, D, R>
where
    E: Display,
    X: Debug,
    String: PartialEq<X>,
    D: Represent<T>,
    R: FailingStrategy,
{
    type ErrorMessage = Spec<'a, String, DebugRepresentation, R>;

    fn has_error_message(self, expected: X) -> Self::ErrorMessage {
        let subject = match self.subject() {
            Ok(value) => Ok(format!(
                "Ok({:?})",
                Represented::from((value, self.representation()))
            )),
            Err(error) => Err(error.to_string()),
        };
        self.mapping(|_result| match subject {
            Ok(value) => panic!(
                r"expected the subject to be `Err(_)` with message {expected:?}, but was `{value}`"
            ),
            Err(error) => error,
        })
        .expecting(is_equal_to(expected))
    }
}

impl<T, E, D> Expectation<Result<T, E>, D> for IsOk
where
    D: Represent<T> + Represent<E>,
{
    fn test(&mut self, subject: &Result<T, E>) -> bool {
        subject.is_ok()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Result<T, E>,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = match actual {
            Ok(value) => mark_unexpected(
                &format!("Ok({})", Represented::from((value, representation))),
                &DisplayRepresentation,
                format,
            ),
            Err(error) => mark_unexpected(
                &format!("Err({})", Represented::from((error, representation))),
                &DisplayRepresentation,
                format,
            ),
        };
        let marked_expected = mark_missing(&"Ok(_)", &DisplayRepresentation, format);
        format!(
            "expected {expression} to be Ok(_)\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T, E, D> Expectation<Result<T, E>, D> for IsErr
where
    D: Represent<T> + Represent<E>,
{
    fn test(&mut self, subject: &Result<T, E>) -> bool {
        subject.is_err()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Result<T, E>,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = match actual {
            Ok(value) => mark_unexpected(
                &format!("Ok({})", Represented::from((value, representation))),
                &DisplayRepresentation,
                format,
            ),
            Err(error) => mark_unexpected(
                &format!("Err({})", Represented::from((error, representation))),
                &DisplayRepresentation,
                format,
            ),
        };
        let marked_expected = mark_missing(&"Err(_)", &DisplayRepresentation, format);
        format!(
            "expected {expression} to be Err(_)\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T, E, D> Expectation<&Result<T, E>, D> for IsOk
where
    D: Represent<T> + Represent<E>,
{
    fn test(&mut self, subject: &&Result<T, E>) -> bool {
        <Self as Expectation<Result<T, E>, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&Result<T, E>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<Result<T, E>, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<T, E, D> Expectation<&Result<T, E>, D> for IsErr
where
    D: Represent<T> + Represent<E>,
{
    fn test(&mut self, subject: &&Result<T, E>) -> bool {
        <Self as Expectation<Result<T, E>, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&Result<T, E>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<Result<T, E>, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<T, E, X, D> Expectation<Result<T, E>, D> for HasValue<X>
where
    T: PartialEq<X>,
    D: Represent<T> + Represent<E> + Represent<X>,
{
    fn test(&mut self, subject: &Result<T, E>) -> bool {
        subject.as_ref().is_ok_and(|value| value == &self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Result<T, E>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected = &self.expected;
        let marked_actual = match actual {
            Ok(value) => mark_unexpected(
                &format!("Ok({:?})", Represented::from((value, representation))),
                &DisplayRepresentation,
                format,
            ),
            Err(error) => mark_unexpected(
                &format!("Err({:?})", Represented::from((error, representation))),
                &DisplayRepresentation,
                format,
            ),
        };
        let marked_expected = mark_missing(
            &format!("Ok({:?})", Represented::from((expected, representation))),
            &DisplayRepresentation,
            format,
        );
        let represented_expected = Represented::from((expected, representation));
        format!(
            "expected {expression} to be ok {not}containing {represented_expected:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}"
        )
    }
}

impl<T, E, X, D> Expectation<&Result<T, E>, D> for HasValue<X>
where
    T: PartialEq<X>,
    D: Represent<T> + Represent<E> + Represent<X>,
{
    fn test(&mut self, subject: &&Result<T, E>) -> bool {
        <Self as Expectation<Result<T, E>, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&Result<T, E>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<Result<T, E>, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<T, E, X, D> Expectation<Result<T, E>, D> for HasError<X>
where
    E: PartialEq<X>,
    D: Represent<T> + Represent<E> + Represent<X>,
{
    fn test(&mut self, subject: &Result<T, E>) -> bool {
        subject.as_ref().is_err_and(|err| err == &self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Result<T, E>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected = &self.expected;
        let marked_actual = match actual {
            Ok(value) => mark_unexpected(
                &format!("Ok({:?})", Represented::from((value, representation))),
                &DisplayRepresentation,
                format,
            ),
            Err(error) => mark_unexpected(
                &format!("Err({:?})", Represented::from((error, representation))),
                &DisplayRepresentation,
                format,
            ),
        };
        let marked_expected = mark_missing(
            &format!("Err({:?})", Represented::from((expected, representation))),
            &DisplayRepresentation,
            format,
        );
        let represented_expected = Represented::from((expected, representation));
        format!(
            "expected {expression} to be an error {not}containing {represented_expected:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}"
        )
    }
}

impl<X> Invertible for HasError<X> {}

impl<T, E, X, D> Expectation<&Result<T, E>, D> for HasError<X>
where
    E: PartialEq<X>,
    D: Represent<T> + Represent<E> + Represent<X>,
{
    fn test(&mut self, subject: &&Result<T, E>) -> bool {
        <Self as Expectation<Result<T, E>, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&Result<T, E>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<Result<T, E>, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

#[cfg(test)]
mod tests;
