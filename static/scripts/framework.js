/*
	Framework script of Rust-UCI(RUCI).
	SPDX-License-Identifier: WTFPL
*/

function switch_web_page(name) {
	invoke_get_ajax(name+'.html',"",
		function(ajax) {
			document.getElementById('detail_container_real').innerHTML = ajax.responseText;
			eval('onload_callback_'+name+'()')
		},
		function (ajax) {
			alert("Cannot load web page:"+ajax.httpRequestStatusCode);
		}
	)
}

function set_content_title(str) {
	document.getElementById('detail_content_title').innerHTML = str;
}

function toggle_expand(item) {
	if(item.parentElement.style.height === '64px') {
		item.parentElement.style.height = 'auto';
		item.querySelector(".expand_icon").style.rotate = '180deg'
	}
	else {
		item.parentElement.style.height = '64px';
		item.querySelector(".expand_icon").style.rotate = '0deg'
	}
}

function copy_information(item) {
	// TODO
	return false;
}
