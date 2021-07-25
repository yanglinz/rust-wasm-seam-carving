export function getCanvasElements() {
  const source = document.getElementById("canvas-source");
  const target = document.getElementById("canvas-target");
  return { source, target };
}

function ImageCanvas(props) {
  const { selectedImage } = props.globalState;
  const originalClassNames = selectedImage.state === "SOURCE" ? "" : "hidden";
  const targetClassNames = selectedImage.state === "TARGET" ? "" : "hidden";

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
