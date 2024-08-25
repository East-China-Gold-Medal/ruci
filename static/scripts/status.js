/*
	Status script of Rust-UCI(RUCI).
	SPDX-License-Identifier: WTFPL
*/

function onload_callback_status() {
	set_content_title('Status')
	invoke_post_ajax('status','',
		function(ajax) {
			let val = JSON.parse(ajax.responseText);
			let elapsed_time = new Date (val.power_on_time * 1000);

			let time_string =
				`${elapsed_time.getUTCFullYear()-1970} years,` +
				`${elapsed_time.getUTCMonth()} months, ` +
				`${elapsed_time.getUTCDate()} days, ` +
				`${elapsed_time.getUTCHours()}:${elapsed_time.getUTCMinutes()}:${elapsed_time.getUTCSeconds()}`
			document.getElementById('device_name').innerHTML = val.device_name;
			document.getElementById('device_model').innerHTML = val.device_model;
			document.getElementById('processor').innerHTML = val.processor;
			document.getElementById('installed_ram').innerHTML = val.installed_ram;
			document.getElementById('architecture').innerHTML = val.architecture;
			document.getElementById('firmware_version').innerHTML = val.firmware_version;
			document.getElementById('target_platform').innerHTML = val.target_platform;
			document.getElementById('kernel_version').innerHTML = val.kernel_version;
			document.getElementById('power_on_time').innerHTML = time_string;
			document.getElementById('average_load').innerHTML = val.average_load;
		},
		function () {

		}
	)
}
