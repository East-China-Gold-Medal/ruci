/*
	Login script of Rust-UCI(RUCI).
	SPDX-License-Identifier: WTFPL
*/

function login() {
	let str = document.getElementById("password_input").value;
    invoke_post_ajax (
        'login',
        "username=root&password="+encodeURI(str),
        function (ajax) {
            let json = JSON.parse(ajax.responseText);
            document.cookie = "token="+json.token;
            window.location.replace("framework.html");
        },
        function (ajax) {
            alert("Login failed!\nIncorrect password?")
        }
    )
}

function blur_on() {
	document.getElementById("background_blur").style.filter = "blur(30px)";
}
function blur_off() {
	document.getElementById("background_blur").style.filter = "";
}

(function(){
    let moving = false,
        timer = null;
    window.onmousemove = window.onkeydown = function(){
        moving = true;
        clearTimeout(timer);
		blur_on();
        timer = setTimeout(function(){
			blur_off();
        },3000);
    }
}());
