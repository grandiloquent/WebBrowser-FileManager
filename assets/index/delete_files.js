async function requestDeleteFiles(paths) {
    
    const response = await fetch(`/api/file/delete`, {
        method: "POST",
        body: JSON.stringify(paths)
    });
    localStorage.setItem('paths', '');
    return response.text();
}

function launchDeleteDialog() {
    const customPathsBottomSheet = document.createElement('custom-paths-bottom-sheet');
    document.body.appendChild(customPathsBottomSheet);
    const paths = getPaths();
    customPathsBottomSheet.data = paths;
    customPathsBottomSheet.addEventListener('submit', async evt => {
        const paths = getPaths();
        await requestDeleteFiles( paths);
    })

}