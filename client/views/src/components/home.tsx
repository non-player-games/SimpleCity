import xs, { MemoryStream, Stream } from 'xstream';
import concat from 'xstream/extra/concat';
import { VNode, DOMSource } from '@cycle/dom';
import { StateSource } from 'cycle-onionify';

import { BaseSources, BaseSinks } from '../interfaces';

// Types
export interface Sources extends BaseSources {
    onion: StateSource<State>;
    pixi: any; // TODO define the source from driver
}
export interface Sinks extends BaseSinks {
    onion?: Stream<Reducer>;
    pixi?: any;
}

// TODO: move common type definition to other package
enum ZoneType {
    NONE,
    RESIDENTIAL,
    COMMERCIAL,
    INDUSTRIAL
}

const mockGrid: number[][] = [
    [1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 0, 2],
    [0, 1, 1, 0, 2]
];

// State
export interface State {
    activeBuild?: ZoneType;
    grid: number[][];
}
export const defaultState: State = {
    activeBuild: ZoneType.NONE,
    grid: mockGrid
};
export type Reducer = (prev: State) => State | undefined;

export function Home({ DOM, onion, pixi }: Sources): Sinks {
    const action$: Stream<Reducer> = intent(DOM, pixi);
    const vdom$: Stream<VNode> = view(onion.state$);
    const gridDom$: MemoryStream<string | Element | Document | HTMLBodyElement | number[][]> = DOM.select('#grid').element().take(1);
    const init$ = xs.of(mockGrid);
    const grid$ = onion.state$.map(state => state.grid);

    return {
        DOM: vdom$,
        onion: action$,
        pixi: concat(gridDom$, init$, grid$)
    };
}

function intent(DOM: DOMSource, pixi: any): Stream<Reducer> {
    const init$ = xs.of<Reducer>(
        prevState => (prevState === undefined ? defaultState : prevState)
    );
    const changeActive$: Stream<Reducer> = DOM.select('.color-circle')
        .events('click')
        .map((evt:any): Reducer => {
            const t: string = evt.target.dataset.type;
            return (state) => {
                return {
                    ...state,
                    activeBuild: ZoneType[t]
                };
            };
        })

    const build$: Stream<Reducer> = pixi.events
        .map((data:any): Reducer => {
            return (state) => {
                return {
                    ...state,
                    grid: updateGrid(state.grid, data.i, data.j, state.activeBuild)
                };
            }
        });

    return concat(init$, xs.merge(build$, changeActive$));
}

function updateGrid(grid: number[][], i: number, j: number, buildType: ZoneType): number[][] {
    const copy = deepCopy(grid);
    copy[i][j] = buildType;
    return copy
}
function deepCopy<T> (obj: T): T {
    return JSON.parse(JSON.stringify(obj));
}

function view(state$: Stream<State>): Stream<VNode> {
    return state$.map(state => (
        <div className="fill-parent">
            <div className="info floating-panel">
            $: 100
            </div>
            <div id="grid" className="fill-paent"></div>
            <div className="actions floating-panel">
                {Object.keys(ZoneType).filter(k => typeof ZoneType[k as any] === "number").map(z => {
                    return <button
                        className={getColorCircleClass(state.activeBuild, z)}
                        data-type={z}
                        on-click={setActiveZone(z)}>
                    </button>
                })}
            </div>
        </div>
    ));
}

function setActiveZone(z: string) {
    return function(): void {
        console.log(z);
    };
}

function getColorCircleClass(active: ZoneType, t: string): string {
    const className = `${t.toLowerCase()} color-circle`;
    return active === ZoneType[t] ? `active ` + className : className;
}
