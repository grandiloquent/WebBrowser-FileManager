(() => {

    class CustomDialog extends HTMLElement {

        constructor() {
            super();
            this.attachShadow({
                mode: 'open'
            });
            const wrapper = document.createElement("div");
            wrapper.setAttribute("class", "wrapper");
            const style = document.createElement('style');
            style.textContent = `.wrapper{position:fixed;top:0;left:0;right:0;bottom:0;display:flex;-webkit-box-align:center;align-items:center;-webkit-box-pack:center;justify-content:center;z-index:4;margin:0 40px;padding:0 env(safe-area-inset-right) env(safe-area-inset-bottom) env(safe-area-inset-left)}.dialog{position:relative;z-index:2;display:flex;-webkit-box-orient:vertical;-webkit-box-direction:normal;flex-direction:column;max-height:100%;box-sizing:border-box;padding:16px;margin:0 auto;overflow-x:hidden;overflow-y:auto;font-size:13px;color:#0f0f0f;border:none;min-width:250px;max-width:356px;box-shadow:0 0 24px 12px rgba(0,0,0,.25);border-radius:12px;background-color:#fff}.dialog-header{display:flex;-webkit-box-orient:vertical;-webkit-box-direction:normal;flex-direction:column;flex-shrink:0}.h2{margin:0 0 3px;display:-webkit-box;-webkit-box-orient:vertical;max-height:2.5em;-webkit-line-clamp:2;overflow:hidden;line-height:1.25;text-overflow:ellipsis;font-weight:normal;font-size:18px}.dialog-body{overflow-y:auto;overflow-x:hidden;max-height:100vh}.dialog-buttons{display:flex;-webkit-box-align:center;align-items:center;-webkit-box-pack:end;justify-content:flex-end;margin-top:12px}.button{display:flex;align-items:center;justify-content:center;padding:0 16px;height:36px;font-size:14px;line-height:36px;border-radius:18px;color:#0f0f0f}.disabled{color:#909090}.overlay{position:fixed;top:0;bottom:0;left:0;right:0;z-index:1;cursor:pointer;background-color:rgba(0,0,0,.3)}input,textarea{background-color:transparent;padding-bottom:4px;outline:none;box-sizing:border-box;border:none;border-radius:0;margin-bottom:1px;font:inherit;color:#0f0f0f}textarea{-webkit-appearance:none;appearance:none;min-height:8.4rem;width:100%;border:1px solid rgba(0,0,0,.1);padding:8px}`;
            this.wrapper = wrapper;
            this.shadowRoot.append(style, wrapper);
        }

        static get observedAttributes() {
            return ['title'];
        }

        set title(name) {
            this._title.textContent = name;
        }

        set content(value) {
            this.textarea.value = value;
        }

        get content() {
            return this.textarea.value;
        }

        navigate(e) {
            this.dispatchEvent(new CustomEvent('submit', {
                detail: e.currentTarget.dataset.href
            }));
            this.remove();
        }

        _close(evt) {
            evt.stopPropagation();
            this.remove();
        }

        _submit(evt) {
            evt.stopPropagation();
            this.remove();
            this.dispatchEvent(new CustomEvent('submit', {
                detail: this.textarea.value
            }));
        }

        connectedCallback() {
            this.wrapper.innerHTML = `<div class="dialog">
      <div class="dialog-header">
        <h2 bind="_title" class="h2">
          ${this.getAttribute("title")}
        </h2>
      </div>
      <div bind="_message" class="dialog-body">
        <textarea bind="textarea"></textarea>
      </div>
      <div class="dialog-buttons">
        <div bind class="button" @click="_close">
          取消
        </div>
        <div bind class="button disabled" @click="_submit">
          确定
        </div>
      </div>
    </div>
    <div bind class="overlay" @click="_close">
    </div>`;
            this.wrapper.querySelectorAll('[bind]').forEach(element => {
                if (element.getAttribute('bind')) {
                    this[element.getAttribute('bind')] = element;
                }
                [...element.attributes].filter(attr => attr.nodeName.startsWith('@')).forEach(attr => {
                    if (!attr.value) return;
                    element.addEventListener(attr.nodeName.slice(1), evt => {
                        this[attr.value](evt);
                    });
                });
            })
        }

        attributeChangedCallback(name, oldValue, newValue) {
        }
    }

    customElements.define('custom-dialog', CustomDialog);


})();
(() => {
    class CustomFavorites extends HTMLElement {

        constructor() {
            super();
            this.attachShadow({
                mode: 'open'
            });
            const wrapper = document.createElement("div");
            wrapper.setAttribute("class", "wrapper");
            const style = document.createElement('style');
            style.textContent = `.icon{display:inline-block;flex-shrink:0;width:24px;height:24px;fill:currentColor;stroke:none;margin-right:12px}button{padding:0;border:none;outline:none;font:inherit;text-transform:inherit;color:inherit;background:transparent}button{cursor:pointer;box-sizing:border-box;text-align:initial;text-transform:unset;width:100%;display:flex;padding:0;margin-left:12px;font-size:1.6rem;line-height:2.2rem}.menu-item{padding:0;height:48px;display:flex;-webkit-box-align:center;align-items:center}.bottom-sheet-layout-content-wrapper{-webkit-box-flex:1;flex:1;overflow-y:scroll;max-height:379.2px}.bottom-sheet-layout-header-title-wrapper{-webkit-box-orient:vertical;-webkit-box-direction:normal;flex-direction:column;display:flex;margin-left:12px}.bottom-sheet-layout-header{-webkit-box-pack:justify;justify-content:space-between;display:flex;margin-top:8px}.bottom-sheet-drag-line{background:#0f0f0f;opacity:.15;border-radius:4px;height:4px;margin:0 auto;width:40px;margin-top:8px}.bottom-sheet-layout-header-wrapper{overflow:hidden;-webkit-box-flex:0;flex:none;border-bottom:1px solid #fff}.bottom-sheet-layout{border-radius:12px;background-color:#fff;display:block;overflow:hidden;position:fixed;margin:0 8px 24px;bottom:0;left:0;right:0;z-index:2}.overlay{position:fixed;top:0;bottom:0;left:0;right:0;z-index:1;cursor:pointer;background-color:rgba(0,0,0,.6)}.wrapper{position:fixed;z-index:5}.buttons{display:grid;grid-template-columns:repeat(2,1fr);width:100%;background:#dadce0;margin:0 12px;gap:1px;height:100%}.buttons>button{background:#fff;margin-left:0;display:flex;align-items:center;justify-content:center}`;
            this.wrapper = wrapper;
            this.shadowRoot.append(style, wrapper);
        }

        set data(value) {

            value && this.contentWrapper.insertAdjacentHTML('afterbegin', value.map(element => {
                const html = `<div bind @click="submit" data-path="${element}" class="menu-item">
      <button style="display: flex; align-items: center;">
        <div style="flex-grow: 1; text-overflow: ellipsis; overflow: hidden;">
          ${element}
        </div>
        <div bind @click="removeItem" class="icon">
          <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24">
            <path d="M12.71,12l8.15,8.15l-0.71,0.71L12,12.71l-8.15,8.15l-0.71-0.71L11.29,12L3.15,3.85l0.71-0.71L12,11.29l8.15-8.15l0.71,0.71 L12.71,12z">
            </path>
          </svg>
        </div>
      </button>
    </div>`
                return html;
            }).join(''));
            this.bind(this.contentWrapper);
        }

        close() {
            this.remove();
        }

        removeItem(evt) {
            evt.stopPropagation();
            const element = evt.currentTarget.parentNode.parentNode;
            this.dispatchEvent(new CustomEvent('remove', {
                detail: element.dataset.path
            }));
            element.remove();
        }

        submit(evt) {
            this.dispatchEvent(new CustomEvent('submit', {
                detail: evt.currentTarget.dataset.path
            }));
            this.remove();
        }

        connectedCallback() {
            const html = `<div bind @click="close" class="overlay">
    </div>
    <div class="bottom-sheet-layout">
      <div class="bottom-sheet-layout-header-wrapper">
        <div class="bottom-sheet-drag-line">
        </div>
        <div class="bottom-sheet-layout-header">
          <div class="bottom-sheet-layout-header-title-wrapper">
          </div>
        </div>
      </div>
      <div bind="contentWrapper" class="bottom-sheet-layout-content-wrapper">
      </div>
    </div>`;
            this.wrapper.innerHTML = html;
            this.bind(this.wrapper);
        }

        bind(parent) {
            // <div bind="name" @click="close"/>
            // 绑定 div 元素, 绑定后可以通过 this.name 访问该元素
            // @click：@+事件名称 将该元素的单击事件绑定到 this.close 方法
            parent.querySelectorAll('[bind]').forEach(element => {
                if (element.getAttribute('bind')) {
                    this[element.getAttribute('bind')] = element;
                }
                [...element.attributes].filter(attr => attr.nodeName.startsWith('@')).forEach(attr => {
                    if (!attr.value) return;
                    element.addEventListener(attr.nodeName.slice(1), evt => {
                        this[attr.value](evt);
                    });
                });
            });
        }

    }

    customElements.define('custom-favorites', CustomFavorites);
})();
(() => {
    function copyLine(editor, count) {
        const start = editor.selectionStart;
        const end = editor.selectionEnd;
        const string = editor.value;
        let offsetStart = start;
        while (offsetStart > 0) {
            if (string[offsetStart - 1] !== '\n' && string[offsetStart - 1] !== '|')
                offsetStart--;
            else {
                // while (offsetStart > 0) {
                //     if (/\s/.test(string[offsetStart - 1]))
                //         offsetStart--;
                //     else break;
                // }
                break;
            }
        }
        let offsetEnd = end;
        while (offsetEnd < string.length) {
            if (string[offsetEnd + 1] !== '\n' && string[offsetEnd + 1] !== '|')
                offsetEnd++;
            else {
                /* while (offsetEnd < string.length) {
                     if (/\s/.test(string[offsetEnd + 1]))
                         offsetEnd++;
                     else break;
                 }*/
                offsetEnd++;
                break;
            }
        }
        const str = string.substring(offsetStart, offsetEnd).trim();
        writeText(str);
        editor.focus()
    }

    function substringBefore(string, delimiter, missingDelimiterValue) {
        const index = string.indexOf(delimiter);
        if (index === -1) {
            return missingDelimiterValue || string;
        } else {
            return string.substring(0, index);
        }
    }

    function substringAfter(string, delimiter, missingDelimiterValue) {
        const index = string.indexOf(delimiter);
        if (index === -1) {
            return missingDelimiterValue || string;
        } else {
            return string.substring(index + delimiter.length);
        }
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

    function findCodeBlockExtend(textarea) {
        const value = textarea.value;
        let start = textarea.selectionStart;
        let end = textarea.selectionEnd;
        while (start > -1) {
            if (value[start] === '`' && value[start - 1] === '`' && value[start - 2] === '`') {
                // start += 1;
                start -= 2;
                while (start - 1 > 0 && value[start - 1] !== '\n') {
                    start--;
                }
                break;
            }
            start--;
        }
        while (end < value.length) {
            if (value[end] === '`' && value[end + 1] === '`' && value[end + 2] === '`') {
                end += 2;
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

    function formatHead(editor, count) {
        // console.log("formatHead, ");
        // let start = editor.selectionStart;
        // const string = editor.value;
        // while (start - 1 > -1 && string.charAt(start - 1) !== '\n') {
        //     start--;
        // }
        // editor.setRangeText('#'.repeat(count || 2) + " ", start, start);
        const start = editor.selectionStart;
        const end = editor.selectionEnd;
        const string = editor.value;
        let offsetStart = start;
        while (offsetStart > 0) {
            if (string[offsetStart - 1] !== '\n')
                offsetStart--;
            else {
                while (offsetStart > 0) {
                    if (/\s/.test(string[offsetStart - 1]))
                        offsetStart--;
                    else break;
                }
                break;
            }
        }
        let offsetEnd = end;
        while (offsetEnd < string.length) {
            if (string[offsetEnd + 1] !== '\n')
                offsetEnd++;
            else {
                while (offsetEnd < string.length) {
                    if (/\s/.test(string[offsetEnd + 1]))
                        offsetEnd++;
                    else break;
                }
                break;
            }
        }
        editor.setRangeText(`\n\n${'#'.repeat(count)} ${string.substring(offsetStart, offsetEnd).trim()}\n`, offsetStart,
            offsetEnd, 'end');
    }

    function getLine(extended) {
        let start = textarea.selectionStart;
        const strings = textarea.value;
        if (strings[start] === '\n' && start - 1 > 0) {
            start--;
        }
        while (start > 0 && strings[start - 1] !== '\n') {
            start--;
        }
        if (extended) {
            while (start + 1 < strings.length && /\s/.test(strings[start])) {
                start++
            }
        }
        let end = textarea.selectionEnd;
        while (end + 1 < strings.length && strings[end] !== '\n') {
            end++;
        }
        return [strings.substring(start, end), start, end]
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
                        fetch(`https://lucidu.cn/v1/picture`, {
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

    function getIndexLine(textarea, index) {
        let start = index || textarea.selectionStart;
        const strings = textarea.value;
        if (strings[start] === '\n' && start - 1 > 0) {
            start--;
        }
        while (start > 0 && strings[start - 1] !== '\n') {
            start--;
        }
        let end = index || textarea.selectionEnd;
        while (end + 1 < strings.length && strings[end] !== '\n') {
            end++;
        }
        return [strings.substring(start, end), start, end]
    }

    async function uploadImage(image, name) {
        const form = new FormData();
        form.append('images', image, name)
        const response = await fetch(`https://lucidu.cn/v1/picture`, {
            method: 'POST', body: form
        });
        return await response.text();
    }

    function getContinueBlock(textarea) {
        let start = textarea.selectionStart;
        const strings = textarea.value;
        if (strings[start] === '\n' && start - 1 > 0) {
            start--;
        }
        while (start > 0) {
            if (strings[start - 1] === '\n') {
                let j = start - 1;
                while (j > 0 && strings[j - 1] !== '\n')
                    j--;
                if (!strings.substring(start, j).trim()) {
                    break
                }
            }
            start--;
        }
        let end = textarea.selectionEnd;

        while (end + 1 < strings.length) {
            if (strings[end] === '\n') {
                let j = end;
                while (j + 1 < strings.length && strings[++j] !== '\n') ;
                if (!strings.substring(end, j).trim()) {
                    break
                }
            }
            end++;
        }
        return [start, end];
    }

    class CustomActions extends HTMLElement {

        constructor() {
            super();
            this.attachShadow({
                mode: 'open'
            });
            const wrapper = document.createElement("div");
            wrapper.setAttribute("class", "wrapper");
            const style = document.createElement('style');
            style.textContent = `.icon{display:inline-block;flex-shrink:0;width:24px;height:24px;fill:currentColor;stroke:none;margin-right:12px}button{padding:0;border:none;outline:none;font:inherit;text-transform:inherit;color:inherit;background:transparent}button{cursor:pointer;box-sizing:border-box;text-align:initial;text-transform:unset;width:100%;display:flex;padding:0;margin-left:12px;font-size:1.6rem;line-height:2.2rem}.menu-item{padding:0;height:48px;display:flex;-webkit-box-align:center;align-items:center;background:#fff;font-size:1.6rem;line-height:2.2rem;justify-content:center}.bottom-sheet-layout-content-wrapper{-webkit-box-flex:1;flex:1;overflow-y:scroll;display:grid;grid-template-columns:repeat(3,1fr);background:#dadce0;gap:1px;border:1px solid #dadce0}.bottom-sheet-layout-header-title-wrapper{-webkit-box-orient:vertical;-webkit-box-direction:normal;flex-direction:column;display:flex;margin-left:12px}.bottom-sheet-layout-header{-webkit-box-pack:justify;justify-content:space-between;display:flex;margin-top:8px}.bottom-sheet-drag-line{background:#0f0f0f;opacity:.15;border-radius:4px;height:4px;margin:0 auto;width:40px;margin-top:8px}.bottom-sheet-layout-header-wrapper{overflow:hidden;-webkit-box-flex:0;flex:none;border-bottom:1px solid #fff}.bottom-sheet-layout{border-radius:12px;background-color:#fff;display:block;overflow:hidden;position:fixed;margin:0 8px 24px;bottom:0;left:0;right:0;z-index:2}.overlay{position:fixed;top:0;bottom:0;left:0;right:0;z-index:1;cursor:pointer;background-color:rgba(0,0,0,.6)}.wrapper{position:fixed;z-index:5}`;
            this.wrapper = wrapper;
            this.shadowRoot.append(style, wrapper);
        }

        click(evt) {
            this.dispatchEvent(new CustomEvent('submit', {
                detail: {
                    id: evt.currentTarget.dataset.id
                }
            }));
        }

        close() {
            this.style.display = 'none';
        }

        connectedCallback() {
            this.wrapper.innerHTML = `<div bind @click="close" class="overlay">
    </div>
    <div class="bottom-sheet-layout">
      <div class="bottom-sheet-layout-header-wrapper">
        <div class="bottom-sheet-drag-line">
        </div>
        <div class="bottom-sheet-layout-header">
          <div class="bottom-sheet-layout-header-title-wrapper">
          </div>
          <div bind="contentWrapper" class="bottom-sheet-layout-content-wrapper">
            <div bind @click="insertLink" class="menu-item">
              插入链接
            </div>
            <div bind @click="openLink" class="menu-item">
              打开链接
            </div>
            <div bind @click="onPreview" class="menu-item">
              预览
            </div>
            <div bind @click="onTranslateChinese" class="menu-item">
              翻译中文
            </div>
            <div bind @click="onTranslateEnglish" class="menu-item">
              翻译英文
            </div>
            <div bind @click="sortLines" class="menu-item">
              排序
            </div>
            <div bind @click="uploadHanlder" class="menu-item">
              上传图片
            </div>
            <div bind @click="onEval" class="menu-item">
              执行表达式
            </div>
            <div bind @click="onDeleteString" class="menu-item">
              删除
            </div>
            <div bind @click="onFormatCodeBlock" class="menu-item">
              格式化代码
            </div>
            <div bind @click="onCopyLine" class="menu-item">
              复制行
            </div>
            <div bind @click="onCutLine" class="menu-item">
              剪切行
            </div>
            <div bind @click="pasteCode" class="menu-item">
              粘贴代码
            </div>
            <div bind @click="onInsertComment" class="menu-item">
              评论
            </div>
            <div bind @click="formatIndentIncrease" class="menu-item">
              缩进
            </div>
            <div bind @click="formatIndentDecrease" class="menu-item">
              缩退
            </div>
            <div bind @click="saveSnippets" class="menu-item">
              保存代码段
            </div>       
            <div bind @click="onShowDialog" class="menu-item">
              替换模式
            </div>
            <div bind @click="close" class="menu-item">
              取消
            </div>
          </div>
        </div>
      </div>
    </div>`;


            this.wrapper.querySelectorAll('[bind]').forEach(element => {
                if (element.getAttribute('bind')) {
                    this[element.getAttribute('bind')] = element;
                }
                [...element.attributes].filter(attr => attr.nodeName.startsWith('@')).forEach(attr => {
                    if (!attr.value) return;
                    element.addEventListener(attr.nodeName.slice(1), evt => {
                        this.style.display = 'none';
                        this[attr.value](evt);
                    });
                });
            });
            const patterns = this.getPatterns();
            if (patterns)
                this.loadPatterns(patterns);
        }

        async insertLink() {
            const strings = await readText();
            let name = '';
            try {
                name = await (await fetch(`/title?path=${encodeURIComponent(strings)}`)).text()
            } catch (e) {
            }
            textarea.setRangeText(
                `[${name.trim()}](${strings})`,
                textarea.selectionStart,
                textarea.selectionEnd,
                'end'
            )
        }

        async onEval() {
            const p = findBlock(textarea);
            const s = textarea.value.substring(p[0], p[1]);
            textarea.setRangeText(
                ` = ${eval(s)}`,
                p[1],
                p[1],
                'end'
            )
        }

        onPreview() {
            const searchParams = new URL(window.location).searchParams;
            if (searchParams.has("path")) {
                const path = searchParams.get("path");
                window.open(`/markdown?path=${path}`, '_blank')
            } else {
                window.open(`/markdown?id=${searchParams.get("id")}`, '_blank')
            }
        }

        async onTranslateChinese() {
            let array1 = getLine();
            let strings = await translate(array1[0], 'zh');
            if (this.patterns) {
                for (let index = 0; index < this.patterns.length; index++) {
                    const element = this.patterns[index];
                    strings = strings.replaceAll(new RegExp(
                        element[0], 'g'
                    ), element[1])
                }
            }
            textarea.setRangeText(`\n\n${(strings)}`, array1[2], array1[2], 'end');
        }

        async onTranslateEnglish() {
            let array1 = getLine();
            textarea.setRangeText(`\n\n${await translate(array1[0], 'en')}
          `, array1[2], array1[2], 'end');
        }

        openLink() {
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

        sortLines() {
            const points = findBlock(textarea);
            const lines = textarea.value.substring(points[0], points[1]).split('\n')
                .sort((x, y) => {
                    let v1 = /\d{4}\)/.exec(x);
                    let v2 = /\d{4}\)/.exec(y)
                    if (v1 && v2)
                        return v1[0].localeCompare(v2[0])
                    return x.localeCompare(y);
                });
            textarea.setRangeText(`\n\n${lines.join('\n')}`, points[0], points[1], 'end');
        }

        async translate(value, to) {
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

        uploadHanlder(editor) {
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

        onDeleteString() {
            const start = textarea.selectionStart;
            let end = textarea.selectionEnd;
            while (end + 1 < textarea.value.length && textarea.value[end] !== ']') {
                end++;
            }
            textarea.setRangeText('', start, end, 'end');
            console.log(start);
        }

        async pasteCode() {
            let strings;
            if (typeof NativeAndroid !== 'undefined') {
                strings = NativeAndroid.readText()
            } else {
                strings = await navigator.clipboard.readText()
            }
            textarea.setRangeText(`
\`\`\`
${strings}
\`\`\`
  `, textarea.selectionStart, textarea.selectionEnd, 'end');
        }

        onInsertComment() {
            let start = textarea.selectionStart;
            const strings = textarea.value;
            if (strings[start] === '\n' && start - 1 > 0) {
                start--;
            }
            while (start > 0 && strings[start - 1] !== '\n') {
                start--;
            }
            let end = textarea.selectionEnd;
            while (end + 1 < strings.length && strings[end] !== '\n') {
                end++;
            }
            if (end < textarea.value.length) {
                let nexEnd = end + 1;
                while (nexEnd + 1 < strings.length && /\s+/.test(strings[nexEnd])) {
                    nexEnd++;
                }
                textarea.setRangeText(`${' '.repeat(nexEnd - end - 1)}// `, start, start, 'end')
                return
            }
            textarea.setRangeText(`// `, this.textarea.selectionStart, this.textarea.selectionEnd, 'end')
        }

        formatIndentIncrease() {
            if (textarea.selectionStart === textarea.selectionEnd) {
                const line = getLine();
                textarea.setRangeText(
                    ' '.repeat(2) + line[0],
                    line[1], line[2], 'end'
                )
            } else {
                const string = getSelectedString(textarea);
                textarea.setRangeText(string.split('\n')
                    // .filter(x => x.trim())
                    .map(x => '  ' + x).join('\n'), textarea.selectionStart, textarea.selectionEnd, 'end');
            }
        }

        formatIndentDecrease() {
            if (textarea.selectionStart === textarea.selectionEnd) {
                const line = getLine();
                if (line[0].startsWith("  "))
                    textarea.setRangeText(
                        line[0].slice(2),
                        line[1], line[2], 'end'
                    )
            } else {
                const string = getSelectedString(textarea);
                textarea.setRangeText(string.split('\n')
                    // .filter(x => x.trim())
                    .map(x => x.startsWith("  ") ? x.slice(2) : x).join('\n'), textarea.selectionStart, textarea.selectionEnd, 'end');
            }
        }

        onFormatCodeBlock() {
            const indexs = findCodeBlock(textarea);
            if (textarea.selectionStart !== textarea.selectionEnd) {
                const selected = textarea.value.substring(textarea.selectionStart, textarea.selectionEnd);
                let s = textarea.value.substring(indexs[0], indexs[1])
                    .split('\n').map(x => {
                        if (x.startsWith(selected)) {
                            x = x.substring(selected.length);
                        }
                        return x;
                    }).join('\n');
                textarea.setRangeText(s, indexs[0], indexs[1], 'end');
            }
        }

        onCopyLine() {
            copyLine(textarea);
        }

        onCutLine() {
            const p = getLine(textarea);
            writeText(p[0]);
            textarea.setRangeText(``,
                p[1], p[2], 'end')
        }

        onDeleteLine() {
            const p = getLine(textarea);
            let start = p[1];
            let end = p[2];
            const re = new RegExp("[\\s\t]");
            while (start > -1 && re.test(textarea.value[start - 1]))
                start--
            while (end + 1 < textarea.value.length && re.test(textarea.value[end]))
                end++;
            textarea.setRangeText(`\n\n`,
                start, end, 'end');
        }

        onFormatCode() {
            let start = textarea.selectionStart;
            let end = textarea.selectionEnd;
            // \(\)\[\].!/\?%-
            const re = new RegExp("[a-zA-Z0-9_.+%'#*=()!?|^&\\[\\]{}\" -]");
            while (start > -1 && re.test(textarea.value[start - 1])) {
                start--;
            }
            while (end + 1 < textarea.value.length && re.test(textarea.value[end])) {
                end++;
            }
            const value = textarea.value.substring(start, end);
            textarea.setRangeText(` \`${value.trim()}\` `, start, end, 'end');
            writeText('`')
        }

        onFormatBold() {
            let start = textarea.selectionStart;
            let end = textarea.selectionEnd;
            // \(\)\[\].!/\?%-
            const re = new RegExp("[a-zA-Z0-9.+%'#*=()!?|^&\\[\\]{}\" -]");
            while (start > -1 && re.test(textarea.value[start - 1])) {
                start--;
            }
            while (end + 1 < textarea.value.length && re.test(textarea.value[end])) {
                end++;
            }
            const value = textarea.value.substring(start, end);
            textarea.setRangeText(` **${value.trim()}** `, start, end, 'end');
            writeText('`')
        }

        onFormatNumberList() {
            let p = getIndexLine(textarea);
            let p1 = p;
            while (true) {
                if (p1[1] <= 0) {
                    break;
                }
                let p2 = getIndexLine(textarea, p1[1] - 1);
                if (p2[0].trim()) {
                    let index = 1;
                    if (/(\d+). /.test(p2[0])) {
                        index = parseInt(/(\d+). /.exec(p2[0])[1]) + 1;
                        textarea.setRangeText(`${index}. ${p[0]}`,
                            p[1], p[2], 'end')
                        return;
                    } else {
                        textarea.setRangeText(`${index}. ${p[0]}`,
                            p[1], p[2], 'end')
                        return;
                    }
                }
                p1 = p2;

            }
            textarea.setRangeText(`1. ${p[0]}`,
                p[1], p[2], 'end')
        }

        onFormatList() {
            let p = getIndexLine(textarea);
            let p1 = p;
            while (true) {
                if (p1[1] <= 0) {
                    break;
                }
                let p2 = getIndexLine(textarea, p1[1] - 1);
                if (p2[0].trim()) {

                    if (/(\d+). /.test(p2[0])) {
                        textarea.setRangeText(`    - ${p[0]}`,
                            p[1], p[2], 'end')
                        return;
                    } else if (/ +- /.test(p2[0])) {
                        textarea.setRangeText(`${/( +)- /.exec(p2[0])[1]}- ${p[0]}`,
                            p[1], p[2], 'end')
                        return;
                    } else {
                        textarea.setRangeText(`- ${p[0]}`,
                            p[1], p[2], 'end')
                        return;
                    }
                }
                p1 = p2;

            }
            textarea.setRangeText(`- ${p[0]}`,
                p[1], p[2], 'end')
        }

        onFormatHead() {
            formatHead(textarea, 3)
        }

        onShowDialog() {
            const customDialog = document.createElement('custom-dialog');
            document.body.appendChild(customDialog);
            customDialog.title = ""
            window.customDialog = customDialog;
            customDialog.addEventListener('submit', evt => {
                this.setPatterns(customDialog.content);
                this.loadPatterns(customDialog.content)
            });
            const patterns = this.getPatterns();
            if (patterns) {
                customDialog.content = patterns;
            }
            //customDialog.style.display = 'none';
        }

        loadPatterns(patterns) {
            this.patterns = patterns.split('\n')
                .filter(x => x.trim())
                .map(x => x.trim().split('|'))
        }

        getPatterns() {
            let strings;
            if (typeof NativeAndroid !== 'undefined') {
                strings = NativeAndroid.getString("pattern")
            } else {
                strings = localStorage.getItem('pattern')
            }
            return strings;
        }

        setPatterns(patterns) {
            if (typeof NativeAndroid !== 'undefined') {
                NativeAndroid.setString("pattern", patterns)
            } else {
                localStorage.setItem('pattern', patterns)
            }

        }

        formatCodeBlock() {
            let p = getContinueBlock(textarea);
            textarea.setRangeText(`\`\`\`
${textarea.value.substring(p[0], p[1])}
\`\`\`
`,
                p[0], p[1], 'end')
        }

        showSnippets() {
            const favorites = document.createElement('custom-favorites');
            document.body.appendChild(favorites);
            favorites.data = Object.keys(this.readSnippets());
            favorites.addEventListener('submit', evt => {
                textarea.setRangeText(this.readSnippets()[evt.detail],
                    textarea.selectionStart, textarea.selectionEnd, 'end')
            })
            favorites.addEventListener('remove', evt => {
                this.removeSnippets(evt.detail)
            })
        }

        removeSnippets(path) {
            let paths = this.readSnippets();
            delete paths[path];
            localStorage.setItem('snippets', JSON.stringify(paths));
        }

        saveSnippets() {
            let paths = this.readSnippets();
            paths [`${substringBefore(textarea.value.trim(), '\n')}`] = substringAfter(textarea.value.trim(), '\n');

            localStorage.setItem('snippets', JSON.stringify(paths));
        }

        readSnippets() {
            const text = localStorage.getItem('snippets');
            return JSON.parse(text || '{}');
        }

    }

    customElements.define('custom-actions', CustomActions);

    /*
    ATTRIBUTE_NODE
  CDATA_SECTION_NODE
  COMMENT_NODE
  DOCUMENT_FRAGMENT_NODE
  DOCUMENT_NODE
  DOCUMENT_POSITION_CONTAINED_BY
  DOCUMENT_POSITION_CONTAINS
  DOCUMENT_POSITION_DISCONNECTED
  DOCUMENT_POSITION_FOLLOWING
  DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC
  DOCUMENT_POSITION_PRECEDING
  DOCUMENT_TYPE_NODE
  ELEMENT_NODE
  ENTITY_NODE
  ENTITY_REFERENCE_NODE
  NOTATION_NODE
  PROCESSING_INSTRUCTION_NODE
  TEXT_NODE
  accessKey
  addEventListener
  after
  animate
  append
  appendChild
  ariaAtomic
  ariaAutoComplete
  ariaBusy
  ariaChecked
  ariaColCount
  ariaColIndex
  ariaColSpan
  ariaCurrent
  ariaDescription
  ariaDisabled
  ariaExpanded
  ariaHasPopup
  ariaHidden
  ariaKeyShortcuts
  ariaLabel
  ariaLevel
  ariaLive
  ariaModal
  ariaMultiLine
  ariaMultiSelectable
  ariaOrientation
  ariaPlaceholder
  ariaPosInSet
  ariaPressed
  ariaReadOnly
  ariaRelevant
  ariaRequired
  ariaRoleDescription
  ariaRowCount
  ariaRowIndex
  ariaRowSpan
  ariaSelected
  ariaSetSize
  ariaSort
  ariaValueMax
  ariaValueMin
  ariaValueNow
  ariaValueText
  assignedSlot
  attachInternals
  attachShadow
  attributeStyleMap
  attributes
  autocapitalize
  autofocus
  baseURI
  before
  blur
  childElementCount
  childNodes
  children
  classList
  className
  click
  clientHeight
  clientLeft
  clientTop
  clientWidth
  cloneNode
  closest
  compareDocumentPosition
  computedStyleMap
  contains
  contentEditable
  dataset
  dir
  dispatchEvent
  draggable
  elementTiming
  enterKeyHint
  firstChild
  firstElementChild
  focus
  getAnimations
  getAttribute
  getAttributeNS
  getAttributeNames
  getAttributeNode
  getAttributeNodeNS
  getBoundingClientRect
  getClientRects
  getElementsByClassName
  getElementsByTagName
  getElementsByTagNameNS
  getInnerHTML
  getRootNode
  hasAttribute
  hasAttributeNS
  hasAttributes
  hasChildNodes
  hasPointerCapture
  hidden
  id
  innerHTML
  innerText
  inputMode
  insertAdjacentElement
  insertAdjacentHTML
  insertAdjacentText
  insertBefore
  isConnected
  isContentEditable
  isDefaultNamespace
  isEqualNode
  isSameNode
  lang
  lastChild
  lastElementChild
  localName
  lookupNamespaceURI
  lookupPrefix
  matches
  namespaceURI
  nextElementSibling
  nextSibling
  nodeName
  nodeType
  nodeValue
  nonce
  normalize
  offsetHeight
  offsetLeft
  offsetParent
  offsetTop
  offsetWidth
  onabort
  onanimationend
  onanimationiteration
  onanimationstart
  onauxclick
  onbeforecopy
  onbeforecut
  onbeforepaste
  onbeforexrselect
  onblur
  oncancel
  oncanplay
  oncanplaythrough
  onchange
  onclick
  onclose
  oncontextmenu
  oncopy
  oncuechange
  oncut
  ondblclick
  ondrag
  ondragend
  ondragenter
  ondragleave
  ondragover
  ondragstart
  ondrop
  ondurationchange
  onemptied
  onended
  onerror
  onfocus
  onformdata
  onfullscreenchange
  onfullscreenerror
  ongotpointercapture
  oninput
  oninvalid
  onkeydown
  onkeypress
  onkeyup
  onload
  onloadeddata
  onloadedmetadata
  onloadstart
  onlostpointercapture
  onmousedown
  onmouseenter
  onmouseleave
  onmousemove
  onmouseout
  onmouseover
  onmouseup
  onmousewheel
  onpaste
  onpause
  onplay
  onplaying
  onpointercancel
  onpointerdown
  onpointerenter
  onpointerleave
  onpointermove
  onpointerout
  onpointerover
  onpointerrawupdate
  onpointerup
  onprogress
  onratechange
  onreset
  onresize
  onscroll
  onsearch
  onseeked
  onseeking
  onselect
  onselectionchange
  onselectstart
  onstalled
  onsubmit
  onsuspend
  ontimeupdate
  ontoggle
  ontransitioncancel
  ontransitionend
  ontransitionrun
  ontransitionstart
  onvolumechange
  onwaiting
  onwebkitanimationend
  onwebkitanimationiteration
  onwebkitanimationstart
  onwebkitfullscreenchange
  onwebkitfullscreenerror
  onwebkittransitionend
  onwheel
  outerHTML
  outerText
  ownerDocument
  parentElement
  parentNode
  part
  prefix
  prepend
  previousElementSibling
  previousSibling
  querySelector
  querySelectorAll
  releasePointerCapture
  remove
  removeAttribute
  removeAttributeNS
  removeAttributeNode
  removeChild
  removeEventListener
  replaceChild
  replaceChildren
  replaceWith
  requestFullscreen
  requestPointerLock
  scroll
  scrollBy
  scrollHeight
  scrollIntoView
  scrollIntoViewIfNeeded
  scrollLeft
  scrollTo
  scrollTop
  scrollWidth
  setAttribute
  setAttributeNS
  setAttributeNode
  setAttributeNodeNS
  setPointerCapture
  shadowRoot
  slot
  spellcheck
  style
  tabIndex
  tagName
  textContent
  title
  toggleAttribute
  translate
  virtualKeyboardPolicy
  webkitMatchesSelector
  webkitRequestFullScreen
  webkitRequestFullscreen
     */
})();