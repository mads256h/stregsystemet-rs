import { getActiveProducts, postQuickBuy, isResponseOk } from "./api.js";
import { populateTable, handleQuickBuyError } from "./product-table.js";

"use strict";

document.addEventListener("DOMContentLoaded", initializePage);

async function initializePage() {
  addQuickBuyHandler();

  try {
    const activeProducts = await getActiveProducts(window.roomId);
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

  const response = await postQuickBuy(quickBuyInput.value, window.roomId);
  console.log(response);

  if (isResponseOk(response)) {
    // Redirect to menu page if user only typed in username
    if (response.content.type === "Username") {
      const username = response.content.username;
      window.location.href = `/${window.roomId}/menu/#username=${encodeURIComponent(username)}`;
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

function outputMultiBuyPurchase(responseContent) {
  const username = responseContent.username;
  const boughtProducts = responseContent.bought_products;
  const productPriceSum = responseContent.product_price_sum;
  const newUserBalance = responseContent.new_user_balance;

  const quickBuyOutputElement = document.getElementById("quickbuy-output");
  console.assert(quickBuyOutputElement);

  // TODO: Output "og" between the last elements
  const productsText = boughtProducts.map(p => `${p.amount} stk ${window.products.find(f => f.id == p.product_id).name}`).join(", ");

  quickBuyOutputElement.innerText = `${username} har lige købt ${productsText} for tilsammen ${productPriceSum} kr\n`;
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
