const invoke = window.__TAURI__.invoke;

window.onload = () => {
    let submit_button = document.querySelector("#submit-button");
    submit_button.addEventListener("click", submit);
}

function submit() {
    let key = document.querySelector("#key");
    let secret = document.querySelector("#secret");
    let passphrase = document.querySelector("#passphrase");
    
    if (key === null) {
        return console.error(new Error("Could not find #key"));
    }

    invoke("set_coinbase_pro_key", { value: key.value })
        .then(() => {
            key.value = "";
            return;
        })
        .catch((_error) => {
            return console.error(new Error("Failed submitting data"));
        });

    if (secret === null) {
        return console.error(new Error("Could not find #secret"));
    }
    
    invoke("set_coinbase_pro_secret", { value: secret.value })
        .then(() => {
            secret.value = "";
            return;
        })
        .catch((_error) => {
            return console.error(new Error("Failed submitting data"));
        });

    if (passphrase === null) {
        return console.error(new Error("Could not find #passphrase"));
    }

    invoke("set_coinbase_pro_passphrase", { value: passphrase.value })
        .then(() => {
            passphrase.value = "";
            return;
        })
        .catch((_error) => {
            return console.error(new Error("Failed submitting data"));
        });
}