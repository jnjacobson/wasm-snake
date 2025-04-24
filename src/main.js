import './style.css'

import init, { Game } from '../snake-game/pkg/snake_game.js';

async function run() {
  await init();
  const canvas = document.getElementById('snake-canvas');

  const game = Game.new(canvas);

  document.addEventListener('keydown', (event) => {
    game.change_direction(event.key);
  });

  function gameLoop() {
    game.update();
    game.draw();
    setTimeout(gameLoop, 100);
  }

  gameLoop();
}

run().catch(console.error);