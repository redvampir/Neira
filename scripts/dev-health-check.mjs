#!/usr/bin/env node
/* neira:meta
id: NEI-20270228-120000-dev-health-check
intent: chore
summary: Проверяет готовность spinal_cord после запуска dev-оркестратора.
*/
/* global process, AbortController, fetch, console */
import { setTimeout as scheduleTimeout, clearTimeout as cancelTimeout } from "node:timers";
import { setTimeout as delay } from "node:timers/promises";
import { URL } from "node:url";

const DEFAULT_RETRIES = 30;
const DEFAULT_INTERVAL_MS = 1000;
const DEFAULT_TIMEOUT_MS = 2000;

function parseBindAddress(rawAddr) {
  if (!rawAddr) {
    return { host: "0.0.0.0", port: "3000" };
  }

  if (rawAddr.startsWith("[")) {
    const closing = rawAddr.indexOf("]");
    if (closing === -1) {
      throw new Error(`Некорректное значение NEIRA_BIND_ADDR: ${rawAddr}`);
    }
    const host = rawAddr.slice(1, closing);
    const remainder = rawAddr.slice(closing + 1);
    const port = remainder.startsWith(":") ? remainder.slice(1) : "3000";
    return { host, port: port || "3000" };
  }

  const parts = rawAddr.split(":");
  if (parts.length === 1) {
    return { host: parts[0], port: "3000" };
  }
  const port = parts.pop();
  const host = parts.join(":") || "0.0.0.0";
  return { host, port: port || "3000" };
}

function buildTargetUrl() {
  const explicit = process.env.NEIRA_HEALTHCHECK_URL;
  if (explicit) {
    return new URL(explicit);
  }

  const { host, port } = parseBindAddress(process.env.NEIRA_BIND_ADDR);
  const protocol = process.env.NEIRA_HEALTHCHECK_PROTOCOL ?? "http";
  const path = process.env.NEIRA_HEALTHCHECK_PATH ?? "/";
  const normalizedHost = host === "0.0.0.0" ? "127.0.0.1" : host === "::" ? "[::1]" : host;

  const origin = `${protocol}://${normalizedHost.includes(":") && !normalizedHost.startsWith("[") ? `[${normalizedHost}]` : normalizedHost}:${port}`;
  return new URL(path, origin);
}

async function checkOnce(targetUrl, timeoutMs) {
  const controller = new AbortController();
  const timer = scheduleTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(targetUrl, { signal: controller.signal });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    return true;
  } finally {
    cancelTimeout(timer);
  }
}

async function main() {
  const targetUrl = buildTargetUrl();
  const retries = Number(process.env.NEIRA_HEALTHCHECK_RETRIES ?? DEFAULT_RETRIES);
  const intervalMs = Number(process.env.NEIRA_HEALTHCHECK_INTERVAL_MS ?? DEFAULT_INTERVAL_MS);
  const timeoutMs = Number(process.env.NEIRA_HEALTHCHECK_TIMEOUT_MS ?? DEFAULT_TIMEOUT_MS);

  if (Number.isNaN(retries) || retries <= 0) {
    console.error("[dev-health-check] Количество попыток должно быть положительным числом.");
    process.exit(1);
  }

  console.log(`[dev-health-check] Проверяю доступность API по адресу ${targetUrl.href}`);

  for (let attempt = 1; attempt <= retries; attempt += 1) {
    try {
      await checkOnce(targetUrl, timeoutMs);
      console.log(`[dev-health-check] API отвечает (попытка ${attempt}). Разработка готова.`);
      return;
    } catch (error) {
      const reason = error instanceof Error ? error.message : String(error);
      console.warn(
        `[dev-health-check] Попытка ${attempt} не удалась (${reason}). Повтор через ${intervalMs} мс.`,
      );
      await delay(intervalMs);
    }
  }

  console.error(
    `[dev-health-check] Не удалось дождаться отклика API за ${retries} попыток. Проверьте логи spinal_cord.`,
  );
  process.exit(1);
}

main().catch((error) => {
  const reason = error instanceof Error ? error.stack ?? error.message : String(error);
  console.error(`[dev-health-check] Критическая ошибка: ${reason}`);
  process.exit(1);
});
