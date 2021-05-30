import React from "react";
import ReactDOM from "react-dom";
import Resizer from "./resizer";

if (module.hot) {
  // Disable HMR in development
  module.hot.decline();
}

function App() {
  return (
    <div>
      <h1>Content Aware Image Resizer</h1>
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
