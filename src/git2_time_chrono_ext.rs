use crate::Error;
use chrono::TimeZone;

/// An extension trait to convert [`git2::Time`] to [`chrono::DateTime`].
/// # Examples
/// ```no_run
/// use git2_time_chrono_ext::{Error, Git2TimeChronoExt};
///
/// // Print `git2::Time` to `stdout`.
/// fn print_git2_time(time: &git2::Time) {
///   println!("{}", time.to_local_date_time().unwrap());
/// }
///
/// // Convert `git2::Time` to `Stirng` in the specified format.
/// fn git2_time_to_string(time: &git2::Time) -> String {
///   time.to_local_date_time().unwrap().format("%Y-%m-%d %H:%M").to_string()
/// }
///
/// // Convert `git2::Time` to ISO 8601 (RFC 3339) string.
/// fn git2_time_to_rfc3339(time: &git2::Time) -> Result<String, Error> {
///   Ok(time.to_date_time_in(&chrono::Utc)?.to_rfc3339())
/// }
/// ```
pub trait Git2TimeChronoExt {
    /// Convert [`git2::Time`] to [`chrono::DateTime`]
    /// retaining the original timezone.
    /// # Examples
    /// ```
    /// use git2_time_chrono_ext::Git2TimeChronoExt;
    ///
    /// // The Eastern Hemisphere time zone.
    /// let east_time = git2::Time::new(1745693791, 540);
    /// let east_datetime = east_time.to_date_time();
    /// assert!(east_datetime.is_ok());
    /// assert_eq!(east_datetime.unwrap().to_string(), "2025-04-27 03:56:31 +09:00");
    /// ```
    /// ```
    /// # use git2_time_chrono_ext::Git2TimeChronoExt;
    /// // The Western Hemisphere time zone.
    /// let west_time = git2::Time::new(1745196130, -420);
    /// let west_datetime = west_time.to_date_time();
    /// assert!(west_datetime.is_ok());
    /// assert_eq!(west_datetime.unwrap().to_string(), "2025-04-20 17:42:10 -07:00");
    /// ```
    fn to_date_time(&self) -> Result<chrono::DateTime<chrono::FixedOffset>, Error>;

    /// Convert [`git2::Time`] to [`chrono::DateTime`]
    /// in the specified [`chrono::TimeZone`].
    /// # Examples
    /// ```
    /// use git2_time_chrono_ext::Git2TimeChronoExt;
    ///
    /// let time = git2::Time::new(1745196130, -420);
    /// let utc_datetime = time.to_date_time_in(&chrono::Utc);
    /// assert_eq!(utc_datetime.unwrap().to_string(), "2025-04-21 00:42:10 UTC");
    /// ```
    fn to_date_time_in<Tz: chrono::TimeZone>(&self, tz: &Tz)
    -> Result<chrono::DateTime<Tz>, Error>;

    /// Convert [`git2::Time`] to [`chrono::DateTime`] in the local time zone.
    /// This function is a shorthand of:
    /// ```
    /// # use git2_time_chrono_ext::Git2TimeChronoExt;
    /// # fn to_local(time: git2::Time) -> Result<chrono::DateTime<chrono::Local>, git2_time_chrono_ext::Error> {
    /// time.to_date_time_in(&chrono::Local)
    /// # }
    /// ```
    fn to_local_date_time(&self) -> Result<chrono::DateTime<chrono::Local>, Error>;

    /// [`to_date_time`][Git2TimeChronoExt::to_date_time] returns
    /// the latest time when the given time is ambiguous.
    ///
    /// This function is useful when you want to handle ambiguous time.
    /// Please see [`chrono::MappedLocalTime`] for more details.
    /// # Examples
    /// ```
    /// use git2_time_chrono_ext::Git2TimeChronoExt;
    /// use chrono::MappedLocalTime;
    ///
    /// let time = git2::Time::new(1745196130, -420);
    /// let mapped = time.to_date_time_opt().unwrap();
    /// if let MappedLocalTime::Single(datetime) = mapped {
    ///     assert_eq!(datetime.to_string(), "2025-04-20 17:42:10 -07:00");
    /// } else {
    ///     panic!("should be Single");
    /// }
    /// ```
    fn to_date_time_opt(
        &self,
    ) -> Result<chrono::MappedLocalTime<chrono::DateTime<chrono::FixedOffset>>, Error>;
}

impl Git2TimeChronoExt for git2::Time {
    fn to_date_time_opt(
        &self,
    ) -> Result<chrono::MappedLocalTime<chrono::DateTime<chrono::FixedOffset>>, Error> {
        let Some(tz) = chrono::FixedOffset::east_opt(self.offset_minutes() * 60) else {
            return Err(Error::InvalidTimeZone {
                offset_minutes: self.offset_minutes(),
            });
        };
        Ok(tz.timestamp_opt(self.seconds(), 0))
    }

    fn to_date_time(&self) -> Result<chrono::DateTime<chrono::FixedOffset>, Error> {
        match self.to_date_time_opt()? {
            chrono::MappedLocalTime::Single(datetime) => Ok(datetime),
            chrono::MappedLocalTime::Ambiguous(_, latest) => Ok(latest),
            chrono::MappedLocalTime::None => Err(Error::TimeNotMappable { time: *self }),
        }
    }

    fn to_date_time_in<Tz: chrono::TimeZone>(
        &self,
        tz: &Tz,
    ) -> Result<chrono::DateTime<Tz>, Error> {
        self.to_date_time()
            .map(|datetime| datetime.with_timezone(tz))
    }

    fn to_local_date_time(&self) -> Result<chrono::DateTime<chrono::Local>, Error> {
        self.to_date_time_in(&chrono::Local)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_date_time_offset_invalid() {
        let time = git2::Time::new(0, 100_000);
        let result = time.to_date_time();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            Error::InvalidTimeZone {
                offset_minutes: 100_000
            }
        ));
    }

    #[test]
    fn to_date_time_not_mappable() {
        let time = git2::Time::new(i64::MAX, 0);
        let result = time.to_date_time();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::TimeNotMappable { .. }));
    }

    #[test]
    fn to_date_time_opt_none() {
        use chrono::MappedLocalTime;
        let time = git2::Time::new(i64::MAX, 0);
        let mapped = time.to_date_time_opt().unwrap();
        assert!(matches!(mapped, MappedLocalTime::None));
    }
}
