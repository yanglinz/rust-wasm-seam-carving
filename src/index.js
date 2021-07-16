import React from "react";
import ReactDOM from "react-dom";

import Resizer from "./resizer";

import "./index.css";

if (module.hot) {
  // Disable HMR in development
  module.hot.decline();
}

function App() {
  return (
    <div className="App">
      <Resizer />
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
