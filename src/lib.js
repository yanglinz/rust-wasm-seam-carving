export function resizeImage(canvasElements, resizedWidth, resizedHeight) {
  const { source, target } = canvasElements;

  function wasmResize(...args) {
    import("./pkg")
      .then((module) => module.resize(...args))
      .catch(console.error);
  }

  // Instead of overwriting the original canvas data, we'll transfer the
  // resized image data onto a second, blank canvas.
  const ctxSource = source.getContext("2d");
  const ctxTarget = target.getContext("2d");

  // TODO: Use resizedWidth and resizedHeight
  wasmResize(
    ctxSource,
    ctxTarget,
    resized.width,
    resized.height,
    resized.width - 1,
    resized.height
  );
}
