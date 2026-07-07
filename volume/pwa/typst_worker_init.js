import init from './typst_worker.js'
init({url:'typst_worker_bg.wasm'}).catch(console.error)