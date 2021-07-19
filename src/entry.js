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

        // const imageData = ctx.getImageData(0, 0, source.width, source.height);
        // console.log(imageData);
      });
  }

  function handleResize() {
    const { source } = getCanvasElements();

    const canvas = document.getElementById("canvas-source");
    const ctx = canvas.getContext("2d");

    const carver = SeamCarver.new(ctx, 32, 45);
    carver.mark_seam();
    carver.delete_seam();

    const imageDataPtr = carver.image_data_ptr();
    const rustValues = new Uint8Array(memory().buffer, imageDataPtr, 4);

    console.log({ imageDataPtr });
    console.log(rustValues);

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
