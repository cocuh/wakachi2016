var gulp = require('gulp');
var webpack = require('webpack-stream');
var webpackConfig = require('./webpack.config.js');
var WebpackDevServer = require("webpack-dev-server");
var watch = require('gulp-watch');

gulp.task('default', ['build']);
gulp.task('build', ['webpack', 'html', 'semantic']);
gulp.task('webpack', function () {
    return gulp.src('src/main.js')
        .pipe(webpack(webpackConfig))
        .pipe(gulp.dest('dist/'));
});

gulp.task('semantic', function () {
    return gulp.src('semantic/*')
        .pipe(gulp.dest('dist/semantic'));
});

gulp.task('html', function () {
    return gulp.src('src/index.html')
        .pipe(gulp.dest('dist/'));
});

gulp.task('watch', function () {
    return watch('src/**/*', ['webpack'], function () {
        gulp.start('build');
    });
});
