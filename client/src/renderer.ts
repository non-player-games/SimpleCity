import {create, drawGrid} from "./views/grid";

const mockGrid: number[][] = [
    [1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 0, 2],
    [0, 1, 1, 0, 2]
];
let activeBuildType: string = "";
const store = {
    grid: mockGrid,
    activeBuildType
};

// getting dom elements
const container = document.querySelector("#grid");
const buildButtons: any = document.querySelectorAll(".color-circle");
console.log(buildButtons);

buildButtons.forEach(( btn ) => {
    btn.onclick = () => {
        buildButtons.forEach(toggleButton(false));
        const buttonType = btn.dataset.type;
        store.activeBuildType = buttonType;
        toggleButton(true)(btn);
    };
});
function toggleButton(toggle) {
    return function(btn) {
        if (toggle) {
            btn.classList.add("active");
        } else {
            btn.classList.remove("active");
        }
    };
}

const app = create(container);
drawGrid(app, mockGrid, store);

