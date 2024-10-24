import { getActiveProducts, postQuickBuy, isResponseOk } from "./api.js";
import { populateTable, handleQuickBuyError } from "./product-table.js";

"use strict";

document.addEventListener("DOMContentLoaded", initializePage);

async function initializePage() {
  addQuickBuyHandler();

  try {
    const activeProducts = await getActiveProducts();
    // TODO: Error handling
    const products = activeProducts.content.products;
    window.products = products;
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

  disableQuickBuy();

  const response = await postQuickBuy(quickBuyInput.value);
  console.log(response);

  if (isResponseOk(response)) {
    // Redirect to menu page if user only typed in username
    if (response.content.type === "Username") {
      const username = response.content.username;
      window.location.href = `/menu/#username=${encodeURIComponent(username)}`;
      return;
    }

    if (response.content.type === "MultiBuy") {
      const quickBuyErrorElement = document.getElementById("quickbuy-error");
      console.assert(quickBuyErrorElement);
      quickBuyErrorElement.innerText = "";

      outputMultiBuyPurchase(response.content);
    }
  }
  else {
    handleQuickBuyError(response.content);
  }

  enableQuickBuy();
}

function disableQuickBuy() {
  const quickBuyInput = document.getElementById("quickbuy-field");
  console.assert(quickBuyInput);

  const quickBuyButton = document.getElementById("quickbuy-button");
  console.assert(quickBuyButton);

  quickBuyInput.disabled = true;
  quickBuyButton.disabled = true;
}

function enableQuickBuy() {
  const quickBuyInput = document.getElementById("quickbuy-field");
  console.assert(quickBuyInput);

  const quickBuyButton = document.getElementById("quickbuy-button");
  console.assert(quickBuyButton);

  quickBuyInput.disabled = false;
  quickBuyButton.disabled = false;
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
