import React, { useReducer, useEffect } from "react";
import ReactDOM from "react-dom";

import { onDocumentReady } from "./helpers/dom";
import { memoize } from "./helpers/cache";
import Worker from "worker-loader!./worker";
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

function _getWorkerInstance() {
  const _worker = new Worker();
  const { source, target } = getCanvasElements();

  // We can only send canvas elements once from the main thread to the worker.
  // Once sent, the offscreen canvas are in a detached state.
  // So we'll call this once at the start of the worker lifecycle.
  _worker.postMessage(["init", { source, target }], [source, target]);
  return _worker;
}

const getWorker = memoize(_getWorkerInstance);

function App() {
  const [state, dispatch] = useReducer(reducer, initialState);

  function loadImage() {
    // const DEMO_IMAGE = "https://source.unsplash.com/yRjLihK35Yw/800x450";
    const worker = getWorker();
    // worker.postMessage(["loadSourceImage", DEMO_IMAGE]);
  }

  function handleResize() {
    const { detachedSource } = getCanvasElements();

    const worker = getWorker();
    const resizedWidth = detachedSource.width - 2;
    const resizedHeight = detachedSource.height;

    worker.postMessage([
      "resizeTargetImage",
      detachedSource.width,
      detachedSource.height,
      resizedWidth,
      resizedHeight,
    ]);

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
