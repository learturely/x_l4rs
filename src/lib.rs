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

extern crate core;

pub mod protocol;
pub mod utils;

#[cfg(any(feature = "ehall", feature = "ids", feature = "rsbbs"))]
mod session;
#[cfg(any(feature = "ehall", feature = "ids", feature = "rsbbs"))]
pub use session::*;

#[cfg(all(feature = "cxlib_user", feature = "ids"))]
mod login_solver;

#[cfg(all(feature = "cxlib_user", feature = "ids"))]
pub use login_solver::*;

#[cfg(test)]
mod tests {}
