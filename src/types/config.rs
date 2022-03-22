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

use crate::types::Domain;

/// [`Config`] struct help you to manage domains with [`Domain`] struct
#[derive(Debug, Clone)]
pub struct Config {
    pub domains: Vec<Domain>,
}

impl Config {
    /// Create new [`Config`] instance
    ///
    /// # Example
    /// ```rust
    /// use domain_changer::types::{Config, Domain};
    ///
    /// let my_config = Config::new(vec![
    ///     Domain::try_from(("https://youtube.com/", "https://piped.kavin.rocks/")).unwrap(),
    ///     Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap(),
    ///     Domain::try_from(("https://reddit.com/", "https://libredd.it/")).unwrap()
    ///  ]);
    ///
    /// assert_eq!(my_config.domains.len(), 3);
    /// assert_eq!(my_config.domains[1].new.as_str(), "https://nitter.net/");
    /// ```
    ///
    pub fn new(domains: Vec<Domain>) -> Self {
        Self { domains }
    }

    /// Returns all old host name of domains
    ///
    /// # Example
    /// ```rust
    /// use domain_changer::types::{Config, Domain};
    ///
    /// let my_config = Config::new(vec![
    ///     Domain::try_from(("https://youtube.com/", "https://piped.kavin.rocks/")).unwrap(),
    ///     Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap(),
    ///     Domain::try_from(("https://reddit.com/", "https://libredd.it/")).unwrap()
    ///  ]);
    ///
    /// assert_eq!(my_config.domains.len(), 3);
    /// assert_eq!(my_config.old_hosts(), vec!["youtube.com", "twitter.com", "reddit.com"]);
    /// ```
    pub fn old_hosts(&self) -> Vec<&str> {
        self.domains
            .iter()
            .filter_map(|domain| domain.old.host_str())
            .collect()
    }

    /// Returns all new host name of domains
    ///
    /// # Example
    /// ```rust
    /// use domain_changer::types::{Config, Domain};
    ///
    /// let my_config = Config::new(vec![
    ///     Domain::try_from(("https://youtube.com/", "https://piped.kavin.rocks/")).unwrap(),
    ///     Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap(),
    ///     Domain::try_from(("https://reddit.com/", "https://libredd.it/")).unwrap()
    ///  ]);
    ///
    /// assert_eq!(my_config.domains.len(), 3);
    /// assert_eq!(my_config.new_hosts(), vec!["piped.kavin.rocks", "nitter.net", "libredd.it"]);
    /// ```
    pub fn new_hosts(&self) -> Vec<&str> {
        self.domains
            .iter()
            .filter_map(|domain| domain.new.host_str())
            .collect()
    }

    /// Returns reference [`Domain`] from [`domains`] by `old_host` if any
    ///
    /// [`domains`]: Config#structfield.domains
    ///
    /// # Example
    /// ```rust
    /// use domain_changer::types::{Config, Domain};
    ///
    /// let my_config = Config::new(vec![
    ///     Domain::try_from(("https://youtube.com/", "https://piped.kavin.rocks/")).unwrap(),
    ///     Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap(),
    ///     Domain::try_from(("https://reddit.com/", "https://libredd.it/")).unwrap()
    ///  ]);
    ///
    /// assert_eq!(my_config.get_by_old("youtube.com"), Some(&my_config.domains[0]));
    /// assert_eq!(my_config.get_by_old("youtube"), None);
    /// ```
    pub fn get_by_old(&self, old_host: &str) -> Option<&Domain> {
        self.domains.iter().find(|domain| {
            if let Some(host) = domain.old.host_str() {
                host == old_host
            } else {
                false
            }
        })
    }

    /// Use [`Domain.contain`] with all domains
    ///
    /// # Note
    /// Return first domain not all (by [`old`])
    ///
    /// # Example
    /// ```rust
    /// use domain_changer::types::Config;
    ///
    /// let config: Config = Config::default();
    /// assert!(config.contain("google.com", true).is_none());
    /// assert!(config.contain("youtube.com", true).is_some());
    /// assert!(config.contain("https://libredd.it", false).is_some());
    /// ```
    ///
    /// [`old`]: Domain#structfield.old
    /// [`Domain.contain`]: method@Domain::contain
    pub fn contain(&self, word: &str, just_old: bool) -> Option<&Domain> {
        self.domains
            .iter()
            .find(|domain| domain.contain(word, just_old).is_some())
    }
}

impl Default for Config {
    /// Default instance of [`Config`] is the most popular privacy sites
    /// it is [piped](https://https://piped.kavin.rocks/),
    /// [nitter](https://nitter.net/),
    /// [libredd](https://libredd.it/)
    ///
    fn default() -> Self {
        Self::new(vec![
            // Youtube domains
            Domain::try_from(("https://youtube.com/", "https://piped.kavin.rocks/")).unwrap(),
            Domain::try_from(("https://www.youtube.com/", "https://piped.kavin.rocks/")).unwrap(),
            Domain::try_from(("https://youtu.be/", "https://piped.kavin.rocks/")).unwrap(),
            // Twitter domains
            Domain::try_from(("https://t.co/", "https://nitter.net/")).unwrap(),
            Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap(),
            // Reddit domains
            Domain::try_from(("https://reddit.com/", "https://libredd.it/")).unwrap(),
        ])
    }
}
