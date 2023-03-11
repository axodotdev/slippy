const SLIDES_SELECTOR = "section";
const UP = ["ArrowUp", 38];
const DOWN = ["ArrowDown", 40];
const LEFT = ["ArrowLeft", 37];
const RIGHT = ["ArrowRight", 39];
const BACKSPACE = ["Backspace", 8];
const PAGE_DOWN = ["PageDown", 34];
const PAGE_UP = ["PageUp", 33];
const FORWARD = [].concat(RIGHT, DOWN, PAGE_DOWN);
const BACKWARD = [].concat(LEFT, UP, BACKSPACE, PAGE_UP);
let SELECTION_TYPE = true;
const progressNode = document.querySelector(".progress-bar");
const slides = document.querySelectorAll(SLIDES_SELECTOR);
const totalSlides = slides.length;

const init = () => {
  let current = -1;

  showSlide(parseInt(window.location.hash.split("#")[1]) || 0);

  document.querySelector("body").addEventListener("keyup", (e) => {
    const key = e.key || e.keyCode;
    if (FORWARD.includes(key)) {
      if (current + 1 < totalSlides) {
        showSlide(current + 1);
      }
    } else if (BACKWARD.includes(key)) {
      if (current - 1 >= 0) {
        showSlide(current - 1);
      }
    }
  });

  function showSlide(idx) {
    if (current >= 0) {
      slides[current].style.visibility = "hidden";
      slides[current].style.opacity = 0;
    }
    current = idx;
    slides[current].style.visibility = "visible";
    slides[current].style.opacity = 1;

    postSlide(current);
  }
};

const updateProgress = (current) =>
  (progressNode.style.width = `${((current + 1) / totalSlides) * 100}%`);

const postSlide = (current) => {
  updateProgress(current);
  setTimeout(() => {
    window.location.href = "#" + current;
  }, 1);
};

init();
