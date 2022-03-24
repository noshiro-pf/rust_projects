import { memory } from 'wasm-game-of-life/wasm_game_of_life_bg.wasm'; // this should be placed at the begining of imports
import { Universe } from 'wasm-game-of-life/wasm_game_of_life';

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

const drawGridImpl = (
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number
) => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const drawCellsImpl = (
  universe: Universe,
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  getIndex: (row: number, column: number) => number
) => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

const bitIsSet = (n: number, arr: Uint8Array): boolean => {
  const byte = Math.floor(n / 8);
  const mask = 1 << n % 8;
  const e = arr[byte];
  return e === undefined ? false : (e & mask) === mask;
};

const main = () => {
  // Construct the universe, and get its width and height.
  const universe = Universe.new();
  const width = universe.width();
  const height = universe.height();

  // Give the canvas room for all of our cells and a 1px border
  // around each of them.
  const canvas = document.getElementById(
    'game-of-life-canvas'
  ) as HTMLCanvasElement | null;

  if (canvas == null) {
    console.error("element of id 'game-of-life-canvas' not found.");
    return;
  }

  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const ctx = canvas.getContext('2d');

  if (ctx == null) {
    console.error("element of id 'game-of-life-canvas' not found.");
    return;
  }

  const getIndex = (row: number, column: number) => row * width + column;

  const drawGrid = () => {
    drawGridImpl(ctx, width, height);
  };

  const drawCells = () => {
    drawCellsImpl(universe, ctx, width, height, getIndex);
  };

  const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
  };

  drawGrid();
  drawCells();
  requestAnimationFrame(renderLoop);
};

main();
