function togglePlay() {
  const audio = document.getElementById('audio');
  const button = document.getElementById('audioBtn');
  if (audio.paused) {
    audio.play();
    button.textContent = '';
  } else {
    audio.pause();
    button.textContent = '';
  }
}

function toggleTheme() {
  const html = document.documentElement; // Use html element instead of body
  html.classList.toggle('dark');
  const themeButton = document.getElementById('themeBtn');
  themeButton.textContent = html.classList.contains('dark') ? '' : '';
}
