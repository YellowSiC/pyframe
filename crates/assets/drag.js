function detectOS() {
  const platform = navigator.platform.toLowerCase();
  if (platform.includes('mac')) return 'macos';
  if (platform.includes('win')) return 'windows';
  if (platform.includes('linux')) return 'linux';
  return 'unknown';
}

function invoke_frameless(id) {
  if (!window.ipc || typeof window.ipc.postMessage !== 'function') {
    console.warn('IPC nicht verfügbar – keine Aktionen ausgeführt.');
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

  // OS-spezifische mousedown / mouseup-Handler (macOS)
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
      // Wenn nicht Drag-Region → Koordinaten senden
      window.ipc.postMessage(`mousedown:${event.clientX},${event.clientY}`);
    }
  }

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

  // ========== NEU: Die „normalen“ Events ==========
  // Bewegung der Maus – nur Positionsdaten
  document.addEventListener('mousemove', (e) => {
    window.ipc.postMessage(`mousemove:${e.clientX},${e.clientY}`);
  });

  // Mousedown: unser spezieller Handler
  document.addEventListener('mousedown', handleMouseDown);

  // Touchstart – Drag nur für Touch
  document.addEventListener('touchstart', (e) => {
    if (e.target.hasAttribute(DRAG_REGION_ATTR)) {
      window.ipc.postMessage('drag_window');
    }
  });

  // Mouseup nur für macOS (Double-Click prüfen)
  if (osName === 'macos') {
    document.addEventListener('mouseup', handleMouseUp);
  }
}
