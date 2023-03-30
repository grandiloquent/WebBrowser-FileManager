
function insertPathLocalStorage(newPath) {
    const pathData = localStorage.getItem('paths');
    let path = (pathData && JSON.parse(pathData)) || [];
    path.push(decodeURIComponent(newPath));
    // 移除数组中的重复项
    path = [...new Set(path)];
    localStorage.setItem('paths', JSON.stringify(path));
}

function removePathLocalStorage(newPath) {
    const pathData = localStorage.getItem('paths');
    let path = (pathData && JSON.parse(pathData)) || [];
    let index = path.indexOf(newPath);
    if (index !== -1)
        path.splice(index, 1);
    // 移除数组中的重复项
    path = [...new Set(path)];
    localStorage.setItem('paths', JSON.stringify(path));
}

function getPaths() {
    const pathData = localStorage.getItem('paths');
    if (!pathData) {
        return null;
    }

    return JSON.parse(pathData);
}

async function requestMoveFiles() {
    const paths = getPaths();
    const path = new URL(window.location).searchParams.get("path");
    const response = await fetch(`/api/files/move?dst=${path}`, {
        method: "POST",
        body: JSON.stringify(paths)
    });
    localStorage.setItem('paths', '');
    return response.json();
}

function launchPasteDialog() {
    const customPathsBottomSheet = document.createElement('custom-paths-bottom-sheet');
    document.body.appendChild(customPathsBottomSheet);
    const paths = getPaths();
    customPathsBottomSheet.data = paths;
    customPathsBottomSheet.addEventListener('submit',async evt=>{
        await requestMoveFiles();
    })

}
