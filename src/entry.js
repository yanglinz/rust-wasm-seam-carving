import React, { useReducer, useEffect } from "react";
import ReactDOM from "react-dom";

import { SeamCarver, wasm_memory as memory } from "./pkg";
import { onDocumentReady } from "./helpers/dom";
import ImageCanvas, { getCanvasElements } from "./components/ImageCanvas";
import Controls from "./components/Controls";

import "./index.css";

const initialState = { display: "SOURCE" };

function reducer(state, action) {
  switch (action.type) {
    case "RESIZE":
      return { display: "TARGET" };
    default:
      throw new Error("Unknown action type in reducer.");
  }
}

function App() {
  const [state, dispatch] = useReducer(reducer, initialState);

  function loadImage() {
    const { source } = getCanvasElements();
    const DEMO_IMAGE = "https://source.unsplash.com/yRjLihK35Yw/500x250";

    // TODO: Handle image loading failure
    return fetch(DEMO_IMAGE)
      .then((r) => r.blob())
      .then((blob) => createImageBitmap(blob))
      .then((img) => {
        // TODO: Handle high DPI screens / scale down heuristics
        source.width = img.width;
        source.height = img.height;

        const ctx = source.getContext("2d");
        // prettier-ignore
        ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, source.width, source.height);
        // prettier-ignore

        const imageData = ctx.getImageData(0, 0, source.width, source.height);
        console.log(imageData);
      });
  }

  function handleResize() {
    const { source, target } = getCanvasElements();

    const carver = SeamCarver.new(
      source.getContext("2d"),
      source.width,
      source.height
    );

    function resize1px() {
      // Modify the image
      carver.mark_seam();
      carver.delete_seam();

      // Get the image data
      const imageDataPtr = carver.image_data_ptr();
      const imageData = new Uint8ClampedArray(
        memory().buffer,
        imageDataPtr,
        carver.width * carver.height * 4
      );

      // Draw the image data
      target.width = carver.width;
      target.height = carver.height;
      const imageDataWrapper = new ImageData(
        imageData,
        carver.width,
        carver.height
      );
      target.getContext("2d").putImageData(imageDataWrapper, 0, 0);
    }

    let steps = 100;
    function incrementalResize() {
      if (steps <= 0) {
        return;
      }

      resize1px();
      steps -= 1;

      requestAnimationFrame(incrementalResize);
    }

    requestAnimationFrame(incrementalResize);

    dispatch({ type: "RESIZE" });
  }

  useEffect(loadImage, []);

  return (
    <div className="App flex flex-col h-screen">
      <div className="flex-grow">
        <div className="flex items-center	justify-center h-full">
          <ImageCanvas currentDisplay={state.display} />
        </div>
      </div>

      <div className="border-t border-gray-150 p-10 bg-white">
        <Controls handleResize={handleResize} />
      </div>
    </div>
  );
}

onDocumentReady(() => {
  ReactDOM.render(<App />, document.getElementById("app"));
});

if (module.hot) {
  // Disable HMR in development
  module.hot.decline();
}
