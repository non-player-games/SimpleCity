import * as path from "path";
import * as ffi from "ffi";

// define system function signatures
interface Systems {
    calc(n: number): number;
}

// TODO: maybe toggle this file to use release if NODE_ENV is prod
const systemFile: string = path.join(
    __dirname,
    "../systems/libsystems"
);


// getting system function from ffi
const systemsLib = ffi.Library(systemFile, {
    calc: ["int", ["int"]]
});

// construct systems with interface above
const systems: Systems = {
    calc: systemsLib.calc
};

export default systems;
