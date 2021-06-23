import { useEffect, useRef } from "react";

const DEMO_IMAGE = "https://source.unsplash.com/YJjm6XD6zF4/800x450";

function loadImage(url) {
  const img = new Image();
  img.crossOrigin = "Anonymous";
  img.src = url;

  return new Promise((resolve, reject) => {
    img.onload = function () {
      resolve(img);
    };
    img.onerror = function (e) {
      reject(e);
    };
  });
}

function loadCanvasImage(canvasElements) {
  const { original, resized } = canvasElements;

  loadImage(DEMO_IMAGE).then((img) => {
    // TODO: Think of high DPI screens / scale down heuristics
    original.width = img.width;
    original.height = img.height;
    resized.width = img.width;
    resized.height = img.height;

    const ctxOriginal = original.getContext("2d");
    // prettier-ignore
    ctxOriginal.drawImage(img, 0, 0, img.width, img.height, 0, 0, original.width, original.height);
    // prettier-ignore
    const imageData = ctxOriginal.getImageData(0, 0, original.width, original.height);
    console.log(imageData);
  });
}

function carverRedize(canvasElements) {
  const { original, resized } = canvasElements;

  function wasmResize(...args) {
    import("./pkg")
      .then((module) => module.resize(...args))
      .catch(console.error);
  }

  const ctxOrignal = original.getContext("2d");
  const ctxResized = resized.getContext("2d");

  wasmResize(
    ctxOrignal,
    ctxResized,
    canvas.width,
    canvas.height,
    canvas.width - 50,
    canvas.height
  );
}

function getCanvas() {}

function Resizer() {
  const canvasOriginal = useRef(null);
  const canvasResized = useRef(null);

  function getCanvasElements() {
    return {
      original: canvasOriginal.current,
      resized: canvasResized.current,
    };
  }

  useEffect(() => {
    if (canvasOriginal && canvasResized) {
      loadCanvasImage(getCanvasElements());
    }
  }, [canvasOriginal, canvasResized]);

  function handleResize() {
    carverRedize(getCanvasElements());
  }

  return (
    <div>
      <button onClick={handleResize}>Click me!</button>
      <hr />
      <div>
        <canvas ref={canvasOriginal}></canvas>
      </div>
      <div>
        <canvas ref={canvasResized}></canvas>
      </div>
    </div>
  );
}

export default Resizer;
