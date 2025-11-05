

async function Call_rust() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  var user = String(document.forms["Credentials"]["username"].value);
  var pass = String(document.forms["Credentials"]["password"].value);
  console.log("this is working 1");
  await invoke("listen_for_cmd_r_and_write", { user: user , pass : pass });
  console.log("this is working 2");
};

function Test(){
  alert("yess");
  console.log("yes")
}

/*window.addEventListener("DOMContentLoaded", () => {

  document.querySelector("Credentials").addEventListener("submit", (e) => {
    e.preventDefault();
    Call_rust();
  });
});*/

window.Test = Test;
window.Call_rust = Call_rust;

