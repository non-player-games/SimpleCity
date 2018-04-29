import * as PIXI from "pixi.js";
import xs, { MemoryStream, Stream } from "xstream";
import { adapt } from "@cycle/run/lib/adapt";
import fromEvent from "xstream/extra/fromEvent";
import * as EventEmitter from "events";

import { ZoneType } from "../models/Zone";

export interface State {
    zones: number[][];
    population: number[][];
}
export interface Sink {
    events: Stream<Event>;
}
export type Input = string | Element | Document | HTMLBodyElement | State;

const width: number = 640;
const height: number = 640;
const rectSize: number = 32;
const rectPadding: number = 8;

// light grey background
const backgroundColor: number = 0x333333;
const whiteColor: number = 0xeeeeee;
const zoneColors: number[] = [
    0x222222,
    0x999999,
    0x00ff7f,
    0x3399ff,
    0xffff66
];

class EventProducer extends EventEmitter {};
const clickEventName: string = "click";

export function makePixiDriver(): (m: MemoryStream<Input>) => Sink {
    let instance: PIXI.Application; // lazy initialize chart on first stream event
    let el: Element;
    const rectGrid: any[][] = [[]];
    const populationGrid: any[][] = [[]];
    const producer = new EventProducer();
    const sink = {
        events: adapt(fromEvent(producer, clickEventName))
    };

    function init(element: Element): void {
        el = element;
    }

    function createApplication(state: State): void {
        instance = new PIXI.Application({
            width,
            height,
        });
        if (el) {
            el.appendChild(instance.view);
        } else {
            console.error("PixiDriver: Cannot find container element");
            return;
        }
        // initial rendering with proper objects
        instance.renderer.backgroundColor = backgroundColor;
        for (let i = 0; i < state.zones.length; i++) {
            rectGrid[i] = [];
            populationGrid[i] = [];
            for (let j = 0; j < state.zones[i].length; j++) {
                rectGrid[i][j] = new PIXI.Graphics();
                populationGrid[i][j] = new PIXI.Text("0", { fontSize: 14, fill: 0xcccccc, align: 'center'});

                instance.stage.addChild(rectGrid[i][j]);
                instance.stage.addChild(populationGrid[i][j]);
                rectGrid[i][j].on("pointerdown", () => {
                    producer.emit(clickEventName, {i, j});
                });
                drawPopulationText(i, j, state.population[i][j]);
                drawRect(i, j, state.zones[i][j], state.zones);
            }
        }
    }

    function drawRect(i: number, j: number, t: number, zonesState: number[][]): void {
        // avoid redraw when the zone grid hasn't changed
        if (
            getGridValueSafe(rectGrid, i, j).zoneType === t &&
            (
                getGridValueSafe(rectGrid, i - 1, j) !== null &&
                getGridValueSafe(rectGrid, i - 1, j).zoneType === getGridValueSafe(zonesState, i - 1, j)
            ) &&
            (
                getGridValueSafe(rectGrid, i, j - 1) !== null &&
                getGridValueSafe(rectGrid, i, j - 1).zoneType === getGridValueSafe(zonesState, i, j - 1)
            )
        ) {
            return;
        }
        const color = zoneColors[t];
        const rectangle = rectGrid[i][j];
        // if the above or left zone is the same as current one, don't draw padding
        // in between to "merge zones"
        const isLeftTheSame = getGridValueSafe(zonesState, i, j - 1) === getGridValueSafe(zonesState, i, j);
        const isAboveTheSame = getGridValueSafe(zonesState, i - 1, j) === getGridValueSafe(zonesState, i, j);
        const x = j * (rectSize + rectPadding);
        const y = i * (rectSize + rectPadding);
        const renderedWidth = isLeftTheSame ?
            rectSize + rectPadding : rectSize;
        const renderedHeight = isAboveTheSame ?
            rectSize + rectPadding : rectSize;
        // clear out the old style before redraw or other wise will not "update"
        rectangle.clear();
        rectangle.beginFill(color);
        if (color === zoneColors[ZoneType.NONE]) {
            rectangle.lineStyle(2, whiteColor);
        }
        rectangle.drawRect(x, y, renderedWidth, renderedHeight);
        rectangle.interactive = true;
        rectangle.buttonMode = true;
        rectGrid[i][j].zoneType = t;
    }
    function drawPopulationText(i: number, j: number, p: number): void {
        // avoid redraw if it hasn't changed
        if (populationGrid[i][j] === p) {
            return;
        }
        const text = populationGrid[i][j];
        text.text = (rectGrid[i][j].zoneType !== 0 && rectGrid[i][j].zoneType !== 1) ? `${p}` : "";
        text.x = j * (rectSize + rectPadding) + rectPadding * 2;
        text.y = i * (rectSize + rectPadding) + rectPadding * 2;
    }

    // getGridValueSafe returns the grid value if it exist or return null
    function getGridValueSafe(grid: any[][], i: number, j: number): any {
        if (i >= 0 && i < grid.length && j >= 0 && j < grid[i].length) {
            return grid[i][j];
        }
        return null;
    }

    function updateChart(data: State): void {
        if (!data) {
            return;
        }
        data.zones.forEach((row, i) => {
            row.forEach((t, j) => {
                drawRect(i, j, t, [[]]);
            });
        });
        data.population.forEach((row, i) => {
            row.forEach((t, j) => {
                drawPopulationText(i, j, t);
            });
        });
    }

    return function pixiDriver(sink$: Stream<Input>): Sink {
        // first input as dom element
        sink$.take(1).addListener({ next: init });
        // second input as the initial grid
        sink$.drop(1).take(1).addListener({ next: createApplication });
        // starting as third and onward to take grid data and update render
        sink$.drop(2).addListener({ next: updateChart });

        return sink;
    }
}
