import { getActiveProducts, postQuickBuy, isResponseOk } from "./api.js";
import { populateTable } from "./product-table.js";

"use strict";

document.addEventListener("DOMContentLoaded", initializePage);

async function initializePage() {
  addQuickBuyHandler();

  try {
    const activeProducts = await getActiveProducts();
    // TODO: Error handling
    const products = activeProducts.content.products;
    populateTable(products, populateProductNameCell);
  }
  catch (error) {
    console.error(error.message);
  }
}

function addQuickBuyHandler() {
  const quickBuyForm = document.getElementById("quickbuy");
  console.assert(quickBuyForm);

  quickBuyForm.addEventListener("submit", performQuickBuy);
}

async function performQuickBuy(e) {
  // Stop the default form submission logic
  e.preventDefault();

  const quickBuyInput = document.getElementById("quickbuy-field");
  console.assert(quickBuyInput);

  const response = await postQuickBuy(quickBuyInput.value);
  console.log(response);

  if (isResponseOk(response)) {
    // Redirect to menu page if user only typed in username
    if (response.content.type === "Username") {
      const username = response.content.username;
      window.location.href = `/menu/#username=${encodeURIComponent(username)}`;
    }
  }
}

function populateProductNameCell(cell, product) {
  // This HTML injection is intentional
  cell.innerHTML = product.name;
  cell.title = getProductTooltip(product);
}

function getProductTooltip(product) {
  if (product.aliases.length === 0) {
    return "";
  }

  return "Aliasser:\n"
    + product.aliases.join("\n");
}
