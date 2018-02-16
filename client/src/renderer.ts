require("pixi.js");

let type = "WebGL";
if (!PIXI.utils.isWebGLSupported()) {
    type = "canvas";
}

PIXI.utils.sayHello(type);

const pixiApp = new PIXI.Application({
    width: 512,
    height: 512,
    antialias: true
});

document.body.appendChild(pixiApp.view);

pixiApp.renderer.backgroundColor = 0x222222;
