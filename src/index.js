// Make a note about how we need module to be async
import("./entry.js").catch((e) =>
  console.error("Error importing `entry.js`:", e)
);

if (module.hot) {
  // Disable HMR in development
  module.hot.decline();
}
