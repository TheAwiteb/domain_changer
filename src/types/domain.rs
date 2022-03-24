//     Rust library that helps you change the domain of the link to another domain
//      Copyright (C) 2022  TheAwiteb <awiteb@hotmail.com>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, version 3 of the License
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License along
// with this program.  If not, see <http://www.gnu.org/licenses/>.

use super::errors::{DomainChangerError, DomainChangerResult};
#[cfg(feature = "json")]
use super::ToFromJson;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use url::Url;

/// [`Domain`] struct help you to put [`old`] and [`new`] domain
///
/// [`old`] is old domain you want to chang it to [`new`]
///
/// [`new`]: Domain#structfield.new
/// [`old`]: Domain#structfield.old
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Domain {
    /// old domain to change it
    pub old: Url,
    /// new domain you want change to it
    pub new: Url,
}

impl Domain {
    /// Create new [`Domain`] instance
    /// # Example
    /// ```rust
    /// use domain_changer::types::Domain;
    /// use url::Url;
    ///
    /// let foo: Domain = Domain::new(
    ///     Url::parse("https://youtube.com").unwrap(),
    ///     Url::parse("https://piped.kavin.rocks").unwrap(),
    /// );
    ///
    /// assert_eq!(foo.new.domain(), Some("piped.kavin.rocks"));
    /// assert_eq!(foo.old.domain(), Some("youtube.com"));
    /// ```
    pub fn new(old: Url, new: Url) -> Self {
        Self { old, new }
    }

    /// Returns the [`Url`] if `word` is url and is in the domain
    /// check [`old`] only if `just_old` is [`true`]
    ///
    /// # Example
    /// ```rust
    /// use domain_changer::types::Domain;
    /// use url::Url;
    ///
    /// let domain: Domain = Domain::new(
    ///     Url::parse("https://youtube.com").unwrap(),
    ///     Url::parse("https://piped.kavin.rocks").unwrap(),
    /// );
    ///
    /// assert!(domain.contain("youtube.com", true).is_some());
    /// assert!(domain.contain("piped.kavin.rocks", true).is_none());
    /// assert!(domain.contain("piped.kavin.rocks", false).is_some());
    /// ```
    ///
    /// [`old`]: Domain#structfield.old
    pub fn contain(&self, word: &str, just_old: bool) -> Option<Url> {
        let host_with_https: String = "https://".to_owned() + word;
        if let Ok(url) = Url::parse(
            if word.starts_with("https://") || word.starts_with("http://") {
                word
            } else {
                &host_with_https
            },
        ) {
            if let Some(str_host) = url.host_str() {
                if self.old.has_host() && (self.old.host_str().unwrap() == str_host)
                    || !just_old
                        && self.new.has_host()
                        && (self.new.host_str().unwrap() == str_host)
                {
                    return Some(url);
                }
            }
        }
        None
    }
}

/// Implementation [`ToFromJson`] to [`Domain`]
///
/// # Example
/// ```rust
/// use domain_changer::types::{Domain, ToFromJson};
///
/// let domain: Domain = Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap();
/// assert_eq!(domain.to_json().unwrap(), "{\"old\":\"https://twitter.com/\",\"new\":\"https://nitter.net/\"}");
/// assert_eq!(Domain::from_json("{\"old\":\"https://twitter.com/\",\"new\":\"https://nitter.net/\"}").unwrap(), domain);
/// assert!(Domain::from_json("{\"old\":\"twitter.com/\",\"new\":\"nitter.net/\"}").is_err())
///
///
/// ```
#[cfg(feature = "json")]
impl ToFromJson<'_> for Domain {}

impl TryFrom<(&str, &str)> for Domain {
    type Error = DomainChangerError;

    /// Create [`Domain`] instance
    /// # Example
    /// ```rust
    /// use domain_changer::types::Domain;
    ///
    ///
    /// let foo: Domain = Domain::try_from(
    ///         ("https://youtube.com", "https://piped.kavin.rocks")
    ///     ).unwrap();
    ///
    /// assert_eq!(foo.new.domain(), Some("piped.kavin.rocks"));
    /// assert_eq!(foo.old.domain(), Some("youtube.com"));
    /// ```
    ///
    fn try_from(domains: (&str, &str)) -> DomainChangerResult<Self> {
        Ok(Self {
            old: Url::parse(domains.0).map_err(|_| {
                DomainChangerError::InvalidOldDomain(format!(
                    "'{}', is invalid old domain",
                    domains.0
                ))
            })?,
            new: Url::parse(domains.1).map_err(|_| {
                DomainChangerError::InvalidNewDomain(format!(
                    "'{}', is invalid new domain",
                    domains.1
                ))
            })?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{errors::DomainChangerResult, Domain};

    #[test]
    fn domain_tryfrom_test() {
        let domain: DomainChangerResult<Domain> =
            Domain::try_from(("twitter.com", "https://nitter.net"));
        assert!(domain.err().unwrap().is_invalid_old_domain());

        let domain: DomainChangerResult<Domain> =
            Domain::try_from(("https://twitter.com", "nitter.net"));
        assert!(domain.err().unwrap().is_invalid_new_domain());
    }
}
