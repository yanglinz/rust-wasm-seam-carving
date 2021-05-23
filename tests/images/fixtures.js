const fs = require("fs");
const path = require("path");
const sharp = require("sharp");

function isImageFile(f) {
  return f.name.endsWith(".jpg");
}

function getImageFixtureData(f) {
  const imageFilePath = path.join(__dirname, f.name);
  const image = sharp(imageFilePath);

  const imageMetadata = image.metadata();
  const imageData = image.toBuffer().then((b) => Uint8ClampedArray.from(b));
  return Promise.all([imageMetadata, imageData]);
}

function writeImageFixtureData(f, imageMetadata, imageData) {
  const { width, height } = imageMetadata;
  const data = { width, height, data: imageData };
  const outFile = path.join(__dirname, f.name + ".json");
  fs.writeFile(outFile, JSON.stringify(data, null, 2), (err) => {
    if (err) {
      throw err;
    }
  });
}

// Generate test fixtures from sample images
fs.readdirSync(__dirname, { withFileTypes: true }).forEach((file) => {
  if (isImageFile(file)) {
    getImageFixtureData(file).then(([imageMetadata, imageData]) => {
      writeImageFixtureData(file, imageMetadata, imageData);
    });
  }
});
