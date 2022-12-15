async function run() {
    const importObject = {
        console: {
            log: () => {
                console.log("log info!");
            },
            error: () => {
                console.log("error info");
            }
        }
    }

    const response = await fetch("test1.wasm");
    const buffer = await response.arrayBuffer();
    debugger;
    const wasm = await WebAssembly.instantiate(buffer, importObject);

    const addFunction = wasm.instance.exports.add;
    const result = addFunction(10, 20);

    console.log(result);
}

run();