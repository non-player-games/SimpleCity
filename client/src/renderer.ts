import * as PIXI from "pixi.js";

const pixiApp = new PIXI.Application({
    width: 512,
    height: 512,
    antialias: true
});

// insert pixiApp to document
document.body.appendChild(pixiApp.view);

pixiApp.renderer.backgroundColor = 0x222222;

const ROW = 5;
const COLUMN = 5;

for (let i = 0; i < ROW; i ++) {
    for (let j = 0; j < COLUMN; j ++) {
        drawRect(i, j, 0xFF3333, null);
    }
}

function onclick (rect, i, j) {
    return function() {
        console.log('rect', rect);
        drawRect(i, j, 0x3333FF, rect);
    }
}

function drawRect(i, j, color, rect) {
    let rectangle;
    if (rect) {
        rectangle = rect;
        rect.clear();
    } else {
        rectangle = new PIXI.Graphics();
    }
    rectangle.beginFill(color);
    rectangle.drawRect(0, 0, 40, 40);
    rectangle.x = i * 50;
    rectangle.y = j * 50;
    rectangle.interactive = true;
    rectangle.buttonMode = true;
    rectangle.on('pointerdown', onclick(rectangle, i, j));
    pixiApp.stage.addChild(rectangle);
}
