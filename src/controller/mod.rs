/* @file

    "controller" mod configuration of RUCI.
    SPDX-License-Identifier: WTFPL

*/

use cgi::http::Method;
use cgi::{Request, Response};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

mod info;
mod login;

include!(concat!(env!("OUT_DIR"), "/bindings.gen.rs"));

// Implement this as lazy_static because it is used frequently in program.
lazy_static! {
    static ref HTTP_PARAM_REGEX: Regex =
        Regex::new(r#"(?<name>[^=&]+)(=)?((?<value>[^&]+)&?)?"#).unwrap();
}

///
/// Extract parameter from query Request.
/// TODO: Support PUT/DELETE method and file upload.
///

pub fn extract_parameter(request: Request) -> HashMap<String, String> {
    let mut ret = HashMap::new();
    let mut params: Option<String> = None;

    match request.method() {
        &Method::GET => match request.uri().query() {
            Some(res) => {
                params = Some(String::from(res));
            }
            None => {}
        },
        &Method::POST => {
            let body = request.body().to_vec();
            if body.len() > 0 {
                params = Some(String::from_utf8(body).unwrap());
            }
        }
        _ => {}
    };

    if params.is_some() {
        let mut skip_position = 0;
        let str = &*params.unwrap().clone();
        while skip_position < str.len() {
            let result = HTTP_PARAM_REGEX.captures_at(str, skip_position).unwrap();
            let total_match = result.get(0).unwrap();
            skip_position += total_match.len();
            let name = result.name("name");
            let value = result.name("value");
            if name.is_some() {
                match value {
                    None => {
                        ret.insert(String::from(name.unwrap().as_str()), String::from(""));
                        skip_position += 1;
                    }
                    Some(v) => {
                        ret.insert(
                            String::from(name.unwrap().as_str()),
                            String::from(v.as_str()),
                        );
                    }
                }
            } else {
                // Not found: What?
                break;
            }
        }
    }

    ret
}

pub fn json_response<T, S>(status_code: T, body: Box<S>) -> Response
where
    http::StatusCode: TryFrom<T>,
    <http::StatusCode as TryFrom<T>>::Error: Into<http::Error>,
    S: ?Sized + Serialize,
{
    let ret = serde_json::to_string(&body).unwrap();
    let body: Vec<u8> = ret.into_bytes();
    http::response::Builder::new()
        .status(status_code)
        .header(
            http::header::CONTENT_LENGTH,
            format!("{}", body.len()).as_str(),
        )
        .header(
            http::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .body(body)
        .unwrap()
}

fn search_tree(key: &str) -> Option<fn(Request) -> Response> {
    let mut current_state = 0;
    let mut previous_state_ptr = 0;
    let mut current_state_ptr = 0;
    let key_raw = key.as_bytes();
    'next_char: for i in 1..key_raw.len() {
        while current_state_ptr != usize::MAX
            && STATES_TABLE[current_state_ptr].current_state == current_state
        {
            if STATES_TABLE[current_state_ptr].input_character == key_raw[i] as char {
                current_state = STATES_TABLE[current_state_ptr].next_state;
                previous_state_ptr = current_state_ptr;
                current_state_ptr = STATE_HINTS_TABLE[current_state];
                continue 'next_char;
            }
            current_state_ptr += 1;
        }
        return None; // Not Accepted!
    }
    // If not in 'ACCEPTED' state, this method cannot be found.
    STATES_TABLE[previous_state_ptr].target
}

pub fn route_uri(request: Request) -> Response {
    return match request.headers().get("x-cgi-path-info") {
        Some(p) => {
            let path = p.to_str().unwrap();
            match search_tree(path) {
                Some(val) => val(request),
                None => cgi::text_response(404, "Not found"),
            }
        }
        None => {
            // TBD: Main page.
            cgi::text_response(404, "Not found")
        }
    };
}
