import init, { hello } from "wasm_game";

init().then(() => {
    console.log("ok!!!!");
    hello("test");
})