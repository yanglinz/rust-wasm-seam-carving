// Any WASM module must be imported asynchronously. So here we'll wrap the entire app
// in a single `import()` so that child modules don't have to deal with the async loading.
import("./entry.js").catch((e) =>
  console.error("Error importing `entry.js`:", e)
);

if (module.hot) {
  // Disable HMR in development
  module.hot.decline();
}
