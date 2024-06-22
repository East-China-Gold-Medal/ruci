/* @file

    Main entry of Rust-UCI(RUCI).
    SPDX-License-Identifier: WTFPL

*/

use crate::control::uci::uci_alloc_context;

mod control;

fn main() {
    println!("It Works!");
    let a = unsafe { uci_alloc_context() };
}
