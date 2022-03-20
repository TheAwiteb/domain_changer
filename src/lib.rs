#![doc = include_str!("../README.md")]

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

pub mod types;
use types::{Config, Domain};

/// Parse [`String`] and return new [`String`] with new domains if any
///
/// # Example
/// ```rust
/// use domain_changer::parse_string;
/// use domain_changer::types::Config;
///
/// let text: String = "Wellcome to my youtube channel: https://www.youtube.com/channel/UCeRbJsc8cl7xBwT3jIxaAdg And my twitter is: twitter.com/Awiteb".to_string();
/// let config: Config = Config::default();
/// assert_eq!(parse_string(&config, text),
///     "Wellcome to my youtube channel: https://piped.kavin.rocks/channel/UCeRbJsc8cl7xBwT3jIxaAdg And my twitter is: https://nitter.net/Awiteb".to_string()
///     );
/// ```
pub fn parse_string(config: &Config, text: String) -> String {
    if !text.is_empty() {
        text.split_ascii_whitespace()
            .map(|word| {
                for domain in config.domains.iter() {
                    if let Some(mut url) = domain.contain(word, true) {
                        if let Some(new_host) = domain.new.host_str() {
                            // Error of `set_host` is `ParseError`, and we got the host
                            // from the Url instance, ensuring that there is no problem
                            url.set_host(Some(new_host)).unwrap();
                            return url.as_str().to_string();
                        }
                    }
                }
                word.to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    } else {
        text
    }
}

/// Returns all [`old`] domains in text if it is in [`config.domains`]
///
/// # Example
/// ```rust
/// use domain_changer::extract_old_domains;
/// use domain_changer::types::{Config, Domain};
///
/// let config: Config = Config::default();
/// let text: String = String::from(
///     "Hi i hate youtube.com and https://twitter.com what about you?"
/// );
/// assert_eq!(
///     extract_old_domains(&config, text),
///     vec![
///         &Domain::try_from(("https://youtube.com/", "https://piped.kavin.rocks/")).unwrap(),
///         &Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap()
///     ]
/// );
/// let empty_vec: Vec<&Domain> = Vec::new();
/// assert_eq!(
///     extract_old_domains(&config, String::from("Hello, World!")),
///     empty_vec
/// );
/// ```
///
/// [`config.domains`]: struct.Config.html#structfield.domains
/// [`old`]: struct.Domain.html#structfield.old
pub fn extract_old_domains(config: &Config, text: String) -> Vec<&Domain> {
    text.split_ascii_whitespace()
        .filter_map(|word| config.contain(word, true))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_string, types::Config};

    #[test]
    fn parse_string_test() {
        let config: Config = Config::default();

        assert_eq!(parse_string(&config, String::new()), String::new());
        assert_eq!(
            parse_string(&config, "Hello, world".to_owned()),
            "Hello, world".to_owned()
        );
        assert_eq!(
            parse_string(&config, "Hello https://randomdooomain.com".to_owned()),
            "Hello https://randomdooomain.com".to_owned()
        );
        assert_eq!(
            parse_string(&config, "hi, youtube.com/something".to_owned()),
            "hi, https://piped.kavin.rocks/something".to_owned()
        );
        assert_eq!(
            parse_string(&config, "hi, youtube.com".to_owned()),
            "hi, https://piped.kavin.rocks/".to_owned()
        )
    }
}
