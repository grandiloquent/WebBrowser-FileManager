function appendSubtitle(video) {
    //document.querySelectorAll('track').forEach(x => x.remove())
    const track = document.createElement('track');
    var tracks = video.textTracks;
    var numTracks = tracks.length;
    for (var i = numTracks - 1; i >= 0; i--)
        video.textTracks[i].mode = "disabled";
    track.src = substringBeforeLast(video.src, ".") + ".srt";
    track.default = true;
    video.appendChild(track);
}

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

function calculateLoadedPercent(video) {
    if (!video.buffered.length) {
        return '0';
    }
    return (video.buffered.end(0) / video.duration) * 100 + '%';
}

function calculateProgressPercent(video) {
    return ((video.currentTime / video.duration) * 100).toFixed(2) + '%';
}

function formatDuration(ms) {
    if (isNaN(ms)) return '0:00';
    if (ms < 0) ms = -ms;
    const time = {
        hour: Math.floor(ms / 3600) % 24,
        minute: Math.floor(ms / 60) % 60,
        second: Math.floor(ms) % 60,
    };
    return Object.entries(time)
        .filter((val, index) => index || val[1])
        .map(val => (val[1] + '').padStart(2, '0'))
        .join(':');
}

function getCurrentVideoFileName() {
    let s = substringAfterLast(decodeURIComponent(video.src), '/');
    s = substringAfterLast(s, '\\')
    return substringBefore(s, "&");
}

function getIndexOfCurrentPlayback() {
    const name = getCurrentVideoFileName();
    return items.indexOf(items.filter(x => x.name === name)[0]);
}

async function loadData() {
    if (!items) {
        const path = substringBeforeLast(new URL(document.URL).searchParams.get('path'), '/');
        const res = await fetch(`/api/files?path=${encodeURIComponent(path)}&idDir=1`);
        items = await res.json();
        items = items.filter(x => {
            return !x.isDir && x.name.endsWith('.mp4');
        })
    }
}

function onBottom(evt) {
    evt.stopPropagation();
    if (evt.clientX > left || evt.clientX <= width + left) {
        let precent = (evt.clientX - left) / width;
        precent = Math.max(precent, 0);
        precent = Math.min(precent, 1);
        video.currentTime = video.duration * precent;
    }
}

function onDownload(evt) {
    evt.stopPropagation();
    renderData();
}

function onDurationChange() {
    console.log(window.innerWidth, window.innerHeight)
    if (window.innerWidth < window.innerHeight) {
        const ratio = video.videoWidth / window.innerWidth;
        layout.style.height = `${video.videoHeight / ratio}px`;
    } else {
        const ratio = video.videoHeight / window.innerHeight;
        layout.style.height = `${video.videoHeight / ratio}px`;
    }
    progressBarPlayed.style.width = calculateProgressPercent(video);
    timeSecond.textContent = formatDuration(video.duration);
}

function onEnded() {
    playIndexedVideo(true)
}

function onLayout(evt) {
    middle.style.display = 'flex';
    bottom.style.display = 'flex';
    timer && clearTimeout(timer);
    timer = setTimeout(() => {
        middle.style.display = 'none';
        bottom.style.display = 'none';
    }, 5000)
}

function onNext(evt) {
    evt.stopPropagation();
    playIndexedVideo(true)
}

function onPause() {
    buttonPlay.querySelector('path').setAttribute('d', 'M6,4l12,8L6,20V4z');
}

function onPlay(evt) {
    evt.stopPropagation();
    buttonPlay.querySelector('path').setAttribute('d', 'M9,19H7V5H9ZM17,5H15V19h2Z');
}

function onPlayButton(evt) {
    evt.stopPropagation();
    if (video.paused) {
        video.play();
        timer && clearTimeout(timer);
        timer = setTimeout(() => {
            middle.style.display = 'none';
            bottom.style.display = 'none';
        }, 5000)
    } else {
        video.pause();
    }
}

function onPrevious(evt) {
    evt.stopPropagation();
    playIndexedVideo(false)
}

function onProgress() {
    progressBarLoaded.style.width = calculateLoadedPercent(video);
}

function onTimeupdate() {
    timeFirst.textContent = formatDuration(video.currentTime);
    const width = calculateProgressPercent(video);
    progressBarPlayed.style.width = width
    progressBarPlayhead.style.left = width
}

async function playIndexedVideo(next) {
    await loadData();
    let index = getIndexOfCurrentPlayback();
    if (next && index + 1 < items.length) {
        index++;
        playVideoAtSpecifiedIndex(index)
    }
    if (!next && index > 0) {
        index--
        playVideoAtSpecifiedIndex(index)
    }
}

async function playVideoAtSpecifiedIndex(index) {
    const v = items[index];
    video.src = `/api/files?path=${encodeURIComponent(v.parent + "\\" + v.name)}&isDir=0`;
    appendSubtitle(video);
    await video.play();
}

async function renderData() {
    await loadData();
    const customBottomSheet = document.createElement('custom-bottom-sheet');
    document.body.appendChild(customBottomSheet);
    items.forEach((x, j) => {
        customBottomSheet.appendItem(x.name, evt => {
            customBottomSheet.remove();
            playVideoAtSpecifiedIndex(j);
        });
    });
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

function toggleFullScreen() {
    if (!document.fullscreenElement) {
        document.documentElement.requestFullscreen();
        layout.style.position = "fixed";
        layout.style.left = '0';
        layout.style.top = '0';
        layout.style.bottom = '0';
        layout.style.right = '0';
        layout.style.height = 'auto'
    } else {
        if (document.exitFullscreen) {
            document.exitFullscreen();
            layout.style.position = "relative";
            const ratio = video.videoWidth / window.innerWidth;
            layout.style.height = `${video.videoHeight / ratio}px`;
        }
    }
}
function getPath() {
    return new URL(document.URL).searchParams.get('path');
}
function onFullscreen(evt) {
    evt.stopPropagation();
    toggleFullScreen();
}
/*
https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement
https://developer.mozilla.org/zh-CN/docs/Web/API/Fullscreen_API
*/