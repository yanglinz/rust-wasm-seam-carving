import React, { useReducer, useEffect } from "react";
import { produce } from "immer";
import ReactDOM from "react-dom";

import { SeamCarver, wasm_memory as memory } from "./pkg";
import { onDocumentReady } from "./helpers/dom";
import { getDemoImages } from "./helpers/unsplash";
import ImageSelect from "./components/ImageSelect";
import ImageCanvas, { getCanvasElements } from "./components/ImageCanvas";
import Controls from "./components/Controls";

import "./index.css";

// Most of the boilerplate below are just React UI state tracking;
// not particularly relevant to the Seam Carving algorithm.

const initialAppState = {
  selectedImage: {
    state: "INITIAL",
    width: 0,
    height: 0,
  },
  control: {
    state: "INITIAL",
  },
};

const appStateModifiers = {
  IMAGE_LOADED: function imageLoaded(state, action) {
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
  source.width = image.width;
  source.height = image.height;

  const ctx = source.getContext("2d");
  // prettier-ignore
  ctx.drawImage(image, 0, 0, image.width, image.height, 0, 0, source.width, source.height);
  // prettier-ignore
}

function canvasLoadExternalImage(dispatch, { imageUrl }) {
  const image = document.createElement("img");
  image.src = imageUrl;
  image.crossOrigin = "Anonymous";

  image.onload = function () {
    canvasLoadImage(dispatch, { image });
    dispatch({
      type: "IMAGE_LOADED",
      payload: { width: image.width, height: image.height, url: image.src },
    });
  };
}

function canvasUploadedImage(dispatch, { inputEvent }) {
  const file = inputEvent.target.files[0];
  const image = document.createElement("img");
  image.src = window.URL.createObjectURL(file);

  image.onload = function () {
    canvasLoadImage(dispatch, { image });
    dispatch({
      type: "IMAGE_LOADED",
      payload: { width: image.width, height: image.height, url: image.src },
    });
  };
}

function canvasResizeImage(dispatch, { resizedWidth }) {
  const { source, target } = getCanvasElements();

  // SeamCarver module is bindings generated from wasm-pack, and corresponds to 
  // the implementation of the same name in src/lib.rs.
  const carver = SeamCarver.from_canvas(
    source.getContext("2d"),
    source.width,
    source.height
  );

  function draw() {
    // Generating a Uint8ClampedArray from the shared memory is a critial component
    // of how we can efficiently manipulate large vectors without paying the
    // serialization/deserialization cost on each iteration.
    const imageData = new Uint8ClampedArray(
      memory().buffer,
      carver.image_data_ptr(),
      carver.width * carver.height * 4
    );

    target.width = carver.width;
    target.height = carver.height;
    const imageDataWrapper = new ImageData(
      imageData,
      carver.width,
      carver.height
    );
    target.getContext("2d").putImageData(imageDataWrapper, 0, 0);
  }

  // Recursively delete seams until we reach our desized target size.
  // Not that for each iteration, we call draw in with the seam drawn
  // and once more with the seam removed to animate the actual seam.
  let steps = source.width - resizedWidth;
  function incrementalResize() {
    if (steps <= 0) {
      return;
    }

    carver.mark_seam();
    draw();

    requestAnimationFrame(() => {
      carver.delete_seam();
      draw();
      steps -= 1;

      requestAnimationFrame(incrementalResize);
    });
  }

  requestAnimationFrame(incrementalResize);

  dispatch({ type: "RESIZE_INITIALIZED" });
}

function App() {
  const [state, dispatch] = useReducer(appStateReducer, initialAppState);

  useEffect(
    () =>
      canvasLoadExternalImage(dispatch, {
        imageUrl: getDemoImages()["photo-1575252663512-b25714ec27f9"].url,
      }),
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
        handleImageUpload={(e) =>
          canvasUploadedImage(dispatch, { inputEvent: e })
        }
        handleClose={() => dispatch({ type: "IMAGE_SELECT_CLOSED" })}
      />
    </div>
  );
}

onDocumentReady(() => {
  ReactDOM.render(<App />, document.getElementById("app"));
});
