import { SeamCarver, wasm_memory } from "./pkg";

// TODO:
// 1. Initialize image
// 2. Copy image array to Rust WASM
// 4. Read post-.tick() array pointer
// 5. Draw post-.tick() pointer
// 6. Do it all in an animation loop

function memory() {
  return wasm_memory();
}

export function initialize() {
  const canvas = document.getElementById("canvas-source");
  const ctx = canvas.getContext("2d");

  const carver = SeamCarver.new(ctx, 32, 45);
  carver.mark_seam();
  carver.delete_seam();

  const imageDataPtr = carver.image_data_ptr();
  const rustValues = new Uint8Array(memory().buffer, imageDataPtr, 4);

  console.log({ imageDataPtr });
  console.log(rustValues);
}
