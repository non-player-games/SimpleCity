import xs, { MemoryStream, Stream } from "xstream";
import concat from "xstream/extra/concat";
import { VNode, DOMSource } from "@cycle/dom";
import { StateSource } from "cycle-onionify";

import { BaseSources, BaseSinks } from "../interfaces";
import { Input as PixiInput } from "../drivers/pixi"
import { ZoneType } from "../models/Zone";

// Types
export interface Sources extends BaseSources {
    onion: StateSource<State>;
    pixi: any; // TODO define the source from driver
    ipc: any;
}
export interface Sinks extends BaseSinks {
    onion?: Stream<Reducer>;
    pixi?: Stream<PixiInput>;
    ipc?: Stream<Action>;
}
interface Action {
    type: string;
    payload?: string;
}
interface Intent {
    actions: Stream<Reducer>,
    requests: Stream<Action>
}

const mockGrid: number[][] = [
    [1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 0, 2],
    [0, 1, 1, 0, 2]
];

// State
enum GameState {
    STOP,
    START
}
export interface State {
    activeBuild: ZoneType;
    grid: number[][];
    gameState: GameState
}
export const defaultState: State = {
    activeBuild: ZoneType.NONE,
    grid: mockGrid,
    gameState: GameState.STOP
};
export type Reducer = (prev: State) => State | undefined;

export function Home({ DOM, onion, pixi, ipc }: Sources): Sinks {
    const intent: Intent = intentFn(DOM, pixi, ipc);
    const vdom$: Stream<VNode> = view(onion.state$);

    const gridDom$: MemoryStream<PixiInput> = DOM.select("#grid").element().take(1);
    const init$ = xs.of(mockGrid);
    const grid$ = onion.state$.map(state => state.grid);
    // debug usage
    ipc.events.subscribe({
        next: console.log
    });

    return {
        DOM: vdom$,
        onion: intent.actions,
        pixi: concat(gridDom$, init$, grid$),
        ipc: intent.requests
    };
}

function intentFn(DOM: DOMSource, pixi: any, ipc: any): Intent {
    const init$ = xs.of<Reducer>(
        prevState => (prevState === undefined ? defaultState : prevState)
    );
    const changeActive$: Stream<Reducer> = DOM.select(".color-circle")
        .events("click")
        .map((evt:any): Reducer => {
            const t: keyof typeof ZoneType = evt.target.dataset.type;
            return (state) => {
                return {
                    ...state,
                    activeBuild: ZoneType[t]
                };
            };
        });
    const startEvent$: Stream<Action> = DOM.select(".start-button")
        .events("click")
        .mapTo<Action>({type: "startGame"});

    const startEventReducer$: Stream<Reducer> = DOM.select(".start-button")
        .events("click")
        .mapTo<Reducer>((state) => {
            return {
                ...state,
                gameState: GameState.START
            };
        });

    const build$: Stream<Reducer> = pixi.events
        .map((data:any): Reducer => {
            return (state) => {
                return {
                    ...state,
                    grid: updateGrid(state.grid, data.i, data.j, state.activeBuild)
                };
            }
        });

    return {
        actions: concat(init$, xs.merge(build$, changeActive$, startEventReducer$)),
        requests: startEvent$
    };
}

function updateGrid(grid: number[][], i: number, j: number, buildType: ZoneType): number[][] {
    const copy = deepCopy(grid);
    copy[i][j] = buildType;
    return copy;
}

function view(state$: Stream<State>): Stream<VNode> {
    return state$.map(state => (
        <div className="fill-parent">
            <div className="info floating-panel">
            $: 100
            </div>
            <div id="grid" className="fill-paent"></div>
            <div className="actions floating-panel">
                {Object.keys(ZoneType)
                    .filter((k: keyof typeof ZoneType) => !isNaN(Number(ZoneType[k])))
                    .map((z: keyof typeof ZoneType) => {
                    return <button
                        className={getColorCircleClass(state.activeBuild, z)}
                        data-type={z}>
                    </button>
                })}
            </div>
            <div className="options floating-panel">
                <button className="start-button">{(state.gameState === GameState.START) ? 'Stop' : 'Start'}</button>
            </div>
        </div>
    ));
}

// Helper functions
function getColorCircleClass(active: ZoneType, t: keyof typeof ZoneType): string {
    const className = `${t.toLowerCase()} color-circle`;
    return active === ZoneType[t] ? `active ` + className : className;
}
function deepCopy<T> (obj: T): T {
    return JSON.parse(JSON.stringify(obj));
}
