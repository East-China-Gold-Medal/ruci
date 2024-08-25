/* @file

    info controller implementation of RUCI.
    SPDX-License-Identifier: WTFPL

*/
use crate::controller::json_response;
use crate::provider;
use cgi::{Request, Response};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

// Implement this as lazy_static because it is used frequently in program.
lazy_static! {
    static ref KEY_VALUE_REGEX: Regex = Regex::new(r#"(?<key>.+)='?(?<value>[^']+)'?"#).unwrap();
}

#[doc = "GET,/info"]
pub(crate) fn info(_request: Request) -> Response {
    let mut settings = provider::settings::initialize_settings();
    let res = settings.get("ruci.root.sessionkey");
    match res {
        Ok(str) => cgi::text_response(200, str),
        Err(err) => cgi::text_response(500, err),
    }
}

#[derive(Serialize)]
struct SystemStatus {
    device_name: String,
    device_model: String,
    processor: String,
    installed_ram: String,
    architecture: String,
    firmware_version: String,
    target_platform: String,
    kernel_version: String,
    power_on_time: i64,
    average_load: String,
}

#[doc = "GET, /status"]
pub(crate) fn status(_request: Request) -> Response {
    let openwrt_release_input = BufReader::new(File::open("/etc/openwrt_release").unwrap());
    let mut openwrt_release_map = HashMap::new();

    for line in openwrt_release_input.lines() {
        let line_result = line.unwrap();
        match KEY_VALUE_REGEX.captures(&*line_result) {
            Some(capture) => {
                openwrt_release_map.insert(
                    String::from(capture.name("key").unwrap().as_str()),
                    String::from(capture.name("value").unwrap().as_str()),
                );
            }
            None => {}
        }
    }

    let system_load_information_response = Command::new("uptime").output().unwrap().stdout;

    let mut system_board_ubus_command = Command::new("ubus");
    system_board_ubus_command.arg("call");
    system_board_ubus_command.arg("system");
    system_board_ubus_command.arg("board");
    let system_board_ubus_response = system_board_ubus_command.output().unwrap().stdout;
    let system_board_ubus_json: Value =
        serde_json::from_slice(system_board_ubus_response.as_slice()).unwrap();

    let mut system_info_ubus_command = Command::new("ubus");
    system_info_ubus_command.arg("call");
    system_info_ubus_command.arg("system");
    system_info_ubus_command.arg("info");
    let system_info_ubus_response = system_info_ubus_command.output().unwrap().stdout;
    let system_info_ubus_json: Value =
        serde_json::from_slice(system_info_ubus_response.as_slice()).unwrap();

    let status = Box::new(SystemStatus {
        device_name: system_board_ubus_json["hostname"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap(),
        device_model: system_board_ubus_json["model"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap(),
        processor: system_board_ubus_json["system"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap(),
        installed_ram: format!(
            "{:.2} MiB total / {:.2} MiB available",
            (system_info_ubus_json["memory"]["total"].as_i64().unwrap() as f64) / 1024.0 / 1024.0,
            (system_info_ubus_json["memory"]["available"]
                .as_i64()
                .unwrap() as f64)
                / 1024.0
                / 1024.0
        ),
        architecture: openwrt_release_map.get("DISTRIB_ARCH").unwrap().clone(),
        firmware_version: openwrt_release_map
            .get("DISTRIB_DESCRIPTION")
            .unwrap()
            .clone(),
        target_platform: openwrt_release_map.get("DISTRIB_TARGET").unwrap().clone(),
        kernel_version: system_board_ubus_json["kernel"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap(),
        power_on_time: system_info_ubus_json["uptime"].as_i64().unwrap(),
        average_load: String::from_utf8(Vec::from(
            &system_load_information_response[system_load_information_response.len() - 17
                ..system_load_information_response.len()],
        ))
        .unwrap(),
    });

    json_response(200, status)
}
