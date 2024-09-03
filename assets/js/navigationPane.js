let pane = document.querySelector('#navigation-pane');
let cover = document.querySelector('#navigation-pane-cover');

function openNavigationPane() {
  pane.style.transform = 'translateX(0)';
  cover.style.display = 'block';
}

function closeNavigationPane() {
  pane.style.transform = 'translateX(-100%)';
  cover.style.display = 'none';
}

document.querySelectorAll('[data-open-nav]').forEach(e => {
  e.addEventListener('click', () => openNavigationPane());
});

document.querySelectorAll('[data-close-nav]').forEach(e => {
  e.addEventListener('click', () => closeNavigationPane());
});

