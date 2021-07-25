import { useState, useEffect } from "react";

function WidthSlider(props) {
  const { minWidth, maxWidth, onChange } = props;
  const [width, setWidth] = useState(maxWidth);
  return (
    <div className="flex">
      <input
        id="resize"
        name="resize"
        type="range"
        className="w-80 mr-3"
        min={minWidth}
        max={maxWidth}
        value={width}
        onChange={(e) => {
          const w = parseInt(e.target.value);
          setWidth(w);
          onChange(w);
        }}
      />

      <label for="resize">{width}px</label>
    </div>
  );
}

function WidthSliderPlaceholder() {
  return (
    <div className="flex">
      <input
        id="resize"
        name="resize"
        type="range"
        className="w-80 mr-3"
        disabled
      />
    </div>
  );
}

function Controls(props) {
  const { globalState, handleResize } = props;
  const { display, sourceWidth } = globalState;
  const [resizedWidth, setResizedWidth] = useState(0);

  const resizeActionEnabled =
    display === "SOURCE" && resizedWidth !== 0 && resizedWidth !== sourceWidth;
  return (
    <div className="Controls">
      <div className="pb-3">
        {display === "SOURCE" ? (
          <WidthSlider
            minWidth={20}
            maxWidth={sourceWidth}
            onChange={setResizedWidth}
          />
        ) : (
          <WidthSliderPlaceholder />
        )}
      </div>

      <div className="flex">
        <button
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
          disabled={!resizeActionEnabled}
          onClick={() => handleResize(resizedWidth)}
        >
          Resize Image
        </button>

        <div className="px-1"></div>

        <button className="bg-gray-100 hover:bg-gray-200 text-gray-700 font-bold py-2 px-4 rounded disabled:opacity-50">
          Try Another Image
        </button>
      </div>
    </div>
  );
}

export default Controls;
