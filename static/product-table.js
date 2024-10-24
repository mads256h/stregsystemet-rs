"use strict";

export function populateTable(products, productNamePopulator) {
  const table1 = document.getElementById("products1").getElementsByTagName("tbody")[0];
  const table2 = document.getElementById("products2").getElementsByTagName("tbody")[0];
  for (const i in products) {
    const row = createRow(products[i], productNamePopulator);
    if (i % 2 === 0) {
      table1.appendChild(row);
    } else {
      table2.appendChild(row);
    }

  }
}

function createRow(product, productNamePopulator) {
  const row = document.createElement("tr")
  const id = createTableCell(product.id);
  const name = createProductNameCell(product, productNamePopulator);
  const price = createTableCell(`${product.price} kr`);
  row.appendChild(id);
  row.appendChild(name);
  row.appendChild(price);
  return row;
}

function createProductNameCell(product, productNamePopulator) {
  const cell = document.createElement("td");
  productNamePopulator(cell, product);
  return cell;
}

function createTableCell(text) {
  const cell = document.createElement("td")
  cell.innerText = text;
  return cell;
}

export function handleQuickBuyError(responseContent) {
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
