# JS-CANVAS-RENDERING-CONTEXT-2D
*Wrapper for CanvasRenderingContext2D*

This library is meant to be a zero dependency wrapper for the [CanvasRenderingContext2D web api](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D) for use in WASM rust applications.

Rust programs using this library can bind to a single canvas element in the DOM and use functions in the library to control that canvas's context.

Also included in this repo is the file `rust.wasm.canvasrenderingcontext2d.js`. To use this library on a web page, load the .js file on the page and then call 
```javascript
// reference the context manager for ease of calls. You will have one manager per webpage. It can contain contexts for multiple canvases on that page.
let manager = rust.wasm.canvasRenderingContext2DManager;

// callback that is called once the canvas element is bound to the wasm program
let initCallback = function(result) {
    // you probably want to call the rust program's main function here 
    result.instance.exports.main();
};

// callback that is called on window resize
let resizeCallback = function(result) {
    // you may want to redraw when the window is resized
    result.instance.export.draw();
}

// callback that is called on the browsers' request animation frame
let animateCallback = function(result) {
    // you may want to tick the program and re-draw here
    result.instance.export.tick();
    result.instance.export.draw();
}

let callbacks = {
    onInit: initCallback,
    onResize: resizeCallback,
    onAnimate: animateCallback
};

// create a context from the manager
let localContext = manager.addContext(canvasId, wasmPath, callbacks);

// initing the context will bind the canvas element to the wasm program and then call the callback
localContext.init();
```

## Running the example included in this repo
The example included in this repo contains a website with several canvases all bound to rust/wasm programs that use this library to interact with the canvases. Once you grab the repo locally, you'll need to do the following to get it running: 
* Add wasm32-unknown-unknown as a build target
    * call `rustup target add wasm32-unknown-unknown`
* build
    * call `cargo build --example * --target wasm32-unknown-unknown`
* Use some sort of local web server to view the example
    * For this, I like to use Ian Kettlewell's `devserver`. It props up a local web server that listens to changes in the directory and pushes them out. 
    * You can install devserver by running `cargo install devserver` from your command line
    * To run it, open a new command line terminal and navigate to the repo's directory. Then run `devserver` on that terminal. You can press ctrl+c to exit out of the server
    * Once a local server is running, go to localhost:8080 in any web browser and you should see the examples.