use crate::assertions::{AssertDebugString, AssertDisplayString};
use crate::spec::{FailingStrategy, Spec};
use crate::std::fmt::{Debug, Display};
use crate::std::format;
use crate::std::string::{String, ToString};

impl<'a, S, R> AssertDebugString<'a, R> for Spec<'a, S, R>
where
    S: Debug,
    R: FailingStrategy,
{
    fn debug_string(self) -> Spec<'a, String, R> {
        let expression_debug_string = format!("{}'s debug string", self.expression());
        self.mapping(|subject| format!("{subject:?}"))
            .named(expression_debug_string)
    }
}

impl<'a, S, R> AssertDisplayString<'a, R> for Spec<'a, S, R>
where
    S: Display,
    R: FailingStrategy,
{
    fn display_string(self) -> Spec<'a, String, R> {
        let expression_display_string = format!("{}'s display string", self.expression());
        self.mapping(|subject| subject.to_string())
            .named(expression_display_string)
    }
}
