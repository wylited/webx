function handleHtmzTransition(iframe) {
  setTimeout(() => {
    const targetEl = document.querySelector(iframe.contentWindow.location.hash || null);
    if (!targetEl) return;

    const currentHeight = targetEl.scrollHeight;
    targetEl.style.maxHeight = currentHeight + 'px';
    
    void targetEl.offsetHeight;
    
    targetEl.style.maxHeight = '0px';
    targetEl.classList.add('transitioning');

    setTimeout(() => {
        targetEl.replaceWith(...iframe.contentDocument.body.childNodes);
        
        const newEl = document.querySelector(iframe.contentWindow.location.hash);
        if (!newEl) return;
        
        newEl.style.maxHeight = '0px';
        newEl.classList.add('transitioning');
        
        void newEl.offsetHeight;
        
        const finalHeight = newEl.scrollHeight;
        newEl.style.maxHeight = finalHeight + 'px';
        
        setTimeout(() => {
            newEl.classList.remove('transitioning');
            newEl.style.maxHeight = '';
        }, 500);
    }, 500);
  });
}
 
function togglePlay() {
  const audio = document.getElementById('audio');
  const button = document.getElementById('audioBtn');
  if (audio.paused) {
    audio.play();
    button.textContent = '||';
  } else {
    audio.pause();
    button.textContent = '|>';
  }
}

function toggleTheme() {
  const html = document.documentElement; // Use html element instead of body
  html.classList.toggle('dark');
  const themeButton = document.getElementById('themeBtn');
  themeButton.textContent = html.classList.contains('dark') ? '☼' : '☾';
}

function updateTime() {
  const timeElement = document.getElementById('time');
  const now = new Date();
  timeElement.textContent = now.toLocaleTimeString('en-US', {
    hour: 'numeric',
    minute: '2-digit',
    hour12: true
  });
}

function updateDuration() {
  const audio = document.getElementById('audio');
  const duration = document.getElementById('duration');
  
  duration.textContent = `${Math.floor(audio.currentTime / 60).toString().padStart(2, '0')}:${Math.floor(audio.currentTime % 60).toString().padStart(2, '0')}`;
}


setInterval(updateTime, 1000);
setInterval(updateDuration, 1000);


