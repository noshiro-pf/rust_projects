'use strict';

/** @typedef { import("webpack").Configuration } Configuration */

const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');

require('webpack-dev-server');

/** @type {Configuration} */
const config = {
  entry: './bootstrap.ts',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bootstrap.js',
  },
  mode: 'development',
  module: {
    rules: [
      {
        test: /\.ts$/u,
        exclude: [/node_modules/u],
        use: {
          loader: 'ts-loader',
          options: {
            configFile: path.resolve(__dirname, './tsconfig.json'),
          },
        },
      },
    ],
  },
  plugins: [new CopyWebpackPlugin({ patterns: ['index.html'] })],
  resolve: {
    extensions: ['.ts', '.js'],
  },
  devServer: {
    open: false,
  },
  experiments: {
    asyncWebAssembly: true,
  },
};

module.exports = config;
