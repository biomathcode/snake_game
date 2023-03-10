import init, { World, Direction } from "snake_game";
import { rnd } from "./utils/random";

init().then((wasm) => {
  const CELL_SIZE = 10;
  const WORLD_WIDTH = 8;
  const SNAKE_SPAWN_IDX = rnd(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);
  const worldWidth = world.get_w();

  const canvas = <HTMLCanvasElement>document.getElementById("canvas");

  const ctx = canvas.getContext("2d");
  canvas.width = world.get_w() * CELL_SIZE;
  canvas.height = world.get_w() * CELL_SIZE;

  const snakeCellPtr = world.snake_cells(); // pointer address
  const snakeLength = world.snake_length();

  const snakeCells = new Uint32Array(
    wasm.memory.buffer,
    snakeCellPtr,
    snakeLength
  );

  console.log(snakeCells);

  document.addEventListener("keydown", (e) => {
    switch (e.code) {
      case "ArrowLeft":
        world.change_direction(Direction.LEFT);
        break;
      case "ArrowUp":
        world.change_direction(Direction.UP);

        break;

      case "ArrowDown":
        world.change_direction(Direction.DOWN);

        break;

      case "ArrowRight":
        world.change_direction(Direction.RIGHT);

        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    );

    snakeCells.forEach((cell, i) => {
      const col = cell % worldWidth;
      const row = Math.floor(cell / worldWidth);
      ctx.fillStyle = i === 0 ? "#7878db" : "#000";

      ctx.beginPath();
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });

    ctx.stroke();
  }
  function drawReward() {
    const rewardIdx = world.get_reward();

    const col = rewardIdx % worldWidth;
    const row = Math.floor(rewardIdx / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = "#ff0000";

    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx.stroke();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
  }

  function update() {
    const fps = 10;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  paint();
  update();
});
