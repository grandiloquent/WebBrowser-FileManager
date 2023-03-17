
document.querySelectorAll('[bind]').forEach(element => {
    if (element.getAttribute('bind')) {
        window[element.getAttribute('bind')] = element;
    }
    [...element.attributes].filter(attr => attr.nodeName.startsWith('@')).forEach(attr => {
        if (!attr.value) return;
        element.addEventListener(attr.nodeName.slice(1), evt => {
            window[attr.value](evt);
        });
    });
});

const id = new URL(window.location).searchParams.get("id");
let baseUri = window.location.host === '127.0.0.1:5500' ? 'http://192.168.8.55:10808' : '';
render();

async function render() {
    //     textarea.value = localStorage.getItem("content");

    //     if (id) {
    //         try {
    //             const obj = await loadData();
    //             document.title = obj.title;
    //             textarea.value = `# ${obj.title}|${obj.tag}

    // ${obj.content.trim()}
    //         `
    //         } catch (error) {
    //             console.log(error)
    //         }
    //     }
    const obj = await loadData();
    const md = new markdownit({
        linkify: true
    });

    wrapper.innerHTML = md.render(obj);
}

async function loadData() {
    const path = new URL(window.location).searchParams.get("path");
    const res = await fetch(`/api/file?path=${encodeURIComponent(path)}`, {cache: "no-cache"});
    return res.text();
}

for (const iterator of Object.keys(window)) {
    console.log(iterator)
}