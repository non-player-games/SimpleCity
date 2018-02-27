import * as PIXI from "pixi.js";
import xs, {Stream} from "xstream";
import {adapt} from "@cycle/run/lib/adapt";
import fromEvent from "xstream/extra/fromEvent";
import * as EventEmitter from "events";

const width: number = 512;
const height: number = 512;
const rectSize: number = 32;
const rectPadding: number = 8;

// light grey background
const backgroundColor: number = 0x333333;
const whiteColor: number = 0xeeeeee;
const zoneColors: number[] = [
    0x222222,
    0x00ff7f,
    0x3399ff,
    0xffff66
];

// TODO: move ZoneType into model definition
enum ZoneType {
    NONE,
    RESIDENTIAL,
    COMMERCIAL,
    INDUSTRIAL
}
class EventProducer extends EventEmitter {};
const clickEventName: string = 'click';

export function makePixiDriver(): any {
    let instance: PIXI.Application; // lazy initialize chart on first stream event
    let el: Element;
    const rectGrid: any[][] = [[]];
    const producer = new EventProducer();
    const sink = {
        events: adapt(fromEvent(producer, clickEventName))
    };

    function init(element: Element): void {
        el = element;
    }

    function createApplication(grid: number[][]): void {
        instance = new PIXI.Application({
            width,
            height,
        });
        if (el) {
            el.appendChild(instance.view);
        } else {
            console.error('PixiDriver: Cannot find conatiner element');
            return;
        }
        instance.renderer.backgroundColor = backgroundColor;
        for (let i = 0; i < grid.length; i++) {
            rectGrid[i] = [];
            for (let j = 0; j < grid[i].length; j++) {
                rectGrid[i][j] = new PIXI.Graphics();
                instance.stage.addChild(rectGrid[i][j]);
                rectGrid[i][j].on('pointerdown', () => {
                    producer.emit(clickEventName, {i, j});
                });
                drawRect(i, j, grid[i][j]);
            }
        }
    }

    function drawRect(i: number, j: number, t: number): void {
        const color = zoneColors[t];
        const rectangle = rectGrid[i][j];
        // clear out the old style before redraw
        rectangle.clear();
        rectangle.beginFill(color);
        if (color === zoneColors[ZoneType.NONE]) {
            rectangle.lineStyle(2, whiteColor);
        }
        rectangle.drawRect(0, 0, rectSize, rectSize);
        rectangle.x = j * (rectSize + rectPadding);
        rectangle.y = i * (rectSize + rectPadding);
        rectangle.interactive = true;
        rectangle.buttonMode = true;
    }

    function updateChart(data: number[][]): void {
        if (data) {
            data.forEach((row, i) => {
                row.forEach((t, j) => {
                    drawRect(i, j, t);
                });
            })
        }
    }

    return function chartDriver(sink$: any): any {
        sink$.take(1).addListener({ next: init });
        sink$.drop(1).take(1).addListener({ next: createApplication });
        sink$.drop(2).addListener({ next: updateChart });

        return sink;
    }
}
