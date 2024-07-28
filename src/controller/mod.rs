/* @file

    "controller" mod configuration of RUCI.
    SPDX-License-Identifier: WTFPL

*/
use std::collections::HashMap;
use cgi::Response;
use lazy_static::lazy_static;

mod login;
mod info;

include!(concat!(env!("OUT_DIR"), "/bindings.gen.rs"));

pub fn route_uri(request: cgi::Request) -> cgi::Response {
    let path = request.headers().get("x-cgi-path-info").unwrap().to_str().unwrap();
    match mapper.get(path) {
        Some (val) => {val(request)}
        None => {cgi::text_response(404, "Not found")}
    }
}