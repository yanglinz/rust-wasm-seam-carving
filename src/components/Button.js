function Button(props) {
  const { children, intent, ...rest } = props;

  let className =
    "rounded-md drop-shadow	px-5 py-3 text-base font-large focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50";
  if (intent === "PRIMARY") {
    className +=
      " border border-gray-300 bg-blue-500 text-white hover:bg-blue-600 focus:ring-blue-500";
  }
  if (intent === "SECONDARY") {
    className +=
      " border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:ring-blue-500";
  }

  return (
    <button className={className} {...rest}>
      {children}
    </button>
  );
}

export default Button;
