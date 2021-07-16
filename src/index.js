import React, { useReducer } from "react";
import ReactDOM from "react-dom";

import { resizeImage } from "./lib";
import DisplayImages from "./components/DisplayImages";
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

  function handleResize() {
    const source = document.getElementById("canvas-source");
    const target = document.getElementById("canvas-target");
    target.width = source.width;
    target.height = source.height;

    const resizedWidth = source.width - 2;
    const resizedHeight = source.height;
    resizeImage(
      { source, target },
      source.width,
      source.height,
      resizedWidth,
      resizedHeight
    );

    dispatch({ type: "RESIZE" });
  }

  const DEMO_IMAGE = "https://source.unsplash.com/yRjLihK35Yw/800x150";
  return (
    <div className="App flex flex-col h-screen">
      <div className="flex-grow">
        <div className="flex items-center	justify-center h-full">
          <DisplayImages src={DEMO_IMAGE} currentDisplay={state.display} />
        </div>
      </div>

      <div className="border-t border-gray-150 p-10 bg-white">
        <Controls handleResize={handleResize} />
      </div>
    </div>
  );
}

function documentReady(fn) {
  if (document.readyState != "loading") {
    fn();
  } else {
    document.addEventListener("DOMContentLoaded", fn);
  }
}

documentReady(() => {
  ReactDOM.render(<App />, document.getElementById("app"));
});

if (module.hot) {
  // Disable HMR in development
  module.hot.decline();
}
