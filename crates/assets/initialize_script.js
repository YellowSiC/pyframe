(function () {
  // Disable drag and open file
  window.addEventListener("dragover", (ev) => ev.preventDefault(), false);

  // OS-Erkennung
  function detectOS() {
    const platform = navigator.platform.toLowerCase();
    if (platform.includes('mac')) return 'macos';
    if (platform.includes('win')) return 'windows';
    if (platform.includes('linux')) return 'linux';
    return 'unknown';
  }

  // Frameless Drag & Events
  function initFrameless(id) {
    if (!window.ipc || typeof window.ipc.postMessage !== 'function') {
      console.warn('IPC nicht verfügbar  keine Aktionen ausgeführt.');
      return;
    }

    const osName = detectOS();
    const DRAG_REGION_ATTR = 'data-pyframe-darg-region';
    const element = document.getElementById(id);

    if (!element) {
      console.warn(`Element mit ID "${id}" nicht gefunden.`);
      return;
    }

    element.setAttribute(DRAG_REGION_ATTR, '');

    let initialMouseX = 0, initialMouseY = 0;

    // Mousedown-Handler
    function handleMouseDown(event) {
      if (
        event.target.hasAttribute(DRAG_REGION_ATTR) &&
        event.button === 0 &&
        (event.detail === 1 || event.detail === 2)
      ) {
        if (osName === 'macos' && event.detail === 2) {
          initialMouseX = event.clientX;
          initialMouseY = event.clientY;
          return;
        }

        event.preventDefault();
        event.stopImmediatePropagation();

        const command = event.detail === 2 ? 'maximize' : 'drag_window';
        window.ipc.postMessage(command);
      } else {
        window.ipc.postMessage(`mousedown:${event.clientX},${event.clientY}`);
      }
    }

    // Mouseup für macOS (Double-Click prüfen)
    function handleMouseUp(event) {
      if (
        event.target.hasAttribute(DRAG_REGION_ATTR) &&
        event.button === 0 &&
        event.detail === 2 &&
        event.clientX === initialMouseX &&
        event.clientY === initialMouseY
      ) {
        window.ipc.postMessage('drag_window');
      }
    }

    // Event-Listener für Bewegungen und Touch
    document.addEventListener('mousemove', (e) => {
      window.ipc.postMessage(`mousemove:${e.clientX},${e.clientY}`);
    });

    document.addEventListener('mousedown', handleMouseDown);

    document.addEventListener('touchstart', (e) => {
      if (e.target.hasAttribute(DRAG_REGION_ATTR)) {
        window.ipc.postMessage('drag_window');
      }
    });

    if (osName === 'macos') {
      document.addEventListener('mouseup', handleMouseUp);
    }
  }

  // === PyFrame-Objekt & Event-System ===
  var PyFrame = {};

  var eventListeners = {};

  PyFrame.addEventListener = function (event, listener) {
    if (!eventListeners[event]) eventListeners[event] = [];
    eventListeners[event].push(listener);
  };

  PyFrame.removeEventListener = function (event, listener) {
    if (!eventListeners[event]) return;
    eventListeners[event] = eventListeners[event].filter(l => l !== listener);
  };

  PyFrame.removeAllEventListeners = function (event) {
    if (!eventListeners[event]) return;
    eventListeners[event] = [];
  };

  PyFrame.__emit__ = function (event, data) {
    setTimeout(() => {
      const keys = [event, event.split('.')[0] + '.*', '*'];
      keys.forEach(key => {
        (eventListeners[key] || []).forEach(listener => listener(event, data));
      });
    }, 0);
  };

  // === PyFrame-API-Calls ===
  var callbacks = {};
  var getNextCallbackId = (() => {
    var callbackId = 0;
    return () => (++callbackId >= Number.MAX_SAFE_INTEGER) ? 0 : ++callbackId;
  })();

  PyFrame.call = function (method, args) {
    const callbackId = getNextCallbackId();
    window.ipc.postMessage(JSON.stringify([callbackId, method, args]));

    let _resolve, _reject;
    const promise = new Promise((resolve, reject) => {
      _resolve = resolve;
      _reject = reject;
    });
    promise.resolve = _resolve;
    promise.reject = _reject;

    callbacks[callbackId] = promise;
    return promise;
  };

  PyFrame.__resolve__ = function (response) {
    setTimeout(() => {
      const [callbackId, code, , data] = response;
      const promise = callbacks[callbackId];
      if (promise) {
        code === 0 ? promise.resolve(data) : promise.reject(response);
        delete callbacks[callbackId];
      }
    }, 0);
  };

  // Proxy-API
  if (typeof Proxy !== 'undefined') {
    PyFrame.api = new Proxy({}, {
      get: (_, namespace) => new Proxy({}, {
        get: (_, method) => (...args) => PyFrame.call(`${namespace}.${method}`, args)
      })
    });
  } else {
    console.log('Proxy not supported, please use PyFrame.call instead');
  }

  // IPC-Callback-Handler
  PyFrame.addEventListener('ipc.callback', (event, response) => {
    PyFrame.__resolve__(response);
  });

  // Fenster-APIs entfernen
  delete window.close;
  delete window.open;

  // Frameless-API als Teil von PyFrame verfügbar machen
  PyFrame.initFrameless = initFrameless;

  window.PyFrame = PyFrame;

  console.log('PyFrame (mit Frameless-API) geladen!');
})();
