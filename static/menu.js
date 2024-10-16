import { getActiveProducts, postQuickBuy } from "./api.js";
import { populateTable } from "./product-table.js";

"use strict";

document.addEventListener("DOMContentLoaded", initializePage);

async function initializePage() {
  const username = getUsernameFromUrl(window.location.href);
  if (username === null) {
    // No username was given go back to the main page
    window.location.href = "/";
    return;
  }

  try {
    const activeProducts = await getActiveProducts();
    // TODO: Error handling
    const products = activeProducts.content.products;
    populateTable(products, (cell, product) => populateProductNameCell(cell, product, username));
  }
  catch (error) {
    console.error(error.message);
  }
}

function populateProductNameCell(cell, product, username) {
  const aElement = document.createElement("a");
  aElement.href = "#";
  aElement.addEventListener("click", e => { e.preventDefault(); buyProduct(product.id, username); });

  // This HTML injection is intentional
  aElement.innerHTML = product.name;

  cell.appendChild(aElement);
}

async function buyProduct(productId, username) {
  const productsTableElement = document.getElementById("product-tables");
  productsTableElement.classList.add("disabled");
  // TODO: Error handling
  try {
    const response = await postQuickBuy(`${username} ${productId}`);
    console.log(response);
  }
  finally {
    productsTableElement.classList.remove("disabled");
  }
}

function getUsernameFromUrl(url) {
  const split = url.split("#");

  if (split.length !== 2) {
    return null;
  }

  // Special characters are not a problem as they are URL encoded
  const usernameFragment = split[1];
  const usernameFragmentSplit = usernameFragment.split("=");
  if (split.length !== 2 || usernameFragmentSplit[0] !== "username") {
    return null;
  }

  const username = decodeURIComponent(usernameFragmentSplit[1]);

  if (username.includes(" ") || username.includes(":")) {
    return null;
  }

  return username;
}
