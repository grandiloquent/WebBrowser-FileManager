export async function readText() {
    let strings;
    if (typeof NativeAndroid !== 'undefined') {
        strings = NativeAndroid.readText()
    } else {
        strings = await navigator.clipboard.readText()
    }
    return strings
}

async function translate(value, to) {
    try {
        const response = await fetch(`${window.location.protocol}//kpkpkp.cn/api/trans?q=${encodeURIComponent(value.trim())}&to=${to}`);
        const obj = await response.json();
        return obj.sentences.map((element, index) => {
            return element.trans;
        }).join(' ');
    } catch (error) {
        console.log(error);
    }
    return null;
}

export async function onTranslateChinese(q) {
    let strings = (await translate(q, 'zh')) || "";
    const patterns = loadPatterns();
    if (patterns) {
        for (let index = 0; index < this.patterns.length; index++) {
            const element = this.patterns[index];
            strings = strings.replaceAll(new RegExp(
                element[0], 'g'
            ), element[1])
        }
    }
    return strings;
}

function loadPatterns() {
    let strings;
    if (typeof NativeAndroid !== 'undefined') {
        strings = NativeAndroid.getString("pattern")
    } else {
        strings = localStorage.getItem('pattern')
    }
    return (strings && strings.split('\n')
        .filter(x => x.trim())
        .map(x => x.trim().split('|'))) || null;
}