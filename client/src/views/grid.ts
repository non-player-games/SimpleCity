import * as PIXI from "pixi.js";

const width: number = 512;
const height: number = 512;
const rectSize: number = 32;
const rectPadding: number = 8;

// light grey background
const backgroundColor: number = 0x222222;
const zoneColors: number[] = [
    0x00ff7f,
    0x3399ff,
    0xffff66
];

enum ZoneType {
    Residential,
    Commercial,
    Industrial
}

export function create (container: Element): PIXI.Application {
    const pixiApp = new PIXI.Application({
        width: width,
        height: height,
        antialias: true
    });
    container.appendChild(pixiApp.view);

    pixiApp.renderer.backgroundColor = backgroundColor;
    return pixiApp;
}

export function drawGrid (pixiApp: PIXI.Application, grid: ZoneType[][]) {
    // create default size of grid
    for (let i = 0; i < grid.length; i ++) {
        for (let j = 0; j < grid[i].length; j ++) {
            drawRect(j, i, zoneColors[grid[i][j]], null);
        }
    }

    function onclick (rect, i, j) {
        return function() {
            drawRect(i, j, 0x3333FF, rect);
        };
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
        rectangle.drawRect(0, 0, rectSize, rectSize);
        rectangle.x = i * (rectSize + rectPadding);
        rectangle.y = j * (rectSize + rectPadding);
        rectangle.interactive = true;
        rectangle.buttonMode = true;
        rectangle.on("pointerdown", onclick(rectangle, i, j));
        pixiApp.stage.addChild(rectangle);
    }
}
