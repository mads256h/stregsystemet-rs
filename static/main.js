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

function handleQuickBuyError(responseContent) {
  const quickBuyOutputElement = document.getElementById("quickbuy-output");
  console.assert(quickBuyOutputElement);

  quickBuyOutputElement.innerText = "";

  switch (responseContent.type) {
    case "Parser":
      handleQuickBuyParserError(responseContent.context);
      break;

    case "Executor":
      handleQuickBuyExecutorError(responseContent.context);
      break;
  }
}

function handleQuickBuyParserError(responseContent) {
  switch (responseContent.type) {
    case "EmptyQuery":
      displayError("Tom forspørgsel");
      break;

    case "Syntax":
      displayError("Syntax fejl");
      break;

    case "EmptyProduct":
      displayError("Tomt produkt navn");
      break;

    case "InvalidAmount":
      displayError("Ikke-positiv nummer af produkter angivet");
      break;

    default:
      displayError("Ukendt fejl. Se konsollen");
      console.error(responseContent);
      break;
  }
}

function handleQuickBuyExecutorError(responseContent) {
  switch (responseContent.type) {
    case "DbError":
      displayError("Database fejl. Se konsollen");
      console.error(responseContent);
      break;

    case "InvalidUsername":
      displayError(`Ukendt brugernavn: ${responseContent.context}`);
      break;

    case "InvalidProduct":
      displayError(`Ukendt produkt: ${responseContent.context}`);
      break;

    case "InsufficientFunds":
      location.href = `/stregforbud/#username=${encodeURIComponent(responseContent.context.username)}`;
      break;

    case "StregCentsOverflow":
      displayError("Overflow/underflow i stregcents");
      break;

    default:
      displayError("Ukendt fejl. Se konsollen")
      break;
  }
}

function displayError(text) {
  const quickBuyErrorElement = document.getElementById("quickbuy-error");
  console.assert(quickBuyErrorElement);

  quickBuyErrorElement.innerText = text;
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

  quickBuyOutputElement.innerText = `${username} har lige købt ${productsText} for tilsammen ${productPriceSum} kr`;
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
