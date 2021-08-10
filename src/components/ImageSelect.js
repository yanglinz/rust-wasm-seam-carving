import ReactDOM from "react-dom";
import Button from "./Button";
import { getDemoImages } from "../helpers/unsplash";

function getModalTargetEl(id = "dialog-mount") {
  let el = document.getElementById(id);
  if (!el) {
    el = document.createElement("div");
    el.id = id;
    document.body.appendChild(el);
  }
  return el;
}

function DialogPortal(props) {
  return ReactDOM.createPortal(props.children, getModalTargetEl());
}

function Dialog(props) {
  return (
    <DialogPortal>
      <div
        className="fixed z-10 inset-0 overflow-y-auto"
        role="dialog"
        aria-modal="true"
      >
        <div className="flex items-end justify-center pb-20 pt-4 px-4 min-h-screen text-center sm:block sm:p-0">
          <div
            className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
            aria-hidden="true"
          ></div>

          <span
            className="hidden sm:inline-block sm:align-middle sm:h-screen"
            aria-hidden="true"
          >
            &#8203;
          </span>

          <div className="inline-block align-bottom text-left bg-white rounded-sm shadow-xl overflow-hidden transform transition-all sm:align-middle sm:my-8 sm:w-full sm:max-w-lg">
            {props.children}
          </div>
        </div>
      </div>
    </DialogPortal>
  );
}

function ImageSelect(props) {
  const { globalState, handleImageSelect, handleImageUpload, handleClose } =
    props;

  if (globalState.control.state !== "IMAGE_SELECT") {
    return null;
  }

  const demoImages = getDemoImages();
  return (
    <Dialog>
      <div className="pb-4 pt-5 px-4 bg-white sm:p-6 sm:pb-4">
        <div className="flex flex-col content-center items-center">
          <h3 className="my-3 text-gray-900 text-lg font-medium leading-6">
            Select or Upload Image
          </h3>

          <div className="grid gap-1 grid-cols-4 overflow-hidden">
            {Object.keys(demoImages).map((key, index) => {
              const i = demoImages[key];
              return (
                <img
                  className="inline-block w-20 h-20"
                  onClick={() => handleImageSelect(i.url)}
                  alt={i.alt}
                  src={i.previewUrl}
                ></img>
              );
            })}
          </div>

          <div className="my-3">
            <input
              className="inline-block"
              type="file"
              accept="image/png, image/jpeg"
              onChange={handleImageUpload}
            />
          </div>
        </div>
      </div>

      <div className="px-4 py-3 bg-gray-50 sm:flex sm:flex-row-reverse sm:px-6">
        <Button intent="SECONDARY" onClick={handleClose}>
          Cancel
        </Button>
      </div>
    </Dialog>
  );
}

export default ImageSelect;
