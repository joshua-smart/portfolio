window.addEventListener('load', () => {
  document.querySelectorAll('[data-h-scroll]').forEach(applyHorizontalScroll);
});

const scrollSpeed = 50;

function applyHorizontalScroll(container) {
    container.addEventListener('wheel', event => {
      const deltaY = event.deltaY;

      container.scrollLeft += scrollSpeed * Math.sign(deltaY);
      event.preventDefault();
    });
}
