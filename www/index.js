import * as Comlink from "comlink";
const size = 2048;

const canvas = document.createElement("canvas");
canvas.setAttribute("width", size);
canvas.setAttribute("height", size);
canvas.style.width = size / 2 + "px";
canvas.style.height = size / 2 + "px";

document.body.appendChild(canvas);

const context = canvas.getContext("2d");

async function spawnRenderer(start, end) {
  const worker = new Worker('./worker.js');
  const renderer = Comlink.wrap(worker);
  
  console.log("initializing ", start, "-", end);
  await renderer.init(size, { start: start, end: end });

  console.log("rendering ", start, "-", end);
  let result;
  while ((result = await renderer.renderNext()) !== false) {
    context.putImageData(result.data, 0, result.y);
  }
}

(async () => {
  const workers = navigator.hardwareConcurrency || 4;
  const perWorker = size / workers;
  for (let i = 0; i < workers; i++) {
    spawnRenderer(i * perWorker, (i + 1) * perWorker);
  }
})();
