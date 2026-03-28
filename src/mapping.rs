use crate::assertions::{AssertDebugString, AssertDisplayString};
use crate::spec::{FailingStrategy, Spec};
use crate::std::fmt::{Debug, Display};
use crate::std::format;
use crate::std::string::{String, ToString};

impl<'a, S, R> AssertDebugString for Spec<'a, S, R>
where
    S: Debug,
    R: FailingStrategy,
{
    type DebugString = Spec<'a, String, R>;

    fn debug_string(self) -> Self::DebugString {
        let expression_debug_string = format!("{}'s debug string", self.expression());
        self.mapping(|subject| format!("{subject:?}"))
            .named(expression_debug_string)
    }
}

impl<'a, S, R> AssertDisplayString for Spec<'a, S, R>
where
    S: Display,
    R: FailingStrategy,
{
    type DisplayString = Spec<'a, String, R>;

    fn display_string(self) -> Self::DisplayString {
        let expression_display_string = format!("{}'s display string", self.expression());
        self.mapping(|subject| subject.to_string())
            .named(expression_display_string)
    }
}
