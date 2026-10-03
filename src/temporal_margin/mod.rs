//! Defines common data types used for asserting temporal types provided by the
//! supported crates.

use crate::std::{
    fmt,
    fmt::{Debug, Display},
};
use std::cmp::Ordering;
use std::time::Duration;

/// Represents a temporal margin with various time units.
#[derive(Clone, Copy)]
pub enum TemporalMargin {
    /// A temporal margin of nanoseconds.
    NanoSeconds(i32),
    /// A temporal margin of microseconds.
    MicroSeconds(i32),
    /// A temporal margin of milliseconds.
    MilliSeconds(i32),
    /// A temporal margin of seconds.
    Seconds(i32),
    /// A temporal margin of minutes.
    Minutes(i32),
    /// A temporal margin of hours.
    Hours(i32),
    /// A temporal margin of days.
    Days(i32),
    /// A temporal margin of weeks.
    Weeks(i32),
}

impl Default for TemporalMargin {
    fn default() -> Self {
        Self::MilliSeconds(0)
    }
}

impl Debug for TemporalMargin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for TemporalMargin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NanoSeconds(nanoseconds) => write!(f, "{nanoseconds}ns"),
            Self::MicroSeconds(microseconds) => write!(f, "{microseconds}µs"),
            Self::MilliSeconds(milliseconds) => write!(f, "{milliseconds}ms"),
            Self::Seconds(seconds) => write!(f, "{seconds}s"),
            Self::Minutes(minutes) => write!(f, "{minutes}m"),
            Self::Hours(hours) => write!(f, "{hours}h"),
            Self::Days(days) => write!(f, "{days}d"),
            Self::Weeks(weeks) => write!(f, "{weeks}w"),
        }
    }
}

impl PartialEq for TemporalMargin {
    fn eq(&self, other: &Self) -> bool {
        self.to_nanoseconds() == other.to_nanoseconds()
    }
}

impl Eq for TemporalMargin {}

impl PartialOrd for TemporalMargin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TemporalMargin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_nanoseconds().cmp(&other.to_nanoseconds())
    }
}

impl TemporalMargin {
    fn to_nanoseconds(self) -> i64 {
        match self {
            Self::NanoSeconds(nanos) => i64::from(nanos),
            Self::MicroSeconds(micros) => i64::from(micros) * 1000,
            Self::MilliSeconds(millis) => i64::from(millis) * 1000 * 1000,
            Self::Seconds(secs) => i64::from(secs) * 1000 * 1000 * 1000,
            Self::Minutes(mins) => i64::from(mins) * 60 * 1000 * 1000 * 1000,
            Self::Hours(hours) => i64::from(hours) * 60 * 60 * 1000 * 1000 * 1000,
            Self::Days(days) => i64::from(days) * 24 * 60 * 60 * 1000 * 1000 * 1000,
            Self::Weeks(weeks) => i64::from(weeks) * 7 * 24 * 60 * 60 * 1000 * 1000 * 1000,
        }
    }
}

/// Conversion to a temporal duration type `T`.
///
/// This trait is used to convert a [`TemporalMargin`] to the various temporal
/// duration types of foreign crates, like `chrono::TimeSpan`,
/// `jiff::SignedDuration` or `time::SignedDuration`.
pub trait ToDuration<T> {
    /// Converts Self into a temporal duration `T`.
    ///
    /// # Panics
    ///
    /// Implementations may panic if the conversion is not possible, e.g.,
    /// a value overflows a boundary.
    fn to_duration(self) -> T;
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl ToDuration<Duration> for TemporalMargin {
    fn to_duration(self) -> Duration {
        Duration::from_nanos(self.to_nanoseconds() as u64)
    }
}

/// An extension trait for constructing [`TemporalMargin`]s from numeric values
/// using a postfix notation.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
/// use asserting::temporal_margin::TemporalMargin;
///
/// assert_eq!(10.milliseconds(), TemporalMargin::MilliSeconds(10));
/// assert_eq!(22.seconds(), TemporalMargin::Seconds(22));
/// assert_eq!(60.minutes(), TemporalMargin::Minutes(60));
/// assert_eq!(5.days(), TemporalMargin::Days(5));
/// ```
pub trait ToTemporalMargin {
    /// Constructs a [`TemporalMargin`] of nanoseconds.
    ///
    /// # Panics
    ///
    /// An implementation may panic if the numeric value cannot be converted
    /// to an `i32`.
    fn nanoseconds(self) -> TemporalMargin;
    /// Constructs a [`TemporalMargin`] of microseconds.
    ///
    /// # Panics
    ///
    /// An implementation may panic if the numeric value cannot be converted
    /// to an `i32`.
    fn microseconds(self) -> TemporalMargin;
    /// Constructs a [`TemporalMargin`] of milliseconds.
    ///
    /// # Panics
    ///
    /// An implementation may panic if the numeric value cannot be converted
    /// to an `i32`.
    fn milliseconds(self) -> TemporalMargin;
    /// Constructs a [`TemporalMargin`] of seconds.
    ///
    /// # Panics
    ///
    /// An implementation may panic if the numeric value cannot be converted
    /// to an `i32`.
    fn seconds(self) -> TemporalMargin;
    /// Constructs a [`TemporalMargin`] of minutes.
    ///
    /// # Panics
    ///
    /// An implementation may panic if the numeric value cannot be converted
    /// to an `i32`.
    fn minutes(self) -> TemporalMargin;
    /// Constructs a [`TemporalMargin`] of hours.
    ///
    /// # Panics
    ///
    /// An implementation may panic if the numeric value cannot be converted
    /// to an `i32`.
    fn hours(self) -> TemporalMargin;
    /// Constructs a [`TemporalMargin`] of days.
    ///
    /// # Panics
    ///
    /// An implementation may panic if the numeric value cannot be converted
    /// to an `i32`.
    fn days(self) -> TemporalMargin;
    /// Constructs a [`TemporalMargin`] of weeks.
    ///
    /// # Panics
    ///
    /// An implementation may panic if the numeric value cannot be converted
    /// to an `i32`.
    fn weeks(self) -> TemporalMargin;
}

impl ToTemporalMargin for i32 {
    fn nanoseconds(self) -> TemporalMargin {
        TemporalMargin::NanoSeconds(self)
    }
    fn microseconds(self) -> TemporalMargin {
        TemporalMargin::MicroSeconds(self)
    }
    fn milliseconds(self) -> TemporalMargin {
        TemporalMargin::MilliSeconds(self)
    }
    fn seconds(self) -> TemporalMargin {
        TemporalMargin::Seconds(self)
    }
    fn minutes(self) -> TemporalMargin {
        TemporalMargin::Minutes(self)
    }
    fn hours(self) -> TemporalMargin {
        TemporalMargin::Hours(self)
    }
    fn days(self) -> TemporalMargin {
        TemporalMargin::Days(self)
    }
    fn weeks(self) -> TemporalMargin {
        TemporalMargin::Weeks(self)
    }
}

#[cfg(test)]
mod tests;
