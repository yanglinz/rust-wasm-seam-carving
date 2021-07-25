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

function DialogInner(props) {
  return ReactDOM.createPortal(props.children, getModalTargetEl());
}

function Dialog(props) {
  return (
    <DialogInner>
      <div
        className="fixed z-10 inset-0 overflow-y-auto"
        aria-labelledby="modal-title"
        role="dialog"
        aria-modal="true"
      >
        <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
          {props.children}
        </div>
      </div>
    </DialogInner>
  );
}

function ImageSelect() {
  return (
    <Dialog>
      <h1>Foo Bar</h1>
    </Dialog>
  );
}

export default ImageSelect;
