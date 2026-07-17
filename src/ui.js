const { invoke } = window.__TAURI__.core;


const checkBox = document.getElementById('wipe-downloads-checkbox');
const warningBox = document.getElementById('downloads-warning');

checkBox.addEventListener('change', (e) =>{
    if(e.target.checked){
        warningBox.style.display = 'block';
    }else{
        warningBox.style.display = 'none';
    }
})

document.getElementById('evp-btn').addEventListener('click', async() => {
    updateStatus("Closing active browsers...")

    const browserState = document.getElementById('browser-state');
    const cookiesState = document.getElementById('cookies-state');
    const downloadsState = document.getElementById('downloads-state');
    const clipboardState = document.getElementById('clipboard-state');
    const usbState = document.getElementById('usb-state')
    
    const shouldWipeDownloads = checkBox.checked;

    try{
        browserState.innerText = "Cleaning browser...";
        browserState.className = "processing";

        cookiesState.innerText = "Zeroing files...";
        cookiesState.className = "processing";

        if (shouldWipeDownloads) {
            downloadsState.innerText = "Purging downloads...";
            downloadsState.className = "processing";
        } else {
            downloadsState.innerText = "Skipping...";
            downloadsState.className = "processing";
        }

        let result = await invoke('run_sanitize_routine' , {
            shouldWipeDownloads: shouldWipeDownloads
        });

        browserState.innerText = "Cleared";
        browserState.className = "success";

        cookiesState.innerText = "Wiped & Shredded";
        cookiesState.className = "success";

        if (shouldWipeDownloads) {
            downloadsState.innerText = "Purged";
            downloadsState.className = "success";
        } else {
            downloadsState.innerText = "Skipped";
            downloadsState.className = "waiting";
        }

        clipboardState.innerText = "Wiped";
        clipboardState.className = "success";

        usbState.innerText = "Ejecting Safely...";
        usbState.className = "processing";

        setTimeout(() => {
            usbState.innerText = "Safe to Remove";
            usbState.className = "ejected";
            updateStatus("STATUS: Footprint cleared... Ejecting USB...");
        }, 800);

    } catch (error) {
        // Fallback catch block if the backend command errors out
        updateStatus(`STATUS: Error occurred: ${error}`);
        
        browserState.innerText = "Failed";
        browserState.className = "error";
        
        cookiesState.innerText = "Failed";
        cookiesState.className = "error";

        downloadsState.innerText = "Failed";
        downloadsState.className = "error";
    }
});

function updateStatus(msg){
    document.getElementById('console-text').innerText = msg;
}