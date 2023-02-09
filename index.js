async function loadData(path) {
    // https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState
    window.history.pushState(null, null, `?path=${encodeURIComponent(path)}`);
    const res = await fetch(`/api/files?path=${encodeURIComponent(path)}`);
    return res.json();
}

function onCustomBottomSheetSubmit(evt) {
    evt.stopPropagation();
    customBottomSheet.style.display = 'none';
    if (evt.detail.id === '1') {
        dialogDelete.style.display = 'block';
        const div = document.createElement('div');
        div.textContent = decodeURIComponent(detail.path);
        dialogDelete.appendChild(div);
    } else if (evt.detail.id === '2') {
        localStorage.setItem('path', decodeURIComponent(detail.path));
        customToast.setAttribute('message', '成功写入剪切板');
    }
}

async function onDialogDeleteSubmit() {
    await fetch(`/api/file?path=${encodeURIComponent(detail.path)}&action=3`);
    location.reload();
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

function onNewFile() {
    this.dialog.style.display = 'block';
    this.dialog.setAttribute('title', '新建文件');
    this.dialog.dataset.action = "1";
}

function onNewFolder() {
    this.dialog.style.display = 'block';
    this.dialog.setAttribute('title', '新建文件');
    this.dialog.dataset.action = "2";
}

async function render(path) {
    const res = await loadData(path || new URL(window.location).searchParams.get("path") || "C:\\Users\\Administrator\\Desktop");
    this.wrapper.innerHTML = res.sort((x, y) => {
        if (x.isDirectory !== y.isDirectory) if (x.isDirectory) return -1; else return 1;
        return x.path.localeCompare(y.path)
    })
        .map(x => {
            return `<custom-item bind @submit="submit" ${x.isDirectory ? 'folder' : ''} title="${x.filename}" path="${encodeURIComponent(x.path)}" isDirectory="${x.isDirectory}"></custom-item>`
        }).join('');
    bind(this.wrapper);
}

function submit(evt) {
    if (evt.detail.id === '0') {
        if (evt.detail.isDirectory === "true") {
            render(evt.detail.path);
        } else {
            if (/\.(?:mp4|m4a)$/.test(evt.detail.path)) {
                window.location = `/video?path=${evt.detail.path}`
            } else if (/\.(?:md|js|c|cpp|h|cs|css|html|java|txt|srt|vtt|cc|sql)$/.test(evt.detail.path)) {
                window.location = `/editor?path=${evt.detail.path}`
            } else {
                window.location = `/api/file?path=${evt.detail.path}`
            }
        }
    } else {
        detail = evt.detail;
        customBottomSheet.style.display = "block";
    }
}

function onFav() {
    fav.style.display = "block";
}

function onFavSubmit(evt) {
    switch (evt.detail.id) {
        case `1`:
            location = `?path=${encodeURIComponent("D:\\")}`;
            break
        case `2`:
            location = `?path=${encodeURIComponent("D:\\资源")}`;
            break
        case `3`:
            location = `?path=${encodeURIComponent("C:\\Users\\Administrator\\Desktop")}`;
            break
        case `4`:
            location = `?path=${encodeURIComponent("C:\\Users\\Administrator\\Downloads")}`;
            break
    }
}

async function onPaste() {
    const source = localStorage.getItem('path');
    localStorage.setItem('path', '');
    const path = new URL(window.location).searchParams.get("path");
    await fetch(`/api/file?path=${encodeURIComponent(source)}&dst=${path}&action=4`);
    //location.reload();
}

///////////////////////////
bind();
customElements.whenDefined('custom-bottom-sheet').then(() => {
    customBottomSheet.data = [{
        title: "删除", id: 1
    }, {
        title: "移动", id: 2
    }, {
        title: "粘贴", id: 3
    }]
    fav.data = [{
        title: "D:\\", id: 1
    }, {
        title: "D:\\资源", id: 2
    }, {
        title: "桌面", id: 3
    }, {
        title: "下载", id: 4
    }]
})


render();


window.addEventListener("popstate", function (e) {
    window.location.href = location.href;
});
let detail;
