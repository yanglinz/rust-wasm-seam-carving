import { useEffect, useRef } from "react";

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

function loadCanvasImage(imageSrc, canvasElement) {
  loadImage(imageSrc).then((img) => {
    // TODO: Think of high DPI screens / scale down heuristics
    canvasElement.width = img.width;
    canvasElement.height = img.height;

    const ctx = canvasElement.getContext("2d");
    // prettier-ignore
    ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, canvasElement.width, canvasElement.height);
    // prettier-ignore
    const imageData = ctx.getImageData(0, 0, canvasElement.width, canvasElement.height);
    console.log(imageData);
  });
}

function CanvasImage(props) {
  const ref = useRef(null);

  useEffect(() => {
    if (ref) {
      loadCanvasImage(props.src, ref.current);
    }
  }, [ref]);

  return <canvas id={props.id} ref={ref}></canvas>;
}

export default CanvasImage;
