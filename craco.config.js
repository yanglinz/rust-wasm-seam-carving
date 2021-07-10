const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  webpack: {
    configure: (webpackConfig) => {
      webpackConfig.resolve.extensions.push(".wasm");

      webpackConfig.module.rules.forEach((rule) => {
        (rule.oneOf || []).forEach((oneOf) => {
          if (oneOf.loader && oneOf.loader.indexOf("file-loader") >= 0) {
            // Make file-loader ignore WASM files
            oneOf.exclude.push(/\.wasm$/);
          }
        });
      });

      webpackConfig.plugins = (webpackConfig.plugins || []).concat([
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, "src"),
          extraArgs: "--no-typescript",
          outDir: path.resolve(__dirname, "src/pkg"),
        }),
      ]);

      return webpackConfig;
    },
  },

  style: {
    postcss: {
      plugins: [require("tailwindcss"), require("autoprefixer")],
    },
  },
};
