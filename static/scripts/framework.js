/*
	Framework script of Rust-UCI(RUCI).
	SPDX-License-Identifier: WTFPL
*/

function switch_web_page(name) {
    invoke_get_ajax(name+'.html',"",
        function(ajax) {
            document.getElementById('detail_container').innerHTML = ajax.responseText
        },
        function (ajax) {
            alert("Cannot load web page:"+ajax.httpRequestStatusCode)
        }
    )
}

function switch_to_status() {
    switch_web_page('status')
}

function switch_to_system() {
    switch_web_page('system')
}

function switch_to_internet() {
    switch_web_page('internet')
}

function switch_to_wireless() {
    switch_web_page('wireless')
}

function switch_to_intranet() {
    switch_web_page('intranet')
}

function switch_to_advanced() {
    switch_web_page('advanced')
}
