/* neira:meta
id: NEI-20250301-120800-ui-services
intent: feature
summary: |
  Сервисный модуль для органов интерфейса: обработка сетевых запросов,
  опрос состояния позвоночника и генерация request_id.
*/

import {
  buildOrganismNarrative,
  ControlStatus,
  normalizeControlStatus,
  OrganismNarrative,
} from "./status-utils";

export interface PollingController {
  refresh: () => Promise<void>;
  dispose: () => void;
}

export interface PollingOptions {
  apiUrl: string;
  intervalMs: number;
  onStatus: (status: ControlStatus, narrative: OrganismNarrative) => void;
  onError: (message?: string) => void;
  onBusy: (busy: boolean) => void;
  onOffline: (message: string) => void;
}

const DEFAULT_REQUEST_TIMEOUT_MS = 12000;

export function setupStatusPolling(options: PollingOptions): PollingController {
  const { apiUrl, intervalMs, onStatus, onError, onBusy, onOffline } = options;
  if (!apiUrl) {
    const offlineMessage = "VITE_API_URL не настроен, поэтому данные статуса не загружаются.";
    onOffline(offlineMessage);
    return {
      refresh: async () => {
        onOffline(offlineMessage);
      },
      dispose: () => undefined,
    };
  }

  let timer: number | null = null;

  const visibilityHandler = (): void => {
    if (document.hidden) {
      if (timer !== null) {
        window.clearInterval(timer);
        timer = null;
      }
    } else {
      void refresh(true);
      arm();
    }
  };

  async function refresh(manual = false): Promise<void> {
    try {
      onBusy(true);
      const response = await fetchWithTimeout(`${apiUrl}/api/neira/control/status`, {
        headers: { Accept: "application/json" },
      });
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      const payload = (await response.json()) as unknown;
      const status = normalizeControlStatus(payload);
      const narrative = buildOrganismNarrative(status);
      onStatus(status, narrative);
      onError();
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      onError(`Не удалось обновить статус: ${message}`);
      if (!manual) {
        console.warn("[neira-ui] Ошибка обновления статуса", error);
      }
    } finally {
      onBusy(false);
    }
  }

  function arm(): void {
    if (timer !== null) {
      return;
    }
    timer = window.setInterval(() => {
      void refresh();
    }, intervalMs);
  }

  void refresh(true);
  arm();
  document.addEventListener("visibilitychange", visibilityHandler);

  return {
    refresh: () => refresh(true),
    dispose: () => {
      if (timer !== null) {
        window.clearInterval(timer);
        timer = null;
      }
      document.removeEventListener("visibilitychange", visibilityHandler);
    },
  };
}

export async function sendToNeira(apiUrl: string, message: string): Promise<string> {
  if (!apiUrl) {
    throw new Error("API не настроен (VITE_API_URL).");
  }

  const body = JSON.stringify({
    message,
    request_id: generateRequestId(),
    channel: "sensory_organs.ui",
  });

  const response = await fetchWithTimeout(`${apiUrl}/api/neira/interact`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body,
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  const data = (await response.json()) as { reply?: unknown };
  if (data && typeof data.reply === "string" && data.reply.trim()) {
    return data.reply.trim();
  }

  return JSON.stringify(data, null, 2);
}

export async function fetchWithTimeout(
  resource: string,
  init: RequestInit,
  timeoutMs: number = DEFAULT_REQUEST_TIMEOUT_MS,
): Promise<Response> {
  const controller = new AbortController();
  const timer = window.setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(resource, { ...init, signal: controller.signal });
    return response;
  } finally {
    window.clearTimeout(timer);
  }
}

function generateRequestId(): string {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return crypto.randomUUID();
  }
  return `sensory-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}
