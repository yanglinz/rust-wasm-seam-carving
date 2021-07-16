import CanvasImage from "./CanvasImage";

function DisplayImages(props) {
  return (
    <div className="border-8 border-gray-600 border-opacity-5">
      <CanvasImage src={props.src} />
    </div>
  );
}

export default DisplayImages;
