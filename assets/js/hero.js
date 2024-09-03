const seText = document.querySelector('#hero-subtitle-se');
const sText = document.querySelector('#hero-subtitle-s');
const aText = document.querySelector('#hero-subtitle-a');

const elements = [seText, sText, aText];
const colorClasses = ["text-theme-red", "text-theme-green", "text-theme-yellow"];
let index = 0;

elements[index].classList.add(colorClasses[index]);

setInterval(() => {
  elements[index].classList.remove(colorClasses[index]);
  index = (index + 1) % elements.length;
  elements[index].classList.add(colorClasses[index]);
}, 3000);

