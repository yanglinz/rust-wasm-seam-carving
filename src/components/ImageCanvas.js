function DisplayImages(props) {
  const { currentDisplay } = props;

  const originalClassNames = currentDisplay === "SOURCE" ? "" : "hidden";
  const targetClassNames = currentDisplay === "TARGET" ? "" : "hidden";

  return (
    <div className="border-8 border-gray-600 border-opacity-5">
      <div className={originalClassNames}>
        <canvas id="canvas-target"></canvas>
      </div>

      <div className={targetClassNames}>
        <canvas id="canvas-target"></canvas>
      </div>
    </div>
  );
}

export default DisplayImages;
