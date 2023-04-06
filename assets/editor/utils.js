function getSearchParams(name) {
    return new URL(window.location).searchParams.get(name)
}

async function getString(uri) {
    const res = await fetch(uri);
    return res.text();
}

function joinPath(filename) {
    let path = getSearchParams('path') || `C:\\Users\\Administrator\\Desktop`;
    return encodeURIComponent(`${path}\\${filename}`)
}
