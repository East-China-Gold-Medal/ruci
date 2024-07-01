/* @file

    Main entry of Rust-UCI(RUCI).
    SPDX-License-Identifier: WTFPL

*/

use crate::control::control_uci::get_key;

mod control;

extern crate cgi;

// Test UCI binding.
cgi::cgi_main! { |request: cgi::Request| -> cgi::Response{
    unsafe {
        let res = get_key("network.lan.proto");
        match res {
            Ok(str) => {cgi::text_response(200, str)},
            Err(err) => {cgi::text_response(500, format!("{:}",err))},
        }
    }
} }
