import {create, drawGrid} from "./views/grid";

// getting dom elements
const container = document.querySelector("#grid");
const buildButtons = document.querySelectorAll(".color-circle");

const mockGrid: number[][] = [
    [1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 0, 2],
    [0, 1, 1, 0, 2]
];
const activeBuildType: string = "";

const app = create(container);
drawGrid(app, mockGrid);

