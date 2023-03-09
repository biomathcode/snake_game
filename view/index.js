import init, { World } from "snake_game";

init().then((_) => {
  const CELL_SIZE = 10;
  const world = World.new();
  const worldWidth = world.get_w();

  const canvas = document.getElementById("canvas");

  const ctx = canvas.getContext("2d");
  canvas.width = world.get_w() * CELL_SIZE;
  canvas.height = world.get_w() * CELL_SIZE;

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
    const snakeIdx = world.snake_head();

    const col = snakeIdx % worldWidth;
    const row = Math.floor(snakeIdx / worldWidth);

    ctx.beginPath();
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  drawWorld();
  drawSnake();

  setInterval(() => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawWorld();
    drawSnake();
    world.update();
  }, 400);
});