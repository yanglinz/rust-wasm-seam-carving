import "./style.css";

function documentReady(fn) {
  if (document.readyState != "loading") {
    fn();
  } else {
    document.addEventListener("DOMContentLoaded", fn);
  }
}

function setupCanvas() {
  const canvas = document.createElement("canvas");
  canvas.id = "app-image-mount";
  document.getElementById("app").appendChild(canvas);
  const convasContext = canvas.getContext("2d");

  const img = new Image();
  img.crossOrigin = "Anonymous";
  img.src = "https://source.unsplash.com/collection/190727/800x450";

  img.onload = function () {
    convasContext.drawImage(
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

    const imageData = convasContext.getImageData(
      0,
      0,
      canvas.width,
      canvas.height
    );
    console.log(imageData);
  };
  img.onerror = function (e) {
    console.log("error", e);
  };
}

function setupRust() {
  const rust = import("../pkg/content_aware_image_resizer_bg.wasm");

  rust
    .then((m) => {
      console.log(m)
      m.greet("World!");
    })
    .catch(console.error);
}

function setup() {
  // setupCanvas();
  setupRust();
}

documentReady(setup);
