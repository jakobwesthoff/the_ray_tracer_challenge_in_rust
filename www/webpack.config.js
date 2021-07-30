// const CopyWebpackPlugin = require("copy-webpack-plugin");
// const path = require("path");

// module.exports = {
//   entry: "./bootstrap.js",
//   output: {
//     path: path.resolve(__dirname, "dist"),
//     filename: "bootstrap.js",
//   },
//   mode: "development",
//   plugins: [new CopyWebpackPlugin(["index.html"])],
//   module: {
//     rules: [
//       {
//         test: /\.worker\.js$/,
//         use: { loader: "worker-loader" },
//       },
//     ],
//   },
// };
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

const dist = path.resolve(__dirname, "dist");

const appConfig = {
  entry: "./bootstrap.js",
  output: {
    path: dist,
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [new CopyWebpackPlugin(["index.html"])],
  module: {
    rules: [
      {
        test: /\.worker\.js$/,
        use: { loader: "worker-loader" },
      },
    ],
  },
};

const workerConfig = {
  entry: "./render.worker.js",
  target: "webworker",
  resolve: {
    extensions: [".js", ".wasm"]
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../")
    })
  ],
  output: {
    path: dist,
    filename: "worker.js"
  }
};

module.exports = [appConfig, workerConfig];
