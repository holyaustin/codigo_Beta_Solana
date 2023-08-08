const path = require("path");

module.exports = {
  entry: "./index.ts",
  mode: "production",
  devtool: "source-map",
  module: {
    rules: [
      {
        test: /\.ts?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  output: {
    filename: "budget-tracker.bundle.js",
    path: path.resolve(__dirname, "dist"),
    library: {
      type: "commonjs-static",
    },
  },
};
