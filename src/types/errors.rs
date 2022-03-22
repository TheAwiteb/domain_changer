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

/// Errors of domain_changer
#[derive(Debug)]
pub enum DomainChangerError {
    /// Error mean the [`old`] domain is invalid
    ///
    /// [`old`]: crate::types::Domain#structfield.old
    InvalidOldDomain(String),
    /// Error mean the [`new`] domain is invalid
    ///
    /// [`new`]: crate::types::Domain#structfield.new
    InvalidNewDomain(String),
}

/// [`Result`] contain [`DomainChangerError`]
pub type DomainChangerResult<T> = Result<T, DomainChangerError>;

impl DomainChangerError {
    /// Returns if the error is [`InvalidOldDomain`]
    ///
    /// # Example
    /// ```rust
    /// use domain_changer::types::errors::DomainChangerError::{InvalidOldDomain, InvalidNewDomain};
    ///
    /// assert!(InvalidOldDomain("error msg".to_owned()).is_invalid_old_domain());
    /// assert!(!InvalidNewDomain("error msg".to_owned()).is_invalid_old_domain());
    /// ```
    ///
    /// [`InvalidOldDomain`]: enum@DomainChangerError#variant.InvalidOldDomain
    pub fn is_invalid_old_domain(&self) -> bool {
        matches!(self, DomainChangerError::InvalidOldDomain(_))
    }

    /// Returns if the error is [`InvalidNewDomain`]
    /// # Example
    /// ```rust
    /// use domain_changer::types::errors::DomainChangerError::{InvalidOldDomain, InvalidNewDomain};
    ///
    /// assert!(InvalidNewDomain("error msg".to_owned()).is_invalid_new_domain());
    /// assert!(!InvalidOldDomain("error msg".to_owned()).is_invalid_new_domain());
    /// ```
    ///
    /// [`InvalidNewDomain`]: enum@DomainChangerError#variant.InvalidNewDomain
    pub fn is_invalid_new_domain(&self) -> bool {
        matches!(self, DomainChangerError::InvalidNewDomain(_))
    }
}
