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
    display_portfolio();
}

function display_portfolio() {
    let asset_table = document.querySelector("#asset-table");
    
    invoke("get_assets").then((json) => {   
        let assets_map = new Map();

        // Group assets by their tag
        JSON.parse(json).forEach((object) => {
            if (!assets_map.get(object.tag)) {
                assets_map.set(object.tag, []);
            } else {
                let value = assets_map.get(object.tag);
                value.push(object);
                assets_map.delete(object.tag);
                assets_map.set(object.tag, value);
            }
        });

        // Fill table
        assets_map.forEach((assets, tag) => {
            // Add header for this asset group
            let table_header = `<tr><th scope="row" rowspan="${assets.length + 1}">${tag}</th></tr>`;
            asset_table.innerHTML += table_header;

            assets.forEach((asset) => {
                // Add name and description together
                let name = asset.name + (asset.description ? ` - ${asset.description}` : "");
                // Only show ticker if currency/ticker is not NOK
                let ticker = asset.currency != "NOK" ? asset.currency : "";
                // Only show balance if currency/ticker is not NOK
                let balance = asset.currency != "NOK" ? asset.balance : "";
                // Calculate value of asset in NOK
                let value = Math.round(asset.value).toLocaleString('no-NO', { style: 'currency', currency: 'NOK' });

                // Add asset into table 
                let table_row = `
                <tr>
                    <td headers="name">${name}</td>
                    <td headers="ticker">${ticker}</td>
                    <td headers="balance">${balance}</td>
                    <td headers="value">${value}</td>
                </tr>`;

                asset_table.innerHTML += table_row;
            });
        });
    });
}