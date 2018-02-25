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

const mockGrid: number[][] = [
    [1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 0, 2],
    [0, 1, 1, 0, 2]
];

// State
export interface State {
    count: number;
    activeBuild?: string;
}
export const defaultState: State = {
    count: 30
};
export type Reducer = (prev: State) => State | undefined;

export function Home({ DOM, onion, pixi }: Sources): Sinks {
    const action$: Stream<Reducer> = intent(DOM);
    const vdom$: Stream<VNode> = view(onion.state$);
    const gridDom$: MemoryStream<string | Element | Document | HTMLBodyElement | number[][]> = DOM.select('#grid').element().take(1);
    const state$: Stream<number[][]> = xs.of(mockGrid);
    pixi.events.addListener({next: (data: any) => {
        console.log(data);
    }});

    return {
        DOM: vdom$,
        onion: action$,
        pixi: concat(gridDom$, state$)
    };
}

function intent(DOM: DOMSource): Stream<Reducer> {
    const init$ = xs.of<Reducer>(
        prevState => (prevState === undefined ? defaultState : prevState)
    );

    const add$: Stream<Reducer> = DOM.select('.add')
        .events('click')
        .mapTo<Reducer>(state => ({ ...state, count: state.count + 1 }));

    const subtract$: Stream<Reducer> = DOM.select('.subtract')
        .events('click')
        .mapTo<Reducer>(state => ({ ...state, count: state.count - 1 }));

    return xs.merge(init$, add$, subtract$);
}

function view(state$: Stream<State>): Stream<VNode> {
    return state$.map(() => (
        <div className="fill-parent">
            <div className="info floating-panel">
            $: 100
            </div>
            <div id="grid" className="fill-paent"></div>
            <div className="actions floating-panel">
                <button className="none color-circle" data-type="NONE"></button>
                <button className="residential color-circle" data-type="RESIDENTIAL"></button>
                <button className="commercial color-circle" data-type="COMMERCIAL"></button>
                <button className="industrial color-circle" data-type="INDUSTRIAL"></button>
            </div>
        </div>
    ));
}
