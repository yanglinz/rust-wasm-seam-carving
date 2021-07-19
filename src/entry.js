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

  function init() {
    const DEMO_IMAGE = "https://source.unsplash.com/yRjLihK35Yw/800x450";
    const canvas = document.getElementById("canvas-source");
    const ctx = canvas.getContext("2d");

    const carver = SeamCarver.new(ctx, 32, 45);
    carver.mark_seam();
    carver.delete_seam();

    const imageDataPtr = carver.image_data_ptr();
    const rustValues = new Uint8Array(memory().buffer, imageDataPtr, 4);

    console.log({ imageDataPtr });
    console.log(rustValues);
  }

  function handleResize() {
    const { source } = getCanvasElements();

    dispatch({ type: "RESIZE" });
  }

  useEffect(init, []);

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
