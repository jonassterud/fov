window.onload = () => {
    load_all_assets();
}

function load_all_assets() {
    load_sb1_assets();
    load_cbp_assets();
    load_nn_assets();
    // ...
}

function load_sb1_assets() {
    load_assets("sb1/assets").then(data => {
        add_assets_to_table(data, "SpareBank 1")
    }).catch(error => {
        console.error(error);
    });
}

function load_cbp_assets() {
    load_assets("cbp/assets").then(data => {
        add_assets_to_table(data, "Coinbase Pro")
    }).catch(error => {
        console.error(error);
    });
}

function load_nn_assets() {
    load_assets("nn/assets").then(data => {
        add_assets_to_table(data, "Nordnet")
    }).catch(error => {
        console.error(error);
    });
}

function add_assets_to_table(data, title) {
    let table_body = document.getElementById("asset_table");
    if (table_body === null) {
        throw new Error("Failed getting tbody with id: asset_table");
    }

    // Add header for this asset group
    let table_header = `<tr><th scope="row" rowspan="${data.length + 1}">${title}</th></tr>`;
    table_body.innerHTML += table_header;

    // Add assets
    data.forEach(asset => {
        let table_row = `
        <tr>
            <td headers="name">${asset.name}</td>
            <td headers="ticker">${asset.currency}</td>
            <td headers="balance">${asset.balance}</td>
            <td headers="value">${"unknown"}</td>
        </tr>`;

        table_body.innerHTML += table_row;
    });
}

function load_assets(path) {
    return fetch(window.location.href + path)
        .then(res => {
            if (!res.ok) {
                throw new Error(`API request failed with status ${res.status}`);
            }

            return res.json();
        })
}