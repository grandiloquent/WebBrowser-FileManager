function showDeleteDialog(path) {
    const element = document.querySelector(`[path="${path}"]`);
    const dialog = document.createElement('custom-dialog');
    document.body.appendChild(dialog);
    let decodedPath = decodeURIComponent(path);
    dialog.appendChild(document.createTextNode(
        `您确定要删除 ${decodedPath} 吗`
    ));
    dialog.addEventListener('submit', async evt => {
        await requestDeleteFiles([
            decodedPath
        ]);
        element && element.remove();
    })

}