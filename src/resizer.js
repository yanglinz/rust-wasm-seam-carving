import { useEffect, useRef } from "react";

import { resizeImage } from "./lib";

function Resizer() {
  const canvasOriginal = useRef(null);
  const canvasResized = useRef(null);

  function getCanvasElements() {
    return {
      original: canvasOriginal.current,
      resized: canvasResized.current,
    };
  }

  function handleResize() {
    // First copy the image as-is to the target canvas
    //
    const currentWidth = "";
    const currentHeight = "";
    console.log(currentWidth, currentHeight);
    // carverRedize(getCanvasElements());
  }

  const canvasStyle = { background: "rgba(0, 0, 0, 0.05)" };
  return (
    <div>
      <div>
        <canvas ref={canvasOriginal} style={canvasStyle}></canvas>
      </div>
      <div>
        <canvas ref={canvasResized} style={canvasStyle}></canvas>
      </div>
    </div>
  );
}

export default Resizer;
