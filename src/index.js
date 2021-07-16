import React from "react";
import ReactDOM from "react-dom";

import Resizer from "./resizer";
import Controls from "./components/Controls";

import "./index.css";

if (module.hot) {
  // Disable HMR in development
  module.hot.decline();
}

function App() {
  return (
    <div className="App flex flex-col h-screen">
      <div className="flex-grow">
        <Resizer />
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
