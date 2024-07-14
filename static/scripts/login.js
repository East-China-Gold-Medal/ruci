/*
	Login script of Rust-UCI(RUCI).
	SPDX-License-Identifier: WTFPL
*/

function login() {
	var str = document.getElementById("password_input").value;
	alert(str);
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
