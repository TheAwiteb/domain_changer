//! A module that includes the types of the library

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

mod config;
mod domain;
pub mod errors;
pub use {config::Config, domain::Domain};
