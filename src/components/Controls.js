import { useState } from "react";

import Button from "./Button";

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
      <div className="align-center flex w-11/12 sm:w-8/12">
        <input
          id="resize"
          name="resize"
          type="range"
          className="flex-grow"
          {...sliderProps}
        />

        <label className="w-20 text-right text-gray-700 font-mono" for="resize">
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
          <Button
            intent="PRIMARY"
            disabled={!resizeActionEnabled}
            onClick={() => handleResize(resizedWidth)}
          >
            Resize Image
          </Button>

          <div className="px-1"></div>

          <Button intent="SECONDARY" onClick={handleOpenImageSelect}>
            Try Another Image
          </Button>
        </div>
      </div>
    </div>
  );
}

export default Controls;
