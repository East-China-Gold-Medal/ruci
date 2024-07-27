/* @file

    Main entry of Rust-UCI(RUCI).
    SPDX-License-Identifier: WTFPL

*/
mod provider;
mod controller;

extern crate cgi;

// Test UCI binding.
cgi::cgi_main! {
    |request: cgi::Request| -> cgi::Response {
        controller::route_uri(request)
    }
}
