const SLIDES_SELECTOR = "section";
const UP = ["ArrowUp", 38];
const DOWN = ["ArrowDown", 40];
const LEFT = ["ArrowLeft", 37];
const RIGHT = ["ArrowRight", 39];
const BACKSPACE = ["Backspace", 8];
const PAGE_DOWN = ["PageDown", 34];
const PAGE_UP = ["PageUp", 33];
const SHIFT = ["Shift", 16];
const FORWARD = [].concat(RIGHT, DOWN, PAGE_DOWN);
const BACKWARD = [].concat(LEFT, UP, BACKSPACE, PAGE_UP);
let SELECTION_TYPE = true;

function init() {
  const progressNode = document.querySelector(".progress-bar");
  const slides = document.querySelectorAll(SLIDES_SELECTOR);
  const totalSlides = slides.length;
  let current = -1;

  showSlide(parseInt(window.location.hash.substr(1)) || 0);

  document.querySelector("body").addEventListener("keyup", (e) => {
    const key = e.key || e.keyCode;
    if (FORWARD.includes(key)) {
      // ******************* forward
      if (current + 1 < totalSlides) {
        showSlide(current + 1);
      }
    } else if (BACKWARD.includes(key)) {
      // *********** backward
      if (current - 1 >= 0) {
        showSlide(current - 1);
      }
    }
    if (SHIFT.includes(key)) {
      setSelectionType();
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
    const postSlide = () => {
      updateProgress();
      setTimeout(() => {
        window.location.href = "#" + current;
      }, 1);
    };
    // Checking for a code snippet. If we have one we are waiting,
    // till it disappears.
    let int;
    (function checkForScriptTag() {
      clearTimeout(int);
      if (slides[current].innerHTML.indexOf("<script") >= 0) {
        int = setTimeout(checkForScriptTag, 100);
      } else {
        postSlide();
      }
    })();
  }

  function updateProgress() {
    const percents = ((current + 1) / totalSlides) * 100;
    progressNode.style.width = percents + "%";
  }
}

init();
