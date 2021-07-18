import { memory } from "./pkg/content_aware_image_resizer_bg.wasm";

console.log("Hello world!", memory);

// TODO: 
// 1. Initialize image
// 2. Copy image array to Rust WASM
// 3. Implement a Carver().tick() mechanism
// 4. Read post-.tick() array pointer
// 5. Draw post-.tick() pointer
// 6. Do it all in an animation loop
