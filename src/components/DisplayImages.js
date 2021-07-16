import CanvasImage from "./CanvasImage";

function DisplayImages(props) {
  return (
    <div>
      <CanvasImage src={props.src} />
    </div>
  );
}

export default DisplayImages;
