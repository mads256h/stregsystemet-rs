"use strict";

const numRetries = 3;
const timeoutMs = 5000;
const retryAfter = 1000;

const statusCodes = {
  "internalServerError": 500
}

export async function getActiveProducts() {
  const url = "/api/products/active";
  return await getRequest(url);
}

export async function getUserInfo(username) {
  const url = `/api/users/info?username=${encodeURIComponent(username)}`;
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

export async function getThemeDefinitions() {
  const url = "/static/themes/definitions.json";

  // TODO: Introduce timeout, retry, and exponential backoff.
  const response = await fetch(
    url,
    {
      method: "GET",
      headers: {
        "Accept": "application/json"
      }
    });

  const isJson = response.headers.get("Content-Type") === "application/json";

  if (isJson) {
    return await response.json();
  }

  const text = await response.text();
  return { status: "Error", content: { "InternalServerError": { "text": text } } };
}

export function isResponseOk(response) {
  return response.status === "Ok";
}

export function isResponseError(response) {
  return !isResponseOk(response);
}


async function getRequest(url) {
  return await retryRequestLoop(url, "GET", null);
}

async function postRequest(url, body) {
  const stringBody = JSON.stringify(body);

  return await retryRequestLoop(url, "POST", stringBody);
}

async function retryRequestLoop(url, method, body) {
  // TODO: Idempotency key is not used for get requests but keep it for now.
  const idempotencyKey = generateUuid();
  let response;
  for (let attempt = 0; attempt < numRetries; attempt++) {
    const next_request_time = Date.now() + retryAfter * 4 ** attempt;
    try {
      response = await performRequest(url, method, idempotencyKey, body, timeoutMs * 3 ** attempt);

      const isJson = response.headers.get("Content-Type") === "application/json";
      if (!isJson || response.status === statusCodes.internalServerError) {
        // Retry
        if (attempt !== numRetries - 1) {
          await sleep_until(next_request_time);
        }
        continue;
      }
      if (isJson) {
        return await response.json();
      }
    }
    catch (e) {
      console.warn(e);

      // Retry
      await sleep_until(next_request_time);
      continue;
    }
  }

  // We never got a response or every response is an internal server error
  if (response == null) {
    return { status: "Error", content: { "NoConnection": {} } };
  }

  const text = await response.text();
  return { status: "Error", content: { "InternalServerError": { "text": text } } };
}

async function performRequest(url, method, idempotencyKey, body, timeout) {
  const noBodyHeaders = {
    "Accept": "application/json",
    "X-Idempotency-Key": idempotencyKey
  };

  const bodyHeaders = {
    "Accept": "application/json",
    "Content-Type": "application/json",
    "X-Idempotency-Key": idempotencyKey
  }

  const headers = body == null ? noBodyHeaders : bodyHeaders;

  const response = await fetch(
    url,
    {
      method: method,
      headers: headers,
      cache: "no-store",
      body: body,
      signal: AbortSignal.timeout(timeout)
    });

  return response;
}

function generateUuid() {
  return "10000000-1000-4000-8000-100000000000".replace(/[018]/g, c =>
    (+c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> +c / 4).toString(16)
  );
}

function sleep_until(time) {
  const diff = time - Date.now();
  if (diff <= 0) {
    return new Promise(r => r());
  }

  return new Promise(resolve => setTimeout(resolve, diff));
}
