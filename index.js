async function loadData(path) {
    // https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState
    //window.history.pushState(null, null, `?path=${encodeURIComponent(path)}`);


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
    } else if (evt.detail.id === '4') {
        fetch(`/api/zip?path=${detail.path}`)
    } else if (evt.detail.id === '5') {
        input.value = substringAfterLast(decodeURIComponent(detail.path), '\\');
        this.dialog.dataset.path = detail.path;
        this.dialog.style.display = 'block';
        this.dialog.setAttribute('title', '重命名');
        this.dialog.dataset.action = "5";
    } else if (evt.detail.id === '6') {
        fetch(`/api/file?action=10&path=${detail.path}`);
    }
}

async function onDialogDeleteSubmit() {
    await fetch(`/api/file?path=${encodeURIComponent(detail.path)}&action=3`);
    location.reload();
}

async function onDialogSubmit() {
    const dst = input.value.trim();
    if (!dst) return;
    const path = this.dialog.dataset.path || new URL(window.location).searchParams.get("path");
    this.dialog.dataset.path = '';
    const url = new URL(`${window.origin}/api/file`);
    url.searchParams.set("path", path || "C:\\Users\\Administrator\\Desktop");
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

function substringAfterLast(string, delimiter, missingDelimiterValue) {
    const index = string.lastIndexOf(delimiter);
    if (index === -1) {
        return missingDelimiterValue || string;
    } else {
        return string.substring(index + delimiter.length);
    }
}

async function render(path) {
    path = path || new URL(window.location).searchParams.get("path") || "C:\\Users\\Administrator\\Desktop";
    document.title = substringAfterLast(decodeURIComponent(path), "\\")
    const res = await loadData(path);
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
    const encodedPath = encodeURIComponent(evt.detail.path);
    if (evt.detail.id === '0') {
        if (evt.detail.isDirectory === "true") {

            const url = new URL(window.location);
            //url.searchParams.set('path', path);
            window.history.pushState({}, '', `?path=${encodedPath}`);
            render(evt.detail.path);
        } else {
            if (/\.(?:mp4|m4a)$/.test(evt.detail.path)) {
                window.location = `/video?path=${encodedPath}`
            } else if (evt.detail.path.endsWith(".srt")) {
                window.location = `/srt?path=${encodeURIComponent(evt.detail.path)}`
            }
                // else if (evt.detail.path.endsWith(".md")) {
                //     window.location = `/markdown?path=${encodeURIComponent(evt.detail.path)}`
            // }
            else if (decodeURIComponent(evt.detail.path).indexOf("\\Books\\") === -1 && /\.(?:md|js|c|cpp|h|cs|css|html|java|txt|srt|vtt|cc|sql)$/.test(evt.detail.path)) {
                window.location = `/editor?path=${encodedPath}`
            } else {
                window.location = `/api/file?path=${encodedPath}`
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

    let path;
    switch (evt.detail.id) {
        case `1`:
            path = "D:\\Books";
            break
        case `2`:
            path = "D:\\资源";
            break
        case `3`:
            path = "C:\\Users\\Administrator\\Desktop";
            break
        case `4`:
            path = "C:\\Users\\Administrator\\Downloads";
            break
        case '5':
            path = "D:\\Resources\\Videos"
            break;
    }
    const url = new URL(window.location);
    url.searchParams.set('path', path);
    window.history.pushState({}, '', url);
    window.location = url;
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
    }, {
        title: "解压", id: 4
    }, {
        title: "重命名", id: 5
    }, {
        title: "整理", id: 6
    }]
    fav.data = [{
        title: "D:\\Books", id: 1
    }, {
        title: "D:\\资源", id: 2
    }, {
        title: "桌面", id: 3
    }, {
        title: "下载", id: 4
    }, {
        title: "视频", id: 5
    }]
})


render();


window.addEventListener("popstate", function (e) {

    let path = new URL(location).searchParams.get('path');
    // if (path)
    //     location = `?path=${path}`;
    window.location = location;
    console.log(decodeURIComponent(path))
    //location.reload();
});
let detail;

document.addEventListener('keydown', evt => {
    console.log(evt.key)
    if (evt.key === 'Enter') {
        evt.preventDefault();
        fetch(`/api/cmd?q=explorer ${new URL(location).searchParams.get('path')}`)
    }
})