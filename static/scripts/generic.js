/*
	Generic utility script of Rust-UCI(RUCI).
	SPDX-License-Identifier: WTFPL
*/

function invoke_post_ajax(uri, param, success_callback, fail_callback) {
	let ajax = new XMLHttpRequest();
	ajax.open("POST",uri,true);
	ajax.send(param);
	ajax.onreadystatechange = function(i) {
		if(ajax.readyState === 4) {
			if(ajax.status === 200) {
				success_callback(ajax)
				return
			}
			fail_callback(ajax)
		}
	}
}

function invoke_get_ajax(uri, param, success_callback, fail_callback) {
	let ajax = new XMLHttpRequest();
	ajax.open("GET",uri+param,true);
	ajax.onreadystatechange = function(i) {
		if(ajax.readyState === 4) {
			if(ajax.status === 200) {
				success_callback(ajax)
				return
			}
			fail_callback(ajax)
		}
	}
	ajax.send();
}
