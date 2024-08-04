/* @file

    Main entry of Rust-UCI(RUCI).
    SPDX-License-Identifier: WTFPL

*/

mod provider;
mod controller;
mod service;

extern crate cgi;

use std::backtrace::Backtrace;
use std::panic;

// Test UCI binding.
fn main() {
    panic::set_hook(Box::new(|info| {
        println!(concat!(
                "Status: 500 Internal Server Error\r\n",
                "cache-control: no-cache\r\n",
                "content-type: text/plain\r\n\r\n",
                "{}\r\n{}\r\n"),
            info,
            Backtrace::force_capture());
    }));

    cgi::handle(|request: cgi::Request| -> cgi::Response {
        controller::route_uri(request)
    });
}
