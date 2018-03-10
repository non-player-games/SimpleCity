import xs, { MemoryStream, Stream } from "xstream";
import concat from "xstream/extra/concat";
import flattenConcurrently from 'xstream/extra/flattenConcurrently'
import { VNode, DOMSource } from "@cycle/dom";
import { TimeSource } from "@cycle/time";
import { StateSource } from "cycle-onionify";

import { BaseSources, BaseSinks } from "../interfaces";
import { Input as PixiInput } from "../drivers/pixi"
import { Sink as IPCSink } from "../drivers/ipc"
import { ZoneType } from "../models/Zone";

// Types
export interface Sources extends BaseSources {
    onion: StateSource<State>;
    pixi: any; // TODO define the source from driver
    ipc: IPCSink;
    time: TimeSource;
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

// State
enum GameState {
    STOP,
    START
}
interface RCINeed {
    residential: number;
    commercial: number;
    industrial: number;
}
export interface State {
    activeBuild: ZoneType;
    grid: number[][];
    peopleLocations: number[][];
    money: number;
    time: number;
    gameState: GameState;
    RCINeed: RCINeed;
}
export const defaultState: State = {
    activeBuild: ZoneType.NONE,
    grid: [],
    peopleLocations: [],
    money: 0,
    time: 0,
    gameState: GameState.STOP,
    RCINeed: {
        residential: 0,
        commercial: 0,
        industrial: 0
    }
};
export type Reducer = (prev: State) => State | undefined;

export function Home({ DOM, onion, pixi, ipc, time }: Sources): Sinks {
    const intent$: Stream<Reducer> = intentFn(DOM, pixi, ipc, onion.state$);
    const vdom$: Stream<VNode> = view(onion.state$);
    const request$: Stream<Action> = request(DOM, ipc, onion.state$, time);

    const gridDom$: MemoryStream<PixiInput> = DOM.select("#grid").element().take(1);
    const init$ = ipc.events
        .filter((data: any) => !!data)
        .filter((data: any) => data.type === "getZoneGrid")
        .take(1)
        .map((d: any) => d.payload);
    const grid$ = onion.state$.map(state => state.grid);
    // debug usage
    ipc.events.subscribe({
        next: console.log,
        error: console.error,
        complete: console.log
    });

    return {
        DOM: vdom$,
        onion: intent$,
        pixi: concat(gridDom$, init$, grid$),
        ipc: request$
    };
}

function request(DOM: DOMSource, ipc: IPCSink, state$: Stream<State>, time: TimeSource): Stream<Action> {
    const startEvent$: Stream<Action> = xs.combine(
        DOM.select(".start-button").events("click"),
        state$
    ).map(([_, state]): Action => {
        return {
            type: (state.gameState === GameState.START) ? "quitGame" : "startGame"
        };
    });
    const startGameFollowedUp$: Stream<Action> = ipc.events
        .filter((action: any): boolean => action.type === "startGame")
        .mapTo<Stream<Action>>(getState())
        .compose(flattenConcurrently);
    const tick$: Stream<Action> = time.periodic(1000)
        .mapTo<Stream<Action>>(getState())
        .compose(flattenConcurrently);
    return xs.merge(startEvent$, startGameFollowedUp$, tick$);

    function getState(): Stream<Action> {
        return xs.from([
            {type: "getTime"},
            {type: "getMoney"},
            {type: "getPeopleLocation"},
            {type: "getRCINeed"}
        ]);
    }
}

function intentFn(DOM: DOMSource, pixi: any, ipc: any, state$: Stream<State>): Stream<Reducer> {
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

    const systemEvents$: Stream<Reducer> = ipc.events
        .map((action: any): Reducer => {
            console.log("got event in views", action);
            return (state) => {
                if (!action) {
                    return state;
                }
                switch (action.type) {
                    case "startGame":
                        return {
                            ...state,
                            gameState: GameState.START
                        };
                    case "getZoneGrid":
                        return {
                            ...state,
                            grid: action.payload
                        };
                    case "getTime":
                        return {
                            ...state,
                            time: action.payload
                        };
                    case "getMoney":
                        return {
                            ...state,
                            money: action.payload
                        };
                    case "getPeopleLocation":
                        return {
                            ...state,
                            peopleLocation: action.payload
                        };
                    case "getRCINeed":
                        return {
                            ...state,
                            RCINeed: action.payload
                        };
                    default:
                        return state;
                }
            }
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

    return concat(init$, xs.merge(build$, changeActive$, systemEvents$));
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
