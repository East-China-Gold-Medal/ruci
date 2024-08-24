/* @file

    "controller" mod configuration of RUCI.
    SPDX-License-Identifier: WTFPL

*/

use cgi::http::Method;
use cgi::{Request, Response};
use lazy_static::lazy_static;
use regex::Regex;
use rust_embed::Embed;
use serde::Serialize;
use std::collections::HashMap;

mod info;
mod login;

#[derive(Embed)]
#[folder = "static/"]
#[prefix = "/"]
struct Asset;

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

#[derive(PartialEq, Copy, Clone)]
enum MimeType {
    HTML,
    CSS,
    JS,
    SVG,
    PNG,
    Unknown,
}

const MIME_TYPE_STRINGS: [&str; 6] = [
    "text/html; charset=utf-8",
    "text/css; charset=utf-8",
    "text/javascript; charset=utf-8",
    "image/svg+xml",
    "image/png",
    "application/octet-stream",
];

///
/// Backward match to get file extension.
/// .html , .css , .js , .svg , .png
/// lmth. , ssc. , sj. , gvs. , gnp.
///
struct ExtensionState {
    current_state: usize,
    next_state: usize,
    mime_type: MimeType,
    input_character: char,
}

const EXTENSION_STATES_TABLE: [ExtensionState; 18] = [
    ExtensionState {
        current_state: 0,
        next_state: 1,
        mime_type: MimeType::Unknown,
        input_character: 'l',
    },
    ExtensionState {
        current_state: 0,
        next_state: 6,
        mime_type: MimeType::Unknown,
        input_character: 's',
    },
    ExtensionState {
        current_state: 0,
        next_state: 12,
        mime_type: MimeType::Unknown,
        input_character: 'g',
    },
    ExtensionState {
        current_state: 1,
        next_state: 2,
        mime_type: MimeType::Unknown,
        input_character: 'm',
    },
    ExtensionState {
        current_state: 2,
        next_state: 3,
        mime_type: MimeType::Unknown,
        input_character: 't',
    },
    ExtensionState {
        current_state: 3,
        next_state: 4,
        mime_type: MimeType::Unknown,
        input_character: 'h',
    },
    ExtensionState {
        current_state: 4,
        next_state: 5,
        mime_type: MimeType::HTML,
        input_character: '.',
    },
    ExtensionState {
        current_state: 6,
        next_state: 7,
        mime_type: MimeType::Unknown,
        input_character: 's',
    },
    ExtensionState {
        current_state: 6,
        next_state: 10,
        mime_type: MimeType::Unknown,
        input_character: 'j',
    },
    ExtensionState {
        current_state: 7,
        next_state: 8,
        mime_type: MimeType::Unknown,
        input_character: 'c',
    },
    ExtensionState {
        current_state: 8,
        next_state: 9,
        mime_type: MimeType::CSS,
        input_character: '.',
    },
    ExtensionState {
        current_state: 10,
        next_state: 11,
        mime_type: MimeType::JS,
        input_character: '.',
    },
    ExtensionState {
        current_state: 12,
        next_state: 13,
        mime_type: MimeType::Unknown,
        input_character: 'n',
    },
    ExtensionState {
        current_state: 12,
        next_state: 16,
        mime_type: MimeType::Unknown,
        input_character: 'v',
    },
    ExtensionState {
        current_state: 13,
        next_state: 14,
        mime_type: MimeType::Unknown,
        input_character: 'p',
    },
    ExtensionState {
        current_state: 14,
        next_state: 15,
        mime_type: MimeType::PNG,
        input_character: '.',
    },
    ExtensionState {
        current_state: 16,
        next_state: 17,
        mime_type: MimeType::Unknown,
        input_character: 's',
    },
    ExtensionState {
        current_state: 17,
        next_state: 18,
        mime_type: MimeType::SVG,
        input_character: '.',
    },
];

const EXTENSION_STATE_TABLE_HINTS: [usize; 19] = [
    0, 3, 4, 5, 6, 0, 7, 9, 10, 0, 11, 0, 12, 14, 15, 0, 16, 17, 18,
];

fn get_file_mime_type_by_uri(uri: &str) -> &'static str {
    let mut state = 0;
    let uri_bytes = uri.as_bytes();
    let iteration_step = if uri_bytes.len() > 5 {
        uri_bytes.len() - 5
    } else {
        0
    };
    let mut current_state_ptr = 0;
    let mut i = uri_bytes.len() - 1;
    'next_char: while i >= iteration_step {
        while EXTENSION_STATES_TABLE[current_state_ptr].current_state == state {
            if EXTENSION_STATES_TABLE[current_state_ptr].input_character == uri_bytes[i] as char {
                let next_step = EXTENSION_STATES_TABLE[current_state_ptr].next_state;
                current_state_ptr = EXTENSION_STATE_TABLE_HINTS[next_step];
                state = next_step;
                i -= 1;
                if EXTENSION_STATES_TABLE[current_state_ptr].mime_type != MimeType::Unknown {
                    return MIME_TYPE_STRINGS
                        [EXTENSION_STATES_TABLE[current_state_ptr].mime_type as usize];
                }
                continue 'next_char;
            }
            current_state_ptr += 1;
        }
        // Not Accepted!
        break;
    }
    MIME_TYPE_STRINGS[MimeType::Unknown as usize]
}

pub fn generate_response_from_file(uri: &str) -> Response {
    //return string_response(200,uri);
    match Asset::get(uri) {
        Some(val) => http::response::Builder::new()
            .status(200)
            .header(http::header::CONTENT_TYPE, get_file_mime_type_by_uri(uri))
            .header(
                http::header::CONTENT_LENGTH,
                format!("{}", val.data.as_ref().len()).as_str(),
            )
            .body(val.data.to_vec())
            .unwrap(),
        None => {
            // TBD: Better 404!
            cgi::text_response(404, "Not found")
        }
    }
}

pub fn route_uri(request: Request) -> Response {
    match request.headers().get("x-cgi-path-info") {
        Some(p) => {
            let path = p.to_str().unwrap();
            match search_tree(path) {
                Some(val) => val(request),
                None => generate_response_from_file(path),
            }
        }
        None => {
            // TBD: Main page.
            generate_response_from_file("framework.html")
        }
    }
}
