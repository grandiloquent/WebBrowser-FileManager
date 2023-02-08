bind();
customElements.whenDefined('custom-bottom-sheet').then(() => {
    customBottomSheet.data = [{
        title: "删除",
        id: 1
    }]
})

async function loadData(path) {
    // https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState

    window.history.pushState(null, null, `?path=${encodeURIComponent(path)}`);
    const res = await fetch(`/api/files?path=${encodeURIComponent(path)}`);
    return res.json();
}

async function render(path) {
    const res = await loadData(path || new URL(window.location).searchParams.get("path") || "C:\\Users\\Administrator\\Desktop");
    this.wrapper.innerHTML = res.sort((x, y) => {
        if (x.isDirectory !== y.isDirectory)
            if (x.isDirectory)
                return -1
            else
                return 1;
        return x.path.localeCompare(y.path)
    })
        .map(x => {
            return `<custom-item bind @submit="submit" ${x.isDirectory ? 'folder' : ''} title="${x.filename}" path="${encodeURIComponent(x.path)}" isDirectory="${x.isDirectory}"></custom-item>`
        }).join('');
    bind(this.wrapper);
}

render();

function onNewFile() {
    this.dialog.style.display = 'block';
    this.dialog.setAttribute('title', '新建文件');
    this.dialog.dataset.action = "1";
}

async function onDialogSubmit() {
    const dst = input.value.trim();
    if (!dst) return;
    const path = new URL(window.location).searchParams.get("path");
    const url = new URL(`${window.origin}/api/file`);
    url.searchParams.set("path", path);
    url.searchParams.set("action", this.dialog.dataset.action);
    url.searchParams.set("dst", dst);
    await fetch(url)
    location.reload();
}

function onNewFolder() {
    this.dialog.style.display = 'block';
    this.dialog.setAttribute('title', '新建文件');
    this.dialog.dataset.action = "2";
}

window.addEventListener("popstate", function (e) {
    window.location.href = location.href;
});
let detail;

function submit(evt) {
    if (evt.detail.id === '0') {
        if (evt.detail.isDirectory === "true") {
            render(evt.detail.path);
        } else {
            window.location = `/editor?path=${evt.detail.path}`
        }
    } else {
        detail = evt.detail;
        customBottomSheet.style.display = "block";
    }
}

function onCustomBottomSheetSubmit(evt) {
    evt.stopPropagation();

    customBottomSheet.style.display = 'none';
    if (evt.detail.id === '1') {
        dialogDelete.style.display = 'block';
        const div = document.createElement('div');
        div.textContent = decodeURIComponent(detail.path);
        dialogDelete.appendChild(div);
        //console.log(`/api/file?path=${encodeURIComponent(detail.path)}&action=3`)
        //    }
    }

}

async function onDialogDeleteSubmit() {
    await fetch(`/api/file?path=${encodeURIComponent(detail.path)}&action=3`);
    location.reload();
}