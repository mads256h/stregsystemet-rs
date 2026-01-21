import { getActiveProducts, getUserInfo, postQuickBuy, isResponseOk, isResponseError } from "./api.js";
import { populateTable, handleQuickBuyError } from "./product-table.js";

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
    const userInfo = await getUserInfo(username);

    if (isResponseError(userInfo)) {
      // User does not exist go back to menu
      window.location.href = "/";
      return;
    }

    const userInfoElement = document.getElementById("user-info");
    userInfoElement.innerText = `${userInfo.content.first_name} ${userInfo.content.last_name} (${userInfo.content.email})`;

    setUserBalance(userInfo.content.balance);

    const activeProducts = await getActiveProducts(window.roomId);
    // TODO: Error handling
    const products = activeProducts.content.products;
    window.products = products;
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

    if (isResponseOk(response)) {
      // Redirect to menu page if user only typed in username
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

function setUserBalance(balance) {
  const userBalanceElement = document.getElementById("user-balance");
  console.assert(userBalanceElement);

  userBalanceElement.innerText = `Du har ${balance} kr til gode`;
}

function outputMultiBuyPurchase(responseContent) {
  const username = responseContent.username;
  const boughtProducts = responseContent.bought_products;
  const productPriceSum = responseContent.product_price_sum;
  const newUserBalance = responseContent.new_user_balance;

  setUserBalance(newUserBalance);

  const quickBuyOutputElement = document.getElementById("quickbuy-output");
  console.assert(quickBuyOutputElement);

  // TODO: Output "og" between the last elements
  const productsText = boughtProducts.map(p => `${p.amount} stk ${window.products.find(f => f.id == p.product_id).name}`).join(", ");

  quickBuyOutputElement.innerText += `${username} har lige købt ${productsText} for tilsammen ${productPriceSum} kr\n`;
}
