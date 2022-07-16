const invoke = window.__TAURI__.invoke;

window.onload = () => {
    let submit_button = document.querySelector("#submit-button");
    submit_button.addEventListener("click", submit);
}

function submit() {
    let access_token = document.querySelector("#access-token");
    
    if (access_token === null) {
        return console.error(new Error("Could not find #access-token"));
    }

    invoke("set_sparebank_1_access_token", { value: access_token.value })
        .then(() => {
            access_token.value = "";
            return;
        })
        .catch((_error) => {
            return console.error(new Error("Failed submitting data"));
        });
}