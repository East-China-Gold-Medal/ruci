/* @file

    "controller" mod configuration of RUCI.
    SPDX-License-Identifier: WTFPL

*/
use std::collections::HashMap;
use cgi::Response;
use lazy_static::lazy_static;

mod login;
mod info;

lazy_static! {
    static ref mapper:HashMap<&'static str, fn(cgi::Request)->Response> = {
        let mut map:HashMap<&'static str, fn(cgi::Request)->Response> = HashMap::new();
        map.insert("/login",login::login as fn(cgi::Request)->Response);
        map.insert("/info",info::info as fn(cgi::Request)->Response);
        map
    };
}

pub fn route_uri(request: cgi::Request) -> cgi::Response {
    let path = request.headers().get("x-cgi-path-info").unwrap().to_str().unwrap();
    match mapper.get(path) {
        Some (val) => {val(request)}
        None => {cgi::text_response(404, "Not found")}
    }
}