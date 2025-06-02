function uid() {
    return window.crypto.getRandomValues(new Uint32Array(1))[0];
}

function transformCallback(callback, once) {
    const identifier = uid();
    const prop = `_${identifier}`;
    Object.defineProperty(window, prop, {
        value: (result) => {
            if (once) Reflect.deleteProperty(window, prop);
            return callback && callback(result);
        },
        writable: false,
        configurable: true
    });
    return identifier;
}

function invoke(cmd, args = {}) {
    return new Promise((resolve, reject) => {
        if (!window.socket || !window.socket.connected) {
            reject(new Error("Socket ist nicht verbunden oder nicht verfügbar!"));
            return;
        }

        const result_id = transformCallback((result) => resolve(result), true);
        const error_id = transformCallback((error) => reject(error), true);

        const message = {
            protocol: "pyinvoker",
            payload: {
                cmd,
                result_id,
                error_id,
                payload: args
            }
        };

        window.socket.emit("python:api", message);
    });
}


function initializeSocketHandlers() {
    // Antwort vom Backend auf Fenster-Requests
    window.socket.on("rust:api", async (data) => {
        const { id, method, args } = data;
        try {
            
            const argList = args ? Object.values(args) : [];
            const result = await PyFrame.call(method, argList);
            
            const response = { id };
            if (result !== null && result !== undefined) {
                response.result = result;
            }


            const message = {
                protocol: "rust:result:api",
                payload: response
            };
     
            window.socket.emit("python:api", message);

        } catch (error) {
            const errorStr = String(error);
            console.log(`The error from Rust: ${errorStr}`);

            let errorMessage = "Unknown error";
            const parts = errorStr.split(",");
            if (parts.length >= 3 && parts[1].trim() === "-1") {
                errorMessage = parts.slice(2).join(",").trim();
            } else {
                errorMessage = errorStr;
            }
            const message = {
                protocol: "rust:result:api",
                payload: {
                    id,
                    error: errorMessage
                }
            };
            window.socket.emit("python:api", message);

        }
    });

    window.socket.on("pyinvoke:result", ({ id, result }) => {
        const prop = `_${id}`;
        if (window[prop]) {
            window[prop](result);
        }
    });

    window.socket.on("pyinvoke:error", ({ id, error }) => {
        const prop = `_${id}`;
        if (window[prop]) {
            window[prop](error);
        }
    });

/*     PyFrame.addEventListener("*", function (data) {
        window.socket.emit("window_eventloop", { event: data });
    }); */


    window.addEventListener("message", (event) => {
        window.socket.emit("python:api",event.data);
    });
}

// Warte auf `window.socket` und seine Verbindung
function waitForSocketAndInitialize() {
    if (window.socket && window.socket.connected) {
        initializeSocketHandlers();
    } else {
        const interval = setInterval(() => {
            if (window.socket && window.socket.connected) {
                clearInterval(interval);
                initializeSocketHandlers();
            }
        }, 100); // prüfe alle 100ms
    }
}

// Startversuch (z. B. am Ende des Scripts)
waitForSocketAndInitialize();


