import React, { useReducer, useEffect } from "react";
import ReactDOM from "react-dom";

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
    import("./entry")
      .then((module) => module.initialize())
      .catch(console.error);
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
