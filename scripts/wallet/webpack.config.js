const path = require('path');
const webpack = require('webpack');

module.exports = {
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: 'ts-loader',
      },
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
        },
      },
    ],
  },
  resolve: {
    alias: {
      // Ensure React is deduplicated to prevent multiple React instances
      'react': path.resolve(__dirname, 'node_modules/react'),
      'react-dom': path.resolve(__dirname, 'node_modules/react-dom'),
    },
    extensions: ['.tsx', '.ts', '.js'],
    fallback: {
      crypto: require.resolve('crypto-browserify'),
      stream: require.resolve('stream-browserify'),
      vm: require.resolve('vm-browserify'),
    },
  },
  output: {
    filename: "wallet.js"
  },
  optimization: {
    splitChunks: false,
    runtimeChunk: false,
  },
  plugins: [
    new webpack.optimize.LimitChunkCountPlugin({
      maxChunks: 1,
    }),
  ],
  externals: {
    '@solana/kit': 'SolanaVendor["solana-kit.global"]',
    '@solana-program/memo': 'SolanaVendor["spl-memo.global"]',
    '@solana-program/system': 'SolanaVendor["spl-system.global"]',
    '@solana-program/token': 'SolanaVendor["spl-token.global"]',
  }
};