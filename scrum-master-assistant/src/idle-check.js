let lastActivity = Date.now();
let promptTimeout, punchOutTimeout;

function resetTimer() {
  lastActivity = Date.now();
  clearTimeout(promptTimeout);
  clearTimeout(punchOutTimeout);

  promptTimeout = setTimeout(() => {
    window.showPrompt(); // Custom function to show popup
    punchOutTimeout = setTimeout(() => {
      window.punchOut(); // Auto punch-out
    }, 5 * 60 * 1000); // 5 mins
  }, 10 * 60 * 1000); // 10 mins
}

['mousemove', 'keydown', 'mousedown'].forEach(event =>
  document.addEventListener(event, resetTimer)
);

resetTimer(); // Start initially
