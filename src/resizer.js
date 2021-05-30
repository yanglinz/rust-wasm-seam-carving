function loadWasm() {
  import("./pkg")
    .then((module) => {
      module.greet("foo");
    })
    .catch(console.error);
}

function Resizer() {
  function handleResize() {
    const canvas = document.getElementById("app-canvas");
    const ctx = canvas.getContext("2d");
  }

  return (
    <div>
      <button onClick={handleResize}>Click me!</button>
      <div>
        <canvas id="app-canvas"></canvas>
      </div>
    </div>
  );
}

export default Resizer;
