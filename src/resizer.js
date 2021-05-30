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
  function wasmResize(ctx) {
    import("./pkg")
      .then((module) => {
        const val = module.resize(ctx);
        console.log(val);
      })
      .catch(console.error);
  }

  const ctx = canvas.getContext("2d");
  wasmResize(ctx);
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
    </div>
  );
}

export default Resizer;
