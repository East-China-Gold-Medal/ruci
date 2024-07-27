/* @file

    info controller implementation of RUCI.
    SPDX-License-Identifier: WTFPL

*/
use cgi::{Request, Response};
use crate::provider;

pub(crate) fn info(_request: Request) -> Response {
    let mut settings= provider::settings::initialize_settings();
    let res =settings.get("network.lan.proto");
    match res {
        Ok(str) => {cgi::text_response(200, str)},
        Err(err) => {cgi::text_response(500, err)},
    }
}