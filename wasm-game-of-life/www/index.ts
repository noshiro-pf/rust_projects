import { Universe } from 'wasm-game-of-life/wasm_game_of_life';

const main = () => {
  const pre = document.getElementById('game-of-life-canvas');

  if (pre == null) {
    console.error("element of id 'game-of-life-canvas' not found.");
    return;
  }

  const universe = Universe.new();

  const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
};

main();
