document.addEventListener("DOMContentLoaded", initializePage)

async function initializePage() {
    try {
        const activeProducts = await getActiveProducts();
        populateTable(activeProducts);
    }
    catch (error) {
        console.error(error.message);
    }
}

async function getActiveProducts() {
    const url = "/api/products/active";
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
    }

    const json = await response.json();
    console.log(json);
    return json.products;
}

function populateTable(products) {
    const table1 = document.getElementById("products1").getElementsByTagName("tbody")[0];
    const table2 = document.getElementById("products2").getElementsByTagName("tbody")[0];
    for (const i in products) {
        const row = createRow(products[i]);
        if (i % 2 == 0) {
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
    const price = createTableCell(product.price);
    row.appendChild(id);
    row.appendChild(name);
    row.appendChild(price);
    return row;
}

function createTableCell(text) {
    const cell = document.createElement("td")
    cell.innerText = text;
    return cell;
}