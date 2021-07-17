function init(canvasElements) {
  self.canvasElements = canvasElements;
}

function loadSourceImage(canvas) {
  console.log("loadSourceImage");
}

function resizeTargetImage() {
  console.log("resizeTargetImage");
}

self.onmessage = function handleMessage(message) {
  const handlersByName = {
    init,
    loadSourceImage,
    resizeImage,
  };

  const name = Array.isArray(message.data) ? message.data[0] : undefined;
  const handler = handlersByName[name];

  if (handler) {
    const [_, ...args] = message.data;
    handler(...args);
  }
};
