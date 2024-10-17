"use strict";

const slowness = 5;

const cssElement = document.createElement("link");
cssElement.rel = "stylesheet";
cssElement.href = "/static/themes/bat/bat.css";
document.head.appendChild(cssElement);

const batContainerElement = document.createElement("div");
batContainerElement.id = "bat-container";
document.body.appendChild(batContainerElement);

function createBat() {
  const batElement = document.createElement("div");
  batElement.className = "bat";

  const imageElement = document.createElement("div");
  imageElement.className = "img";
  imageElement.style.animationDelay = `${-Math.random()}s`;
  batElement.appendChild(imageElement);

  batContainerElement.appendChild(batElement);

  const width = 48;
  const height = 48;

  let x = Math.random() * (window.innerWidth - width);
  let y = Math.random() * (window.innerHeight - height);

  batElement.style.left = x + "px";
  batElement.style.top = y + "px";

  function updateBat() {
    const width = imageElement.clientWidth;
    const height = imageElement.clientHeight;

    let newX = Math.random() * (window.innerWidth - width);
    let newY = Math.random() * (window.innerHeight - height);

    let distance = Math.sqrt((newX - x) ** 2 + (newY - y) ** 2) * slowness;

    batElement.style.transform = newX > x ? "scaleX(-1)" : "scaleX(1)";
    batElement.style.transitionDuration = distance + "ms";
    batElement.style.left = newX + "px";
    batElement.style.top = newY + "px";

    x = newX;
    y = newY;

    return distance;
  }

  function updater() {
    let distance = updateBat();
    setTimeout(updater, distance);
  }

  updateBat();
  setTimeout(updater, 0.1);
}

const now = new Date();
for (let i = 0; i < now.getDate(); i++) {
  createBat();
}
