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
