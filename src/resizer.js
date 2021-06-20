import { useEffect } from "react";

function loadImage(canvas) {
  const ctx = canvas.getContext("2d");

  const img = new Image();
  img.crossOrigin = "Anonymous";
  img.src = "https://source.unsplash.com/collection/190727/800x450";

  img.onload = function () {
    ctx.drawImage(
      img,
      0,
      0,
      img.width,
      img.height,
      0,
      0,
      canvas.width,
      canvas.height
    );

    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    console.log(imageData);
  };
  img.onerror = function (e) {
    console.log("error", e);
  };
}

function carverRedize(canvas) {
  function wasmResize(...args) {
    import("./pkg")
      .then((module) => module.resize(...args))
      .catch(console.error);
  }

  const ctx = canvas.getContext("2d");
  const canvasTarget = document.getElementById("test-canvas");
  const ctxTarget = canvasTarget.getContext("2d");

  wasmResize(
    ctx,
    ctxTarget,
    canvas.width,
    canvas.height,
    canvas.width - 50,
    canvas.height
  );
}

function Resizer() {
  useEffect(() => {
    const canvas = document.getElementById("app-canvas");
    loadImage(canvas);
  }, []);

  function handleResize() {
    const canvas = document.getElementById("app-canvas");
    carverRedize(canvas);
  }

  return (
    <div>
      <button onClick={handleResize}>Click me!</button>
      <hr />
      <div>
        <canvas id="app-canvas"></canvas>
      </div>
      <div>
        <canvas
          id="test-canvas"
          style={{ width: 300, height: 150, background: "#ddd" }}
        ></canvas>
      </div>
    </div>
  );
}

export default Resizer;
