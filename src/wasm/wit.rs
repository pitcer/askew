use wasmtime::component;

component::bindgen!({
    path: "wit/askew.wit",
    async: true,
});
