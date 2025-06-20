const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js', // changed from index.js to avoid conflicts
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
    ],
    mode: process.env.NODE_ENV || 'development',
    experiments: {
        asyncWebAssembly: true
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: 'webassembly/async'
            }
        ]
    },
    resolve: {
        extensions: ['.js', '.wasm']
    }
};