import { getRoomInfo, getActiveNews, isResponseError } from "./api.js";

"use strict";

document.addEventListener("DOMContentLoaded", initializePage);
window.addEventListener("resize", calculateTickerSpeed);

async function initializePage() {
  await initializeRoomTitle();
  await initializeNews();
  calculateTickerSpeed();
}

async function initializeRoomTitle() {
  if (window.roomId == null) {
    // We are not on a page that uses room id.
    return;
  }
  

  const titleElement = document.getElementById("title");
  console.assert(titleElement);

  const roomInfo = await getRoomInfo(window.roomId);

  if (isResponseError(roomInfo)) {
    // Just fuck off if we hit an error.
    location.href = "/";
    return;
  }

  // HTML injection on purpose
  titleElement.innerHTML = `TREOENs STREGSYSTEM : ${roomInfo.content.name}`;
}

async function initializeNews() {
  const newsItems = document.getElementById("news-items");
  console.assert(newsItems);

  const news = await getActiveNews();
  // TODO: Error handling
  window.news = news.content.news;
}

function calculateTickerSpeed() {
  const newsItems = document.getElementById("news-items");
  console.assert(newsItems);

  // Clear news items
  newsItems.innerHTML = "";

  if (window.news.length === 0) {
    return;
  }

  // Add at least one copy of news
  addCopyOfNews(newsItems);

  // Add items until we have at least window width.
  while (window.innerWidth > newsItems.clientWidth) {
    addCopyOfNews(newsItems);
  }

  // Add another exact copy to do infinite scrolling
  const width = newsItems.clientWidth;
  while (width * 2 > newsItems.clientWidth) {
    addCopyOfNews(newsItems);
  }

  // Yeah this does not take dpi into account TOO BAD :)
  const pixelsPerSecond = 60;

  const seconds = newsItems.clientWidth / (pixelsPerSecond * 2);

  newsItems.style.animationDuration = `${seconds}s`;
}

function addCopyOfNews(newsItems) {
  for (const n of window.news) {
    const span = document.createElement("span");
    // This HTML injection is intentional
    span.innerHTML = n;
    newsItems.appendChild(span);
  }
}
