import React from "react";
import ReactDOM from "react-dom";

if (module.hot) {
  // Disable HMR in development
  module.hot.dispose();
}

function loadWasm() {
  import("./pkg")
    .then((module) => {
      module.greet("foo");
    })
    .catch(console.error);
}

function App() {
  return (
    <div>
      <h1>Content Aware Image Resizer</h1>
      <canvas></canvas>
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById("app"));
