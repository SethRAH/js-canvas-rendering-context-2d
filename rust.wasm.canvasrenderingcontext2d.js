(function(global){
    global.rust = global.rust || {};
    global.rust.wasm = global.rust.wasm || {};

    function CanvasRenderingContext2D (context2D, wasmPath) {
        this.context2D = context2D;
        this.wasmPath = wasmPath;
        this.init = function() {        
            let myContext = this.context2D;
            let imports = {
                env:{ 
                    // property getters/setters
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
                    js_begin_path: () =>{
                        myContext.beginPath();
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
            .then( result => {
                result.instance.exports.main();
            });  
        };
        return this;
    };

    let _manager = (function(){
        let contexts = {};
        let id = 0;
        
        const addContext = (id, wasmPath) => {            
            let canvas = global.document.getElementById(id);
            let context2D = canvas.getContext("2d");
            context2D.canvas.width = canvas.clientWidth;
            context2D.canvas.height = canvas.clientHeight;
            let contextObj = new global.rust.wasm.CanvasRenderingContext2D(context2D, wasmPath);
            let newId = id;
            contexts[newId] = contextObj;
            id++;
            return contextObj;
        };

        return {
            addContext: addContext
        }
    }());


    global.rust.wasm.canvasRenderingContext2DManager = _manager;
    global.rust.wasm.CanvasRenderingContext2D = CanvasRenderingContext2D;

})(window);