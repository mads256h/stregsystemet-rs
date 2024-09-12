use maud::{html, Markup};

use crate::dso::product::Product;

pub fn products_table_component(products: &[Product]) -> Markup {
    let left_products = products.iter().step_by(2);
    let right_products = products.iter().skip(1).step_by(2);

    products_table_template_component(
        products_table_part_component(left_products),
        products_table_part_component(right_products),
    )
}

pub fn clickable_products_table_component(products: &[Product], username: &str) -> Markup {
    let left_products = products.iter().step_by(2);
    let right_products = products.iter().skip(1).step_by(2);

    products_table_template_component(
        clickable_products_table_part_component(left_products, username),
        clickable_products_table_part_component(right_products, username),
    )
}

// TODO: SHIT FUCKING NAME
fn products_table_template_component(left_products: Markup, right_products: Markup) -> Markup {
    html! {
        table cellpadding="2" cellspacing="2" border="0" {
            tr {
                td valign="top" { (left_products) }
                td valign="top" { (right_products) }
            }
        }
    }
}

fn products_table_part_component<'a, I>(products: I) -> Markup
where
    I: Iterator<Item = &'a Product>,
{
    html! {
        table cellpadding="2" cellspacing="2" border="1" {
            tr {
                th { "ID" }
                th { "Produkt" }
                th { "Pris" }
            }

            @for product in products {
                tr {
                    td { (product.id) }
                    td { (product.name) }
                    td align="right" { (product.price) "kr" }
                }
            }
        }
    }
}

fn clickable_products_table_part_component<'a, I>(products: I, username: &str) -> Markup
where
    I: Iterator<Item = &'a Product>,
{
    html! {
        table cellpadding="2" cellspacing="2" border="1" {
            tr {
                th { "Produkt" }
                th { "Pris" }
            }

            @for product in products {
                tr {
                    td { (clickable_product_link_component(&product, username)) }
                    td align="right" { (product.price) "kr" }
                }
            }
        }
    }
}

fn clickable_product_link_component(product: &Product, username: &str) -> Markup {
    html! {
        form method="post" action="/buy/" {
            a href="" onclick="this.closest('form').submit(); return false;" { (product.name) }

            input type="hidden" name="quickbuy" value={ (username) " " (product.id) } ;
        }
    }
}
