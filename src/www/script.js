// Global variables
var total_value = 0;
var assets = [];

// Events
window.onload = () => {
    try {
        load_all_assets().then(() => {
            create_diversification_chart();
        });
    } catch (error) {
        console.error(error);
    }
}

// Asset functions
function load_all_assets() {
    return Promise.all([
        load_sb1_assets(),
        load_cbp_assets(),
        load_nn_assets()
    ]);
}

function load_sb1_assets() {
    return load_assets("sb1/assets").then(data => {
        add_assets_to_table(data, "SpareBank 1")
    });
}

function load_cbp_assets() {
    return load_assets("cbp/assets").then(data => {
        add_assets_to_table(data, "Coinbase Pro")
    });
}

function load_nn_assets() {
    return load_assets("nn/assets").then(data => {
        add_assets_to_table(data, "Nordnet")
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
        assets.push(asset);

        // Add name and description together
        let name = asset.name + (asset.description ? ` - ${asset.description}` : "");
        // Only show ticker if currency/ticker is not NOK
        let ticker = asset.currency === "NOK" ? "" : asset.currency;
        // Only show balance if currency/ticker is not NOK
        let balance = asset.currency === "NOK" ? "" : asset.balance;
        // Calculate value of asset in NOK
        let value = Math.round(asset.value) + " NOK";
        total_value += asset.value;

        // Add asset into table 
        let table_row = `
        <tr>
            <td headers="name">${name}</td>
            <td headers="ticker">${ticker}</td>
            <td headers="balance">${balance}</td>
            <td headers="value">${value}</td>
        </tr>`;

        table_body.innerHTML += table_row;
    });

    // Update total value
    let total_value_cell = document.getElementById("total_value");

    if (total_value_cell === null) {
        throw new Error("Failed getting td with id: total_value");
    }

    total_value_cell.innerHTML = Math.round(total_value) + " NOK";
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

// Diversification functions
function create_diversification_chart() {
    let diversification = document.getElementById("diversification");
    let ul = document.getElementById("diversification_overview");
    let c = document.getElementById("diversification_chart");
    let cx = c.getContext("2d");

    if (diversification === null || c === null || ul === null) {
        return new Error("Failed getting one or more HTML elements");
    }

    let list_items = "";
    let prev_angle = 0;
    assets.forEach(asset => {
        let name = asset.name;
        let procentage = asset.value / total_value;
        let angle = procentage * Math.PI * 2;
        let random_color = `rgb(${Math.random() * 255}, ${Math.random() * 255}, ${Math.random() * 255})`;
        
        // Add to list
        list_items += `<li><span style="color: ${random_color}">▣  </span>${name}</li>`;

        // Draw pie
        cx.strokeStyle = "white";
        cx.fillStyle = random_color;
        cx.beginPath();
        cx.arc(c.width/2, c.height/2, c.width/2, prev_angle, prev_angle + angle, false);
        cx.lineTo(c.width/2, c.height/2);
        cx.closePath();
        cx.fill();
        cx.stroke();

        prev_angle += angle;
    });

    ul.innerHTML += list_items;
}