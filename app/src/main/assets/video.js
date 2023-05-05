function adjustSize(video) {
    if (video.videoWidth > 0) {
        let ratio = Math.min(window.innerWidth / video.videoWidth, window.innerHeight / video.videoHeight);
        let height = video.videoHeight * ratio
        let width = video.videoWidth * ratio;
        video.style.width = `${width}px`;
        video.style.height = `${height}px`;
        video.style.left = `${(window.innerWidth - width) / 2}px`;
        video.style.top = `${(window.innerHeight - height) / 2}px`
    }
}
function durationchange(video) {
    return evt => {
        if (video.duration) {
            second.textContent = formatDuration(video.duration);
        }
        adjustSize(video);
    }
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
function getBaseAddress() {
    return window.location.host === "127.0.0.1:5500" ? "http://192.168.0.109:3000" : "";
}
async function getUrl(baseUri, url) {
    const res = await fetch(`${baseUri}/video/fetch?url=${encodeURIComponent(url)}`);
    return res.json();
}
function initializeSeek(video) {
    const width = video.getBoundingClientRect().width;
    const first = document.querySelector('.first');
    let right, start = video.currentTime, dif = 0, timer;
    video.addEventListener('touchstart', evt => {
        video.pause();
        right = (evt.touches[0].clientX >= width / 2);
        clearInterval(timer);
        timer = setInterval(() => {
            dif += 1 * (right ? 1 : -1);
            first.textContent = formatDuration(dif + start);
        }, 100);
    })
    video.addEventListener('touchend', evt => {
        clearInterval(timer);
        video.currentTime = dif + start;
        video.play();
    })
}
function progress(video, loaded) {
    return evt => {
        if (video.buffered.length) {
            loaded.style.width = `${video.buffered.end(0) / video.duration * 100}%`;
        }
    }
}
async function readText() {
    let strings;
    if (typeof NativeAndroid !== 'undefined') {
        strings = NativeAndroid.readText()
    } else {
        strings = await navigator.clipboard.readText()
    }
    return strings
}
function setSrc(video, src) {
    console.log(src);
    if (!video.canPlayType('application/vnd.apple.mpegurl') && Hls.isSupported()) {
        console.log("Hls");
        var hls = new Hls();
        hls.loadSource(src);
        hls.attachMedia(
            video
        );
    } else {
        video.src = src;
    }
}
function timeupdate(video, first) {
    return evt => {
        if (video.currentTime) {
            first.textContent = formatDuration(video.currentTime);
        }
    }
}
async function initialize() {
    const searchParams = new URL(window.location).searchParams;
    const url = searchParams.get("url");
    let videoInformation;
    try {
        videoInformation = await getUrl(getBaseAddress(), url);
    } catch (error) {
        console.log(error);
        return;
    }
    document.title = videoInformation.title;
    const video = document.querySelector('video');
    const first = document.getElementById('first');
    const second = document.getElementById('second');
    const play = document.querySelector('.play');
    const middle = document.getElementById('middle');
    const bottom = document.getElementById('bottom');
    const loaded = document.querySelector('.progress_bar_loaded');
    setSrc(video, videoInformation.file);
    initializeSeek(video);
    video.addEventListener('durationchange', durationchange(video, second));
    video.addEventListener('timeupdate', timeupdate(video, first));
    video.addEventListener('progress', progress(video, loaded));
    let timer;
    play.addEventListener('click', evt => {
        if (video.paused) {
            video.play();
            timer = startTimer(timer, middle, bottom);
        }
    })
}
initialize();

function startTimer(timer, middle, bottom) {
    clearTimeout(timer);
    timer = setTimeout(() => {
        middle.style.display = "none";
        bottom.style.display = "none";
    }, 10000);
    return timer;
}
