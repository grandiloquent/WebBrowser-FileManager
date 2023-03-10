/*
绑定元素和事件
例如：<div bind="div" @click="click"></div>
执行下列代码后，即可通过 this.div 访问该元素
在全局下编写click函数，即可自动绑定到该元素的click事件
*/
function bind(elememnt) {
    (elememnt || document).querySelectorAll('[bind]').forEach(element => {
        if (element.getAttribute('bind')) {
            window[element.getAttribute('bind')] = element;
        }
        [...element.attributes].filter(attr => attr.nodeName.startsWith('@')).forEach(attr => {
            if (!attr.value) return;
            element.addEventListener(attr.nodeName.slice(1), evt => {
                window[attr.value](evt);
            });
        });
    })
}

function camel(string) {
    return string.replaceAll(/[ _-]([a-zA-Z])/g, m => m[1].toUpperCase());
}

function encodeSVG(textarea) {
    const selectedString = getSelectedString(textarea).trim();
    let s = selectedString
        .replace("<svg", "<svg fill='rgb(25, 103, 210)' xmlns='http://www.w3.org/2000/svg' ")
        .replaceAll("\"", "'")
        .replaceAll(/[\r\n]+/g, '');
    replaceSelectedText(textarea, `
  background-size:36px 36px;
  background-position: center center;
            background-repeat: no-repeat;
            background-image:url("data:image/svg+xml;utf8,${s}");
  `)
}

function findBlock(textarea) {
    let start = textarea.selectionStart;
    let end = textarea.selectionEnd;
    const strings = textarea.value;
    if (strings[start] === '\n' && start - 1 > 0) {
        start--;
    }
    let founded = false;
    while (start > 0) {
        if (strings[start] == '\n') {
            let j = start - 1;
            while (j > 0 && /\s/.test(strings[j])) {
                if (strings[j] === '\n') {
                    founded = true;
                    break;
                }
                j--;
            }
        }
        if (founded) {
            break
        }
        start--;
    }
    founded = false;
    while (end + 1 < strings.length) {
        if (strings[end] == '\n') {
            let j = end + 1;
            while (j + 1 < strings.length && /\s/.test(strings[j])) {
                if (strings[j] === '\n') {
                    founded = true;
                    break;
                }
                j++;
            }
        }
        if (founded) {
            break
        }
        end++;
    }
    return [start, end]
}

function findCodeBlock(textarea) {
    const value = textarea.value;
    let start = textarea.selectionStart;
    let end = textarea.selectionEnd;
    while (start > -1) {
        if (value[start] === '`' && value[start - 1] === '`' && value[start - 2] === '`') {
            start += 1;
            while (start < value.length) {
                if (value[start] === '\n') {
                    start++;
                    break;
                }
                start++;
            }
            break;
        }
        start--;
    }
    while (end < value.length) {
        if (value[end] === '`' && value[end + 1] === '`' && value[end + 2] === '`') {
            end--;
            break;
        }
        end++;
    }
    return [start, end];
}

function findExtendPosition(editor) {
    const start = editor.selectionStart;
    const end = editor.selectionEnd;
    let string = editor.value;
    let offsetStart = start;
    while (offsetStart > 0) {
        if (!/\s/.test(string[offsetStart - 1])) offsetStart--; else {
            let os = offsetStart;
            while (os > 0 && /\s/.test(string[os - 1])) {
                os--;
            }
            if ([...string.substring(offsetStart, os).matchAll(/\n/g)].length > 1) {
                break;
            }
            offsetStart = os;
        }
    }
    let offsetEnd = end;
    while (offsetEnd < string.length) {
        if (!/\s/.test(string[offsetEnd + 1])) {
            offsetEnd++;
        } else {
            let oe = offsetEnd;
            while (oe < string.length && /\s/.test(string[oe + 1])) {
                oe++;
            }
            if ([...string.substring(offsetEnd, oe + 1).matchAll(/\n/g)].length > 1) {
                offsetEnd++;
                break;
            }
            offsetEnd = oe + 1;
        }
    }
    while (offsetStart > 0 && string[offsetStart - 1] !== '\n') {
        offsetStart--;
    }
    // if (/\s/.test(string[offsetEnd])) {
    //     offsetEnd--;
    // }
    return [offsetStart, offsetEnd];
}

function getLine() {
    let start = textarea.selectionStart;
    const strings = textarea.value;
    if (strings[start] === '\n' && start - 1 > 0) {
        start--;
    }
    while (start > 0 && strings[start] != '\n') {
        start--;
    }
    let end = textarea.selectionEnd;
    while (end - 1 < strings.length && strings[end] != '\n') {
        end++;
    }
    return [strings.substring(start, end), start, end]
}

function getSelectedString(textarea) {
    return textarea.value.substring(textarea.selectionStart, textarea.selectionEnd);
}

function humanFileSize(size) {
    if (size === 0) return '0';
    var i = Math.floor(Math.log(size) / Math.log(1024));
    return (size / Math.pow(1024, i)).toFixed(2) * 1 + ' ' + ['B', 'kB', 'MB', 'GB', 'TB'][i]
}

function jumpPage(textarea) {
    const line = getLine(textarea);
    const value = /(?<=(href|src)=")[^"]+(?=")/.exec(line);
    const path = new URL(window.location).searchParams.get("path");
    if (!value) {
        window.open('http://127.0.0.1:8081/' + substringBeforeLast(substringAfter(path, "\\app\\"), "."), "_blank");
        return
    }
    const src = `${window.location.origin}${window.location.pathname}?path=${encodeURIComponent(`${substringBeforeLast(path, "/")}/${value[0]}`)}`;
    window.open(src, '_blank');
}

async function loadData(path) {
    const res = await fetch(`/api/file?path=${encodeURIComponent(path)}`, {cache: "no-cache"});
    return res.text();
}

function onInsert() {
    this.textarea.setRangeText(`/*
  */`, this.textarea.selectionStart, this.textarea.selectionEnd)
}

async function onSave() {
    const path = new URL(window.location).searchParams.get("path");
    if (path.endsWith(".srt")) {
        textarea.value = textarea.value.replace(/WEBVTT\s+/,"").replaceAll(/\s*\d{2}:\d{2}:\d{2}\.\d{3} --> \d{2}:\d{2}:\d{2}\.\d{3}[\s]+/g, ' ');
        return;
    }
    const res = await fetch(`/api/file?path=${path}`, {
        method: 'POST', body: textarea.value
    });
    toast.setAttribute('message', '成功');
}

async function onTranslateChinese() {
    let array1 = getLine();
    textarea.setRangeText(`\n\n${await translate(array1[0], 'zh')}
          `, array1[2], array1[2], 'end');
}

async function onTranslateEnglish() {
    let array1 = getLine();
    textarea.setRangeText(`\n\n${await translate(array1[0], 'en')}
          `, array1[2], array1[2], 'end');
}

async function pasteCode() {
    let strings;
    if (typeof NativeAndroid !== 'undefined') {
        strings = NativeAndroid.readText()
    } else {
        strings = await navigator.clipboard.readText()
    }
    textarea.setRangeText(`
\`\`\`pgsql
${strings}
\`\`\`
`, textarea.selectionStart, textarea.selectionEnd, 'end');
}

function readText() {
    const textarea = document.createElement("textarea");
    textarea.style.position = 'fixed';
    textarea.style.right = '100%';
    document.body.appendChild(textarea);
    textarea.value = message;
    textarea.select();
    document.execCommand('paste');
    return textarea.value;
}

async function removeLines() {
    if (textarea.selectionStart === textarea.selectionEnd) {
        const p = findExtendPosition(textarea);
        let start = p[0];
        while (start > -1 && /\s/.test(textarea.value[start - 1])) {
            start--;
        }
        let end = p[1];
        while (end + 1 < textarea.value.length && /\s/.test(textarea.value[end + 1])) end++;
        if (typeof NativeAndroid !== 'undefined') {
            NativeAndroid.writeText(textarea.value.substring(start, end));
        } else {
            await navigator.clipboard.writeText(textarea.value.substring(start, end))
        }
        textarea.setRangeText('\n', start, end);
        textarea.selectionEnd = start;
    } else {
        textarea.value = textarea.value.substring(textarea.selectionEnd);
        textarea.selectionStart = 0;
        textarea.selectionEnd = 0;
        textarea.scrollLeft = 0;
        textarea.scrollTop = 0;
    }
}

async function render() {
    const res = await loadData(new URL(window.location).searchParams.get("path"));
    this.textarea.value = res;
}

function snake(string) {
    return string.replaceAll(/(?<=[a-z])[A-Z]/g, m => `_${m}`).toLowerCase()
        .replaceAll(/[ -]([a-z])/g, m => `_${m[1]}`)
}

function sortLines() {
    const points = findBlock(textarea);
    const lines = textarea.value.substring(points[0], points[1]).split('\n')
        .sort((x, y) => x.localeCompare(y));
    textarea.setRangeText(`\n\n${lines.join('\n')}`, points[0], points[1], 'end');
}

function substring(strings, prefix, suffix) {
    let start = strings.indexOf(prefix);
    if (start === -1) {
        return [0, 0]
    }
    start += prefix.length;
    let end = strings.indexOf(suffix, start);
    if (end === -1) {
        return [0, 0]
    }
    return [start, end]
}

function substringAfter(string, delimiter, missingDelimiterValue) {
    const index = string.indexOf(delimiter);
    if (index === -1) {
        return missingDelimiterValue || string;
    } else {
        return string.substring(index + delimiter.length);
    }
}

function substringAfterLast(string, delimiter, missingDelimiterValue) {
    const index = string.lastIndexOf(delimiter);
    if (index === -1) {
        return missingDelimiterValue || string;
    } else {
        return string.substring(index + delimiter.length);
    }
}

function substringBefore(string, delimiter, missingDelimiterValue) {
    const index = string.indexOf(delimiter);
    if (index === -1) {
        return missingDelimiterValue || string;
    } else {
        return string.substring(0, index);
    }
}

function substringBeforeLast(string, delimiter, missingDelimiterValue) {
    const index = string.lastIndexOf(delimiter);
    if (index === -1) {
        return missingDelimiterValue || string;
    } else {
        return string.substring(0, index);
    }
}

function substringNearest(string, index, start, end) {
    let j = index;
    while (j > -1) {
        if (start.indexOf(string[j]) !== -1) {
            j++
            break;
        }
        j--;
    }
    let k = index;
    while (k < string.length) {
        if (end.indexOf(string[k]) !== -1) {
            break;
        }
        k++;
    }
    return string.substring(j, k);
}

function tab(textarea) {
    textarea.addEventListener('keydown', function (e) {
        if (e.keyCode === 9) {
            const p = findExtendPosition(textarea);
            const start = this.selectionStart;
            textarea.setRangeText(
                textarea.value.substring(p[0], p[1])
                    .split('\n')
                    .map(i => {
                        return '\t' + i;
                    })
                    .join('\n'), p[0], p[1]);
            this.selectionStart = this.selectionEnd = start + 1;
            // prevent the focus lose
            e.preventDefault();
        }
    }, false);
}

function toBlocks(string) {
    let count = 0;
    let buf = [];
    const blocks = [];
    for (let i = 0; i < string.length; i++) {
        buf.push(string[i])
        if (string[i] === '{') {
            count++;
        } else if (string[i] === '}') {
            count--;
            if (count === 0) {
                blocks.push(buf.join(''))
                buf = [];
            }
        }
    }
    return blocks;
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
}

function tryUploadImageFromClipboard(success, error) {
    navigator.permissions.query({
        name: "clipboard-read"
    }).then(result => {
        if (result.state === "granted" || result.state === "prompt") {
            navigator.clipboard.read().then(data => {
                console.log(data[0].types);
                const blob = data[0].getType("image/png");
                console.log(blob.then(res => {
                    const formData = new FormData();
                    formData.append("images", res, "1.png");
                    fetch(`https://lucidu.cn/api/article/2`, {
                        method: "POST", body: formData
                    }).then(res => {
                        return res.text();
                    }).then(obj => {
                        success(obj);
                    })
                }).catch(err => {
                    console.log(err)
                    error(err);
                }))
            })
                .catch(err => {
                    error(err);
                });
        } else {
            error(new Error());
        }
    });
}

function uploadHanlder(editor) {
    tryUploadImageFromClipboard((ok) => {
        const string = `![](https://static.lucidu.cn/images/${ok})\n\n`;
        editor.setRangeText(string, editor.selectionStart, editor.selectionStart);
    }, () => {
        const input = document.createElement('input');
        input.type = 'file';
        input.addEventListener('change', async ev => {
            const file = input.files[0];
            const imageFile = await uploadImage(file, file.name);
            const string = `![](https://static.lucidu.cn/images/${imageFile})\n\n`;
            editor.setRangeText(string, editor.selectionStart, editor.selectionStart);
        });
        input.click();
    });
}

async function uploadImage(image, name) {
    const form = new FormData();
    form.append('images', image, name)
    const response = await fetch(`https://lucidu.cn/api/article/2`, {
        method: 'POST', body: form
    });
    return await response.text();
}

function upperCamel(string) {
    string = camel(string);
    return string.slice(0, 1).toUpperCase() + string.slice(1);
}

function writeText(message) {
    const textarea = document.createElement("textarea");
    textarea.style.position = 'fixed';
    textarea.style.right = '100%';
    document.body.appendChild(textarea);
    textarea.value = message;
    textarea.select();
    document.execCommand('copy');
    textarea.remove();
}

function openLink() {
    let start = textarea.selectionStart;
    let end = textarea.selectionEnd;
    while (start > -1 && textarea.value[start] !== ' ' && textarea.value[start] !== '(' && textarea.value[start] !== '\n') {
        start--;
    }
    while (end < textarea.value.length && textarea.value[end] !== ' ' && textarea.value[end] !== ')' && textarea.value[end] !== '\n') {
        end++;
    }

    if (textarea.selectionStart === textarea.selectionEnd) {
        window.open(textarea.value.substring(start + 1, end));
    } else {
        textarea.setRangeText(` [](${textarea.value.substring(start, end).trim()})`, start, end, 'end');
    }

}

function onCopy() {
    const pv = findCodeBlock(textarea);
    writeText(textarea.value.substring(pv[0], pv[1]));
}