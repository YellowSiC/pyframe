const button = document.getElementById('submitBtn');
const input = document.getElementById('myInput');
const output = document.getElementById('output');





submitBtn.addEventListener('click', async () => {
  const name = input.value;
try {
  if (name) {
    const greeting = await invoke("greet", { name });
    output.textContent = JSON.parse(greeting);
    output.classList.add('visible');
  } else {
    output.textContent = '';
    output.classList.remove('visible');
  }
  } catch (error) {
    output.textContent = "Fehler: " + error;
  }
});