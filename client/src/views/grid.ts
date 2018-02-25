import * as PIXI from "pixi.js";


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

export function drawGrid (pixiApp: PIXI.Application, grid: ZoneType[][], state) {
    // create default size of grid
    for (let i = 0; i < grid.length; i ++) {
        for (let j = 0; j < grid[i].length; j ++) {
            drawRect(j, i, zoneColors[grid[i][j]], null);
        }
    }

    function onclick (rect, i, j) {
        return function() {
            drawRect(i, j, zoneColors[ZoneType[state.activeBuildType]], rect);
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
        if (color === zoneColors[ZoneType.NONE]) {
            rectangle.lineStyle(2, whiteColor);
        }
        rectangle.drawRect(0, 0, rectSize, rectSize);
        rectangle.x = i * (rectSize + rectPadding);
        rectangle.y = j * (rectSize + rectPadding);
        rectangle.interactive = true;
        rectangle.buttonMode = true;
        rectangle.on("pointerdown", onclick(rectangle, i, j));
        pixiApp.stage.addChild(rectangle);
    }
}
