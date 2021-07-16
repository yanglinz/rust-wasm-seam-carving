function Controls(props) {
  const { handleResize } = props;
  return (
    <div className="Controls">
      <button
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        onClick={handleResize}
      >
        Resize to 50%
      </button>
    </div>
  );
}

export default Controls;
