import { memoize } from "../helpers/cache";

function _getOffscreenCanvas(canvasId) {
  const canvas = document.getElementById(canvasId);
  return canvas.transferControlToOffscreen();
}

const getOffscreenCanvas = memoize(_getOffscreenCanvas);

export function getCanvasElements() {
  const source = document.getElementById("canvas-source");
  const offscreenSource = source
    ? getOffscreenCanvas("canvas-source")
    : undefined;
  const target = document.getElementById("canvas-target");
  const offscreenTarget = target
    ? getOffscreenCanvas("canvas-target")
    : undefined;
  return {
    source: offscreenSource,
    detachedSource: source,
    target: offscreenTarget,
    detachedTarget: target,
  };
}

function ImageCanvas(props) {
  const { currentDisplay } = props;

  const originalClassNames = currentDisplay === "SOURCE" ? "" : "hidden";
  const targetClassNames = currentDisplay === "TARGET" ? "" : "hidden";

  return (
    <div className="border-8 border-gray-600 border-opacity-5">
      <div className={originalClassNames}>
        <canvas id="canvas-source"></canvas>
      </div>

      <div className={targetClassNames}>
        <canvas id="canvas-target"></canvas>
      </div>
    </div>
  );
}

export default ImageCanvas;
