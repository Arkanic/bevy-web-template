const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: "./web/index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js"
    },
    plugins: [
        new HtmlWebpackPlugin({
            filename: "index.html",
            template: path.resolve(__dirname, "web/index.html")
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "rs")
        }),
        new webpack.ProvidePlugin({
            TextDecoder: ["text-encoding", "TextDecoder"],
            TextEncoder: ["text-encoding", "TextEncoder"]
        })
    ],
    mode: "development",
    experiments: {
        asyncWebAssembly: true
    }
}