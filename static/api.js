"use strict";

export async function getActiveProducts() {
  const url = "/api/products/active";
  return await getRequest(url);
}

export async function getActiveNews() {
  const url = "/api/news/active";
  return await getRequest(url);
}

export async function postQuickBuy(quickbuyQuery) {
  const url = "/api/purchase/quickbuy";
  return await postRequest(url, { quickbuy: quickbuyQuery });
}

export function isResponseOk(response) {
  return response.status === "Ok";
}

export function isResponseError(response) {
  return !isResponseOk(response);
}


async function getRequest(url) {
  // TODO: Introduce timeout, retry, and exponential backoff.
  const response = await fetch(
    url,
    {
      method: "GET",
      headers: {
        "Accept": "application/json"
      },
      cache: "no-store"
    });

  const isJson = response.headers.get("Content-Type") === "application/json";

  if (isJson) {
    return await response.json();
  }

  const text = await response.text();
  return { status: "Error", content: { "InternalServerError": { "text": text } } };
}

async function postRequest(url, body) {
  // TODO: Introduce timeout, retry, and exponential backoff.
  const response = await fetch(
    url,
    {
      method: "POST",
      headers: {
        "Accept": "application/json",
        "Content-Type": "application/json"
      },
      cache: "no-store",
      body: JSON.stringify(body)
    });

  const isJson = response.headers.get("Content-Type") === "application/json";

  if (isJson) {
    return await response.json();
  }

  const text = await response.text();
  return { status: "Error", content: { "InternalServerError": { "text": text } } };
}
