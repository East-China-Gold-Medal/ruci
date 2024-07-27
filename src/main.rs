/* @file

    Main entry of Rust-UCI(RUCI).
    SPDX-License-Identifier: WTFPL

*/
pub mod provider;

extern crate cgi;

use crate::provider::SettingsProvider;

// Test UCI binding.
cgi::cgi_main! {
    |request: cgi::Request| -> cgi::Response {
        let settings: &mut dyn SettingsProvider = provider::initialize_settings();
        let res =&settings.get("network.lan.proto");
        match res {
            Ok(str) => {cgi::text_response(200, str)},
            Err(err) => {cgi::text_response(500, err)},
        }
    }
}
