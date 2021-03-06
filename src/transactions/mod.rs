// Rust Bitcoin tools library
// Written in 2016 by
//   Ilya Eriklintsev <erik.lite@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Transactions
//!
//! Craftable transactions specifications and related functions

/// A trait which allows us to craft transactions of given type and to match existing data
trait Craftable {
}

/// Simple transfer of bitcoins
struct SimpleTransfer;

///
impl Craftable for SimpleTransfer {
}
