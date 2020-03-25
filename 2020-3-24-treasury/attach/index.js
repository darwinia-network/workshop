// env
process.env.NODE_OPTIONS = '--experimental-repl-await';

const { ApiPromise, WsProvider } = require("@polkadot/api");
const customizeType = require("./types.json");

async function setPolkadotJs() {
    return await ApiPromise.create({
    	types: customizeType,
    	provder: new WsProvider("ws://0.0.0.0:9944"),
    }).catch(e => console.error);
}

// expose to repl
const api = setPolkadotJs();
