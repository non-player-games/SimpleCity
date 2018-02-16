// loading pixi
require("pixi.js");

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

for (var i = 0; i < ROW; i ++) {
    for (var j = 0; j < COLUMN; j ++) {
        const rectangle = new PIXI.Graphics();
        rectangle.beginFill(0xFF3333);
        rectangle.drawRect(0, 0, 40, 40);
        rectangle.x = i * 50;
        rectangle.y = j * 50;
        pixiApp.stage.addChild(rectangle);
    }
}

