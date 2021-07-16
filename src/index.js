import React from "react";
import ReactDOM from "react-dom";

import DisplayImages from "./components/DisplayImages";
import Controls from "./components/Controls";

import "./index.css";

const DEMO_IMAGE = "https://source.unsplash.com/yRjLihK35Yw/800x450";

function App() {
  return (
    <div className="App flex flex-col h-screen">
      <div className="flex-grow">
        <div className="flex items-center	justify-center h-full">
          <DisplayImages src={DEMO_IMAGE} currentDisplay="SOURCE" />
        </div>
      </div>

      <div className="border-t border-gray-150 p-10 bg-white">
        <Controls />
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
