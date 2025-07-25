const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./index.js", // Change this to your JS entry point if different
  output: {
    path: __dirname + "/dist",
    filename: "bundle.js",
  },
  mode: "development", // Or "production"
  experiments: {
    asyncWebAssembly: true, // Enable WebAssembly support
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async", // Add rule for WebAssembly files
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "./index.html", // Change this if your HTML entry is elsewhere
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      extraArgs: "--target bundler",
    }),
  ],
  resolve: {
    extensions: [".js", ".wasm"], // Add .wasm extension for resolving files
  },
  devServer: {
    static: "./dist",
    hot: true,
  },
};