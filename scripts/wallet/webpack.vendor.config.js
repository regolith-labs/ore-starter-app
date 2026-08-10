// scripts/wallet/webpack.vendor.config.js
const path = require('path');

module.exports = {
    mode: 'production',
    entry: {
        'solana-kit.global': '@solana/kit',
        'spl-memo.global': '@solana-program/memo',
        'spl-system.global': '@solana-program/system',
        'spl-token.global': '@solana-program/token',
    },
    output: {
        path: path.resolve(__dirname, 'dist/vendor'),
        filename: '[name].js',
        library: ['SolanaVendor', '[name]'],
        libraryTarget: 'umd',
        globalObject: 'self', // works in window or worker
    },
};