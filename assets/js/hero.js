window.addEventListener('load', () => {
  setTimeout(animateHero, 500);
});

function animateHero() {
  document.querySelectorAll('#hero .fake-line').forEach((e, i) => {
    let length = e.dataset.length;

    e.style.transition = `width ${length * 100}ms ease-in-out ${i * 200}ms`;
    e.style.width = `${length}ch`;
  });
}
