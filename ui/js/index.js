const invoke = window.__TAURI__.invoke;

window.onload = () => {
    let new_button = document.querySelector("#new-button");
    let load_button = document.querySelector("#load-button");
    let save_button = document.querySelector("#save-button");
    let refresh_button = document.querySelector("#refresh-button");

    new_button.addEventListener("click", new_portfolio);
    load_button.addEventListener("click", load_portfolio);
    save_button.addEventListener("click", save_portfolio);
    refresh_button.addEventListener("click", refresh_portfolio);
}

function new_portfolio() {
    let name = prompt("What would you like to call your portfolio?");
    invoke("new_portfolio", {name: name});
}

function load_portfolio() {
    invoke("get_portfolio_names").then((portfolio_names) => {
        let name = prompt(portfolio_names + "\n\nWhich portfolio do you wish to load?");
        let password = prompt("Please enter password:")
        invoke("load_portfolio", {name: name, password: password});
    });
    
}

function save_portfolio() {
    let password = prompt("Please enter password to use for encryption:");
    invoke("save_portfolio", {password: password});
}

function refresh_portfolio() {
    invoke("update_assets");
}