// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/ 
var Macro_run = false


async function Call_rust() {
  var user = String(document.forms["Credentials"]["username"].value);
  var pass = String(document.forms["Credentials"]["password"].value);
  await window.__TAURI__.core.invoke("listen_for_cmd_r_and_write", { user: user , pass : pass });
  document.getElementById("red-dot").style.backgroundColor="#4f8f4aff";
  console.log("Rust Function called without problem");


  //IMMA CLEAr this form
  document.forms["Credentials"]["username"].value = "";
  document.forms["Credentials"]["password"].value = "";

};

async function Stop_rust() {
  await window.__TAURI__.core.invoke("stop_listening")
  document.getElementById("red-dot").style.backgroundColor="#b12121ff"
  console.log("rust was stopped")
}

  


window.Call_rust = Call_rust;
window.Stop_rust = Stop_rust;

