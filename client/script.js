window.getPristineWebSocket = () => {
    const iframe = document.createElement("iframe");
    document.body.appendChild(iframe);
    const wsConstructor = iframe.contentWindow.WebSocket;
    document.body.removeChild(iframe);
    return wsConstructor;
};

// adjust canvas if window resizes
const canvas = document.getElementById("ctx");
window.addEventListener("resize", (e) => {
    canvas.width = window.innerWidth * window.devicePixelRatio;
    canvas.height = window.innerHeight * window.devicePixelRatio;

    canvas.style.width = `${window.innerWidth}px`;
    canvas.style.height = `${window.innerHeight}px`;
});

window.addEventListener("DOMContentLoaded", (e) => {
    canvas.width = window.innerWidth * window.devicePixelRatio;
    canvas.height = window.innerHeight * window.devicePixelRatio;
    canvas.style.width = `${window.innerWidth}px`;
    canvas.style.height = `${window.innerHeight}px`;
});

window.addEventListener("wheel", (e) => {
    //e.preventDefault();

    if(e.deltaY < 0) {
        window.innerHeight *= 0.01;
        window.innerWidth *= 0.01;

    } else {
        window.innerHeight *= 1.01;
        window.innerWidth *= 1.01;

    }

    console.log(e.deltaY);
    //window.innerHeight *= e.deltaY;
})