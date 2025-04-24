import './style.css'

import init, { SnakeGame } from '../snake-game/pkg/snake_game.js';

async function run() {
  await init();
  const canvas = document.getElementById('snake-canvas');

  const game = new SnakeGame(canvas);

  function gameLoop() {
    game.update();
    game.draw();
    setTimeout(gameLoop, 100);
  }

  gameLoop();
}

run().catch(console.error);