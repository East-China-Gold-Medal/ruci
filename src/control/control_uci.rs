/* @file

    "control" mod configuration of RUCI, binding to LibUCI.
    SPDX-License-Identifier: WTFPL

*/
use rust_uci::error::Result;
use rust_uci::Uci;

pub unsafe fn get_key(key: &str) -> Result<String> {
    let mut uci: Uci = Uci::new()?;
    uci.get(&*key)
}
