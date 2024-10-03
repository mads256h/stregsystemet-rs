"use strict";

document.addEventListener("DOMContentLoaded", initializePage);
window.addEventListener("resize", calculateTickerSpeed);

async function initializePage() {
  await initializeNews();
  calculateTickerSpeed();
}

async function initializeNews() {
  const newsItems = document.getElementById("news-items");
  console.assert(newsItems);

  const news = await getNews();
  window.news = news;
}

async function getNews() {
  // TODO: Error handling pls :)
  const url = "/api/news/active";
  const response = await fetch(url,
    {
      method: "GET",
      headers: {
        "Accept": "application/json",
      },
    });

  const json = await response.json();
  return json.content.news;
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
