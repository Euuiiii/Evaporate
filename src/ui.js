const { invoke } = window.__TAURI__.core;

document.getElementById('evp-btn').addEventListener('click', async() => {
    updateStatus("Closing active browsers...")

    try{
        let result = await invoke('run sanitize routine');
        updateStatus("Footprint cleared...Ejecting USB...");
    } catch(error){
        updateStatus(`Error occured: ${error}`);
    }
});

function updateStatus(msg){
    document.getElementById('console-text').innerText = msg;
}