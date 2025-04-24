import './style.css'

import init, { GameWrapper } from '../snake-game/pkg/snake_game.js';

let score = 0;
let highScore = localStorage.getItem('highScore') || 0;
let gameWrapper;

function updateScore(newScore) {
  score = newScore;
  document.getElementById('score').textContent = score;
  if (score > highScore) {
    updateHighScore(score);
  }
}

function updateHighScore(newHighScore) {
  highScore = newHighScore;
  localStorage.setItem('highScore', newHighScore);
  document.getElementById('high-score-value').textContent = newHighScore;
}

function gameOver() {
  document.getElementById('final__score').textContent = score;
  document.getElementById('final__high-score-value').textContent = highScore;
  document.getElementById('game-over').style.display = 'flex';
}

function restart() {
  document.getElementById('game-over').style.display = 'none';
  updateScore(0);
  gameWrapper.restart(updateScore, gameOver);
}

async function run() {
  await init();
  const canvas = document.getElementById('snake-canvas');
  gameWrapper = new GameWrapper(canvas);
  gameWrapper.start(updateScore, gameOver);
}

window.restart = restart;

updateHighScore(highScore);
run().catch(console.error);