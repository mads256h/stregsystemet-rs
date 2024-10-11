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
  const batElement = document.createElement("img");
  batElement.className = "bat";
  batElement.src = "/static/themes/bat/bat.gif";
  batContainerElement.appendChild(batElement);

  const width = batElement.clientWidth;
  const height = batElement.clientHeight;

  let x = Math.random() * (window.innerWidth - width);
  let y = Math.random() * (window.innerHeight - height);

  function updateBat() {
    const width = batElement.clientWidth;
    const height = batElement.clientHeight;

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
