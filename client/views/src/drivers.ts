import xs, { Stream } from "xstream";
import { restartable } from "cycle-restart";
import { makeDOMDriver } from "@cycle/dom";
import { makeHistoryDriver } from "@cycle/history";
import { timeDriver } from "@cycle/time";
import { routerify, RouteMatcher } from "cyclic-router";
import onionify from "cycle-onionify";
import switchPath from "switch-path";

import {makePixiDriver} from "./drivers/pixi";

import { Component } from "./interfaces";

export type DriverThunk = Readonly<[string, () => any]> & [string, () => any]; // work around readonly
export type DriverThunkMapper = (t: DriverThunk) => DriverThunk;

// Set of Drivers used in this App
const driverThunks: DriverThunk[] = [
    ['DOM', () => makeDOMDriver('#app')],
    ['time', () => timeDriver],
    ['history', () => makeHistoryDriver()],
    ['pixi', () => makePixiDriver()]
];

export const buildDrivers = (fn: DriverThunkMapper) =>
    driverThunks
        .map(fn)
        .map(([n, t]: DriverThunk) => ({ [n]: t }))
        .reduce((a, c) => Object.assign(a, c), {});

export const driverNames = driverThunks
    .map(([n, t]) => n)
    .concat(['onion', 'router']);

export function wrapMain(main: Component): Component {
    return routerify(
        onionify(main as any),
        switchPath
    ) as any;
}
