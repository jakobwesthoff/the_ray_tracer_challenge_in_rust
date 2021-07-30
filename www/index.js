import { World } from "the-raytracer-challenge";
const size = 2048;
const perFrame = size*4;

const w = World.new(size);

const canvas = document.createElement('canvas');
canvas.setAttribute('width', size);
canvas.setAttribute('height', size);
canvas.style.width = size / 2 + "px";
canvas.style.height = size / 2 + "px";

document.body.appendChild(canvas);

const context = canvas.getContext('2d');

let y = 0;
let x = 0;

function doAnimationFrame() {
  // console.log("calling render for ", x, "x", y);
  let thisFrame = 0;
  while (thisFrame < perFrame && y < size) {
    w.render(context, x, y);

    x += 1;
    if (x >= size) {
      x = 0;
      y += 1;
    }
    thisFrame += 1;
  }

  if (y < size) {
    requestAnimationFrame(doAnimationFrame);
  }
}

requestAnimationFrame(doAnimationFrame);
