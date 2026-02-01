import init, { Idn } from "/idn/pkg/idn.js";

async function init_worker() {
    // buffer incoming messages while wasm is initializing
    let msgQueue = [];
    self.onmessage = (evt) => {
            msgQueue.push(evt);
    };

    // initialize wasm
    await init();

    var idn = await Idn.new();

    // set callback to handle messages passed to the worker
    self.onmessage = async (evt) => {
        console.log(evt.data);
        try {
            await idn.add_sentences(evt.data);
        } catch(e) {
            console.error(e);
        }
    };

    ////// flush the message buffer
    for (const evt of msgQueue) {
        self.onmessage(evt);
    }

    msgQueue = null;
}

init_worker();
