module.exports = {
    entry: __dirname + "/src/main.js",
    output: {
        path: __dirname + "/dist",
        filename: "main.js"
    },
    module: {
        loaders: [
            {
                test: /\.js[x]?$/,
                exclude: /node_modules/,
                loader: "babel",
                query: {
                    presets: ['react', 'es2015']
                }
            },
            { test: /\.css$/, loader: "style-loader!css-loader" },
            { test: /\.json$/, loader: 'json-loader' }
        ],
    }
};
