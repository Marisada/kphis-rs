import init from './drg_worker.js'
init({url:'drg_worker_bg.wasm'}).catch(console.error)