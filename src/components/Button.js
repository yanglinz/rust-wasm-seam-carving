function Button(props) {
  const { children, intent, ...rest } = props;

  if (intent === "PRIMARY") {
    return (
      <button
        className="drop-shadow	px-5 font-large py-3 text-white text-base bg-blue-500 hover:bg-blue-600 border border-gray-300 rounded-md focus:outline-none disabled:opacity-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
        {...rest}
      >
        {children}
      </button>
    );
  }
  if (intent === "SECONDARY") {
    return (
      <button
        className="drop-shadow	px-5 font-large py-3 text-gray-700 text-base hover:bg-gray-50 bg-white border border-gray-300 rounded-md focus:outline-none disabled:opacity-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
        {...rest}
      >
        {children}
      </button>
    );
  }

  return null;
}

export default Button;
