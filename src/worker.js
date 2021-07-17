function init(canvasElements) {
  self.canvasElements = canvasElements;
}

function loadSourceImage(imageUrl) {
  if (!self.canvasElements) {
    throw new Error("Unable load image on uninitialized source canvas");
  }

  function loadImage(url) {
    return fetch(url)
      .then((r) => r.blob())
      .then((blob) => createImageBitmap(blob));
  }

  loadImage(imageUrl).then((img) => {
    const { source } = self.canvasElements;

    // TODO: Handle high DPI screens / scale down heuristics
    source.width = img.width;
    source.height = img.height;

    const ctx = source.getContext("2d");
    // prettier-ignore
    ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, source.width, source.height);
    // prettier-ignore
    const imageData = ctx.getImageData(0, 0, source.width, source.height);
    console.log(imageData);
  });
}

function resizeTargetImage(
  sourceWidth,
  sourceHeight,
  targetWidth,
  targetHeight
) {
  console.log("resizeTargetImage", {
    sourceWidth,
    sourceHeight,
    targetWidth,
    targetHeight,
  });
}

self.onmessage = function handleMessage(message) {
  const handlersByName = {
    init,
    loadSourceImage,
    resizeTargetImage,
  };

  const name = Array.isArray(message.data) ? message.data[0] : undefined;
  const handler = handlersByName[name];

  if (handler) {
    const [_, ...args] = message.data;
    handler(...args);
  }
};
