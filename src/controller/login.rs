/* @file

    login controller implementation of RUCI.
    SPDX-License-Identifier: WTFPL

*/
use cgi::{Request, Response};

#[doc = r"GET,/login"]
pub(crate) fn login(request: Request) -> Response {
    cgi::text_response(200, "Success".to_owned() +request.method().as_str())
}