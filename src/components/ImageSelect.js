import ReactDOM from "react-dom";

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
        <div className="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
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

          <div className="inline-block align-bottom bg-white rounded-sm text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full">
            {props.children}
          </div>
        </div>
      </div>
    </DialogPortal>
  );
}

function ImageSelect() {
  const demoImages = [
    "https://source.unsplash.com/yRjLihK35Yw/200x150",
    "https://source.unsplash.com/random/200x150",
    "https://source.unsplash.com/random/200x150",
    "https://source.unsplash.com/random/200x150",
    "https://source.unsplash.com/random/200x150",
    "https://source.unsplash.com/random/200x150",
    "https://source.unsplash.com/random/200x150",
    "https://source.unsplash.com/random/200x150",
  ];

  return (
    <Dialog>
      <div className="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
        <div className="flex flex-col content-center items-center">
          <h3 className="my-3 text-lg leading-6 font-medium text-gray-900">
            Select Image
          </h3>

          <div className="overflow-hidden grid grid-cols-4 gap-1">
            {demoImages.map((url) => (
              <img
                className="inline-block h-20 w-20 rounded-full"
                src={url}
              ></img>
            ))}
          </div>
        </div>
      </div>

      <div className="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
        <button
          type="button"
          className="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
        >
          Cancel
        </button>
      </div>
    </Dialog>
  );
}

export default ImageSelect;
