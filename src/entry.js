import React, { useReducer, useEffect } from "react";
import produce from "immer";
import ReactDOM from "react-dom";

import { SeamCarver, wasm_memory as memory } from "./pkg";
import { onDocumentReady } from "./helpers/dom";
import ImageSelect from "./components/ImageSelect";
import ImageCanvas, { getCanvasElements } from "./components/ImageCanvas";
import Controls from "./components/Controls";

import "./index.css";

const initialAppState = {
  selectedImage: {
    state: "INITIAL",
    url: null,
    width: 0,
    height: 0,
  },
  control: {
    state: "INITIAL",
  },
};

const appStateModifiers = {
  SOURCE_IMAGE_LOADED: function sourceImageLoaded(state, action) {
    const { width, height, url } = action.payload;
    state.selectedImage.state = "SOURCE";
    state.selectedImage.width = width;
    state.selectedImage.height = height;
    state.selectedImage.url = url;
  },
  RESIZE_INITIALIZED: function resizeInitialized(state, action) {
    state.selectedImage.state = "TARGET";
  },
  IMAGE_SELECT_OPENED: function imageSelectOpened(state, action) {
    state.control.state = "IMAGE_SELECT";
  },
  IMAGE_SELECT_CLOSED: function imageSelectClosed(state, action) {
    state.control.state = "INITIAL";
  },
};

function appStateReducer(state, action) {
  const modifer = appStateModifiers[action.type];
  if (!modifer) {
    throw new Error("Unknown action type in reducer.");
  }
  return produce(state, (draftState) => modifer(draftState, action));
}

function canvasLoadImage(dispatch, { image }) {
  const { source } = getCanvasElements();

  // TODO: Handle high DPI screens / scale down heuristics
  source.width = image.width;
  source.height = image.height;

  const ctx = source.getContext("2d");
  // prettier-ignore
  ctx.drawImage(image, 0, 0, image.width, image.height, 0, 0, source.width, source.height);
  // prettier-ignore
}

function canvasLoadExternalImage(dispatch, { imageUrl }) {
  const { source } = getCanvasElements();

  return fetch(imageUrl)
    .then((r) => r.blob())
    .then((blob) => createImageBitmap(blob))
    .then((image) => canvasLoadImage(dispatch, { image }))
    .then(() => {
      dispatch({
        type: "SOURCE_IMAGE_LOADED",
        payload: {
          width: source.width,
          height: source.height,
          url: imageUrl,
        },
      });
    });
}

function canvasUploadedImage(dispatch) {
  // TODO
}

function canvasResizeImage(dispatch, { resizedWidth }) {
  const { source, target } = getCanvasElements();

  const carver = SeamCarver.from_canvas(
    source.getContext("2d"),
    source.width,
    source.height
  );

  function drawCurrent() {
    // Get the image data
    const imageDataPtr = carver.image_data_ptr();
    const imageData = new Uint8ClampedArray(
      memory().buffer,
      imageDataPtr,
      carver.width * carver.height * 4
    );

    // Draw the image data
    target.width = carver.width;
    target.height = carver.height;
    const imageDataWrapper = new ImageData(
      imageData,
      carver.width,
      carver.height
    );
    target.getContext("2d").putImageData(imageDataWrapper, 0, 0);
  }

  let steps = source.width - resizedWidth;
  function incrementalResize() {
    if (steps <= 0) {
      return;
    }

    carver.mark_seam();
    drawCurrent();

    requestAnimationFrame(() => {
      carver.delete_seam();
      drawCurrent();
      steps -= 1;

      requestAnimationFrame(incrementalResize);
    });
  }

  requestAnimationFrame(incrementalResize);

  dispatch({ type: "RESIZE_INITIALIZED" });
}

function App() {
  const [state, dispatch] = useReducer(appStateReducer, initialAppState);

  const DEMO_IMAGE = "https://source.unsplash.com/yRjLihK35Yw/500x250";
  useEffect(
    () => canvasLoadExternalImage(dispatch, { imageUrl: DEMO_IMAGE }),
    []
  );

  return (
    <div className="App flex flex-col h-screen">
      <div className="flex-grow">
        <div className="items-center	justify-center flex h-full">
          <ImageCanvas globalState={state} />
        </div>
      </div>

      <div className="border-gray-150 p-10 bg-white border-t">
        <Controls
          globalState={state}
          handleResize={(resizedWidth) =>
            canvasResizeImage(dispatch, { resizedWidth })
          }
          handleOpenImageSelect={() =>
            dispatch({ type: "IMAGE_SELECT_OPENED" })
          }
        />
      </div>

      <ImageSelect
        globalState={state}
        handleImageSelect={(imageUrl) => {
          canvasLoadExternalImage(dispatch, { imageUrl });
          dispatch({ type: "IMAGE_SELECT_CLOSED" });
        }}
        handleImageUpload={(e) => {
          const file = e.target.files[0];
          const img = document.createElement("img");
          img.src = window.URL.createObjectURL(file);

          img.onload = function () {
            console.log(img.width);
            console.log(img.height);

            const { source } = getCanvasElements();
            source.width = img.width;
            source.height = img.height;
            const ctx = source.getContext("2d");
            // prettier-ignore
            ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, source.width, source.height);
          };
        }}
        handleClose={() => dispatch({ type: "IMAGE_SELECT_CLOSED" })}
      />
    </div>
  );
}

onDocumentReady(() => {
  ReactDOM.render(<App />, document.getElementById("app"));
});

if (module.hot) {
  // Disable HMR in development
  module.hot.decline();
}
