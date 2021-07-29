import { useState, useEffect } from "react";

function WidthSlider(props) {
  const { minWidth, maxWidth, disabled, onChange } = props;
  const [width, setWidth] = useState(maxWidth);

  let sliderProps = { disabled: true };
  if (!disabled) {
    sliderProps = {
      min: minWidth,
      max: maxWidth,
      value: width,
      onChange: (e) => {
        const w = parseInt(e.target.value);
        setWidth(w);
        onChange(w);
      },
    };
  }

  return (
    <div className="flex justify-center">
      <div className="flex w-8/12 align-center">
        <input
          id="resize"
          name="resize"
          type="range"
          className="flex-grow"
          {...sliderProps}
        />

        <label className="font-mono w-20 text-right text-gray-700" for="resize">
          {disabled ? 0 : width}px
        </label>
      </div>
    </div>
  );
}

function Controls(props) {
  const { globalState, handleResize, handleOpenImageSelect } = props;

  const { selectedImage } = globalState;
  const [resizedWidth, setResizedWidth] = useState(0);

  const resizeActionEnabled =
    selectedImage.state === "SOURCE" &&
    resizedWidth !== 0 &&
    resizedWidth !== selectedImage.width;
  return (
    <div className="Controls">
      <div className="pb-3">
        <WidthSlider
          key={selectedImage.url}
          disabled={selectedImage.state !== "SOURCE"}
          minWidth={20}
          maxWidth={selectedImage.width}
          onChange={setResizedWidth}
        />
      </div>

      <div className="flex justify-center">
        <div className="flex">
          <button
            className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
            disabled={!resizeActionEnabled}
            onClick={() => handleResize(resizedWidth)}
          >
            Resize Image
          </button>

          <div className="px-1"></div>

          <button
            className="bg-gray-100 hover:bg-gray-200 text-gray-700 font-bold py-2 px-4 rounded disabled:opacity-50"
            onClick={handleOpenImageSelect}
          >
            Try Another Image
          </button>
        </div>
      </div>
    </div>
  );
}

export default Controls;
