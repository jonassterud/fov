function load_sb1_assets() {
    load_assets("sb1/assets").then(data => {
        add_assets(data, "sb1_assets")
    }).catch(error => {
        console.error(error);
    });
}

function load_cbp_assets() {
    load_assets("cbp/assets").then(data => {
        add_assets(data, "cbp_assets")
    }).catch(error => {
        console.error(error);
    });
}

function load_nn_assets() {
    load_assets("nn/assets").then(data => {
        add_assets(data, "nn_assets")
    }).catch(error => {
        console.error(error);
    });
}

function add_assets(data, container_id) {
    let container = document.getElementById(container_id);

    if (container === null) {
        throw new Error(`Failed getting container with id: ${container_id}`)
    }

    data.forEach(asset => {
        let text_node = `
        <span class="asset">
            <span class="asset_name">${asset.name}</span>
            <span class="asset_description">${asset.description ? asset.description : ""}</span>
            <span class="asset_balance">${asset.balance} ${asset.currency}</span>
        </span>
        <br>`;

        container.innerHTML += text_node;
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