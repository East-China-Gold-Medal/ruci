/* @file

    login controller implementation of RUCI.
    SPDX-License-Identifier: WTFPL

*/

use cgi::{Request, Response};
use serde::{Deserialize, Serialize};
use crate::controller::{extract_parameter, json_response};
use crate::service;

#[derive(Serialize, Deserialize)]
struct LoginResult {
    status: String,
    username: String,
    token: String,
    token_valid_before: u128
}

#[doc = "POST,/login"]
pub(crate) fn login(request: Request) -> Response {

    let param = extract_parameter(request);

    if param.contains_key("username") && param.contains_key("password") {
        match service::session::login (
            param.get("username").unwrap(),
            param.get("password").unwrap()
        ) {
            Some(token) => {
                json_response(200, Box::new(LoginResult{
                    status: "success".to_string(),
                    username: param.get("username").unwrap().to_string(),
                    token:token.0,
                    token_valid_before:token.1
                }))
            },
            None => {
                json_response(403, Box::new(LoginResult{
                    status: "incorrect password".to_string(),
                    username: "".to_string(),
                    token: "".to_string(),
                    token_valid_before: 0,
                }))
            }
        }
    }
    else {
        json_response(403, Box::new(LoginResult{
            status: "invalid parameter".to_string(),
            username: "".to_string(),
            token: "".to_string(),
            token_valid_before: 0,
        }))
    }
}
