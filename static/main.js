import {getActiveProducts, postQuickBuy} from "./api.js";

"use strict";

document.addEventListener("DOMContentLoaded", initializePage);

async function initializePage() {
  addQuickBuyHandler();

  try {
    const activeProducts = await getActiveProducts();
    // TODO: Error handling
    const products = activeProducts.content.products;
    populateTable(products);
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
}

function populateTable(products) {
  const table1 = document.getElementById("products1").getElementsByTagName("tbody")[0];
  const table2 = document.getElementById("products2").getElementsByTagName("tbody")[0];
  for (const i in products) {
    const row = createRow(products[i]);
    if (i % 2 === 0) {
      table1.appendChild(row);
    } else {
      table2.appendChild(row);
    }

  }
}

function createRow(product) {
  const row = document.createElement("tr")
  const id = createTableCell(product.id);
  const name = createTableCell(product.name);
  name.title = getProductTooltip(product);
  const price = createTableCell(`${product.price} kr`);
  row.appendChild(id);
  row.appendChild(name);
  row.appendChild(price);
  return row;
}

function createTableCell(text) {
  const cell = document.createElement("td")
  // This HTML injection is intentional
  cell.innerHTML = text;
  return cell;
}

function getProductTooltip(product) {
  if (product.aliases.length === 0) {
    return "";
  }

  return "Aliasser:\n"
    + product.aliases.join("\n");
}
