import {create, drawGrid} from "./views/grid";

const container = document.querySelector("#grid");

const mockGrid: number[][] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 0, 2],
    [0, 1, 1, 0, 2]
];

const app = create(container);
drawGrid(app, mockGrid);
