(function(global){
    global.rust = global.rust || {};
    global.rust.wasm = global.rust.wasm || {};

    function CanvasRenderingContext2D (context2D, wasmPath, eventHandlers) {
        this.context2D = context2D;
        this.wasmPath = wasmPath;
        let handlers = eventHandlers || {};

        this.eventHandlers = { 
            onInit: handlers.onInit || ((result) => {}),
            onResize: handlers.onResize ||  ((result) => {}),
            onAnimate: handlers.onAnimate ||  ((result) => {})
        };
        this.init = function() {        
            let myContext = this.context2D;
            let myHandlers = this.eventHandlers;
            let imports = {
                env:{ 
                    // property getters/setters         
                    js_get_canvas_height: () => myContext.canvas.height,
                    js_get_canvas_width: () => myContext.canvas.width,           
                    js_set_fill_style_rgba: (r, g, b, a) => {
                        const red = r.toString(16).padStart(2, '0');
                        const green = g.toString(16).padStart(2, '0');
                        const blue = b.toString(16).padStart(2, '0');
                        const alpha = a.toString(16).padStart(2, '0');
                        myContext.fillStyle = "#"+red+green+blue+alpha;
                    },
                    js_get_line_cap: () => { 
                        const capString = myContext.lineCap.toLower(); 
                        switch(capString){
                            case "butt": return 1;
                            case "round": return 2;
                            case "square": return 3;
                        }
                        return 1;
                    },
                    js_set_line_cap: (value) => {
                        let capVal = "butt";
                        switch(value){
                            case 1: capVal = "butt"; break;
                            case 2: capVal = "round"; break;
                            case 3: capVal = "square"; break;
                        }
                        myContext.lineCap = capVal;
                    },
                    js_get_line_join: () => { 
                        const joinString = myContext.lineJoin.toLower(); 
                        switch(joinString){
                            case "bevel": return 1;
                            case "round": return 2;
                            case "miter": return 3;
                        }
                        return 1; 
                    },
                    js_set_line_join: (value) => {
                        let joinVal = "bevel";
                        switch(value){
                            case 1: joinVal = "bevel"; break;
                            case 2: joinVal = "round"; break;
                            case 3: joinVal = "miter"; break;
                        }
                        myContext.lineJoin = joinVal;
                    },
                    js_get_line_width: () => { return myContext.lineWidth; },
                    js_set_line_width: (value) => {myContext.lineWidth = value; },
                    js_set_stroke_style_rgba: (r, g, b, a) => {
                        const red = r.toString(16).padStart(2, '0');
                        const green = g.toString(16).padStart(2, '0');
                        const blue = b.toString(16).padStart(2, '0');
                        const alpha = a.toString(16).padStart(2, '0');
                        myContext.strokeStyle = "#"+red+green+blue+alpha;
                    },
                    
                    // functions
                    js_arc: (x, y, radius, start_angle, end_angle, counter_clockwise) =>{
                        myContext.arc(x,y,radius,start_angle,end_angle,counter_clockwise);
                    },
                    js_arc_to: (x1, y1, x2, y2, radius) =>{
                        myContext.arcTo(x1,y1,x2,y2,radius);
                    },
                    js_begin_path: () => {
                        myContext.beginPath();
                    },
                    js_clear_rect: (x,y,width,height) => {
                        myContext.clearRect(x,y,width, height);
                    },
                    js_close_path: () => {
                        myContext.closePath();
                    },
                    js_fill: () => {
                        myContext.fill();
                    },
                    js_fill_rect: (x,y,width,height) => {
                        myContext.fillRect(x,y,width, height);
                    },
                    js_line_to: (x, y) =>{
                        myContext.lineTo(x,y);
                    },
                    js_move_to: (x, y) =>{
                        myContext.moveTo(x,y);
                    },
                    js_stroke: () => {
                        myContext.stroke();
                    }
                }
            };

            
    
            WebAssembly.instantiateStreaming(fetch(this.wasmPath), imports)
            .then((result) => {
                //call init handler
                myHandlers.onInit(result); 

                //add Resize Event Handler
                if(global.addEventListener){
                    global.addEventListener('resize', () => {
                        setDimensions(myContext, myContext.canvas);
                        myHandlers.onResize(result);  
                    }, true);
                }

                //add animation handler -- this may slow stuff down because its looping animation and doing a no-op... is that alright?
                if(global.requestAnimationFrame){
                    function animate(){
                        myHandlers.onAnimate(result);
                        global.requestAnimationFrame(animate);
                    }
                    global.requestAnimationFrame(animate);
                }
            });  
        };
        return this;
    };

    let _manager = (function(){
        let contexts = {};
        let contextsId = 0;
        
        const addContext = (id, wasmPath, initCallback) => {            
            let canvas = global.document.getElementById(id);
            let context2D = canvas.getContext("2d");
            setDimensions(context2D, canvas);
            let contextObj = new global.rust.wasm.CanvasRenderingContext2D(context2D, wasmPath, initCallback);
            let newId = contextsId;
            contexts[newId] = contextObj;
            contextsId++;           

            return contextObj;
        };

        return {
            addContext: addContext
        }
    }());

    

    const setDimensions = (context, canvas) => {            
        context.canvas.width = canvas.clientWidth;
        context.canvas.height = canvas.clientHeight;
    };

    global.rust.wasm.canvasRenderingContext2DManager = _manager;
    global.rust.wasm.CanvasRenderingContext2D = CanvasRenderingContext2D;

})(window);