const invoke = window.__TAURI__.invoke;

window.onload = () => {
    let submit_button = document.querySelector("#submit-button");
    submit_button.addEventListener("click", submit);
}

function submit() {
    let key = document.querySelector("#key");
    let btc_address = document.querySelector("#btc-address");
    let ltc_address = document.querySelector("#ltc-address");

    if (key === null) {
        return console.error(new Error("Could not find #access-token"));
    }

    invoke("set_nownodes_key", { value: key.value })
        .then(() => {
            key.value = "";
            return;
        })
        .catch((_error) => {
            return console.error(new Error("Failed submitting data"));
        });

    if (btc_address === null) {
        return console.error(new Error("Could not find #btc-address"));
    }

    invoke("set_btc_addresses", { value: [btc_address.value] })
        .then(() => {
            btc_address.value = "";
            return;
        })
        .catch((_error) => {
            return console.error(new Error("Failed submitting data"));
        });

    if (ltc_address === null) {
        return console.error(new Error("Could not find #ltc-address"));
    }

    invoke("set_ltc_addresses", { value: [ltc_address.value] })
        .then(() => {
            ltc_address.value = "";
            return;
        })
        .catch((_error) => {
            return console.error(new Error("Failed submitting data"));
        });
}