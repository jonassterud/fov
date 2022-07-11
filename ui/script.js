var ready = false;

window.onload = () => {
    const invoke = window.__TAURI__.invoke;

    if (invoke != undefined) {
        ready = true;
    }
}

