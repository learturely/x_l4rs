//     [xdlinux/libxduauth] for Rust.
//     Copyright (C) 2024  learturely <learturely@gmail.com>
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![feature(vec_into_raw_parts)]
#![feature(let_chains)]
#![feature(str_as_str)]
#![feature(adt_const_params)]
extern crate core;

pub mod protocol;
pub mod utils;

#[cfg(any(feature = "ehall_login_impl", feature = "ids_login_impl"))]
mod session;
#[cfg(any(feature = "ehall_login_impl", feature = "ids_login_impl"))]
pub use session::*;

#[cfg(feature = "cxlib_login_integrated")]
mod login_solver;

#[cfg(feature = "cxlib_login_integrated")]
pub use login_solver::*;

#[cfg(test)]
mod tests {}
