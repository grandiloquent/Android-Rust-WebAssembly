function adjustSize(video) {
    if (video.videoWidth > 0) {
        const w = Math.min(window.outerWidth, window.innerWidth);
        const h = Math.min(window.outerHeight, window.innerHeight);
        let ratio = Math.min(w / video.videoWidth, h / video.videoHeight);
        let height = video.videoHeight * ratio
        let width = video.videoWidth * ratio;
        video.style.width = `${width}px`;
        video.style.height = `${height}px`;
        video.style.left = `${(w - width) / 2}px`;
        video.style.top = `${(h - height) / 2}px`
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
function initializeSeek(video, first) {
    // const width = video.getBoundingClientRect().width;
    // let right, start, dif = 0, timer;
    // video.addEventListener('touchstart', evt => {
    //     video.pause();
    //     start = video.currentTime;
    //     right = (evt.touches[0].clientX >= width / 2);
    //     clearInterval(timer);
    //     timer = setInterval(() => {
    //         dif += 1 * (right ? 1 : -1);
    //         first.textContent = formatDuration(dif + start);
    //         startTimer();
    //     }, 100);
    // })
    // video.addEventListener('touchend', evt => {
    //     clearInterval(timer);
    //     console.log(dif + start)
    //     video.currentTime = dif + start;
    //     video.play();
    // })

    document.getElementById('back').addEventListener('click', evt => {
        startTimer();
        if (video.currentTime - 30 > 0)
            video.currentTime = video.currentTime - 30;
    })
    document.getElementById('forward').addEventListener('click', evt => {
        startTimer();
        if (video.currentTime + 30 <= video.duration)
            video.currentTime = video.currentTime + 30;
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
    if (!video.canPlayType('application/vnd.apple.mpegurl') && Hls.isSupported()) {
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
let timer;
const middle = document.getElementById('middle');
const bottom = document.getElementById('bottom');
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

    const loaded = document.querySelector('.progress_bar_loaded');
    setSrc(video, videoInformation.file);
    video.addEventListener('durationchange', durationchange(video, second));
    video.addEventListener('timeupdate', timeupdate(video, first));
    video.addEventListener('progress', progress(video, loaded));
    video.addEventListener('play', play());
    video.addEventListener('pause', pause());

    initializeSeek(video, first);
    initializeSeekDialog(video);
    document.getElementById('play').addEventListener('click', evt => {
        if (video.paused) {
            video.play();
            startTimer();
        } else {
            video.pause();
        }
    });
    video.addEventListener('click', evt => {
        middle.style.display = "flex";
        bottom.style.display = "flex";
        startTimer();
    });
}
function play() {
    return evt => {
        document.querySelector('#play path').setAttribute('d', 'M9,19H7V5H9ZM17,5H15V19h2Z');
    }
}
function pause() {
    return evt => {
        document.querySelector('#play path').setAttribute('d', ' M6,4l12,8L6,20V4z');
    }
}
initialize();

function startTimer() {
    if (timer)
        clearTimeout(timer);
    timer = setTimeout(() => {
        middle.style.display = "none";
        bottom.style.display = "none";
    }, 10000);
    return timer;
}

function initializeSeekDialog(video) {
    const dialogContainer = document.querySelector('.dialog-container');
    dialogContainer.querySelector('.dialog-overlay')
        .addEventListener('click', evt => {
            dialogContainer.style.display = 'none';
        });
    dialogContainer.querySelector('.dialog-buttons>div')
        .addEventListener('click', evt => {
            const values = /((\d+)h)?((\d+)m)?(\d+)s/.exec(dialogContainer.querySelector('input').value);
            let currentTime = 0;
            if (values[2]) {
                currentTime += parseInt(values[2]) * 3600;
            }
            if (values[4]) {
                currentTime += parseInt(values[4]) * 60;
            }
            if (values[5]) {
                currentTime += parseInt(values[5]);
            }
            video.currentTime = currentTime;
            dialogContainer.style.display = 'none';
        });

    document.querySelector('.playback_speed')
        .addEventListener('click', evt => {
            dialogContainer.style.display = 'flex';
        })
}
