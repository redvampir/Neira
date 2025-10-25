/* neira:meta
id: NEI-20250301-120900-ui-components
intent: feature
summary: |
  Вынесены строительные функции интерфейса: герой-блок, чат и панель статуса.
  Компоненты принимают зависимости через параметры и используют утилиты
  состояния организма.
*/

import { renderMetrics } from "./diagram";
import {
  ControlStatus,
  formatDuration,
  OrganismNarrative,
  totalQueueDepth,
} from "./status-utils";

export type MessageRole = "user" | "bot" | "system" | "error";

export interface ChatPanel {
  container: HTMLElement;
  addMessage: (role: MessageRole, text: string) => void;
  setPending: (pending: boolean, note?: string) => void;
  updateContext: (narrative: OrganismNarrative) => void;
  setOffline: (message: string) => void;
}

export interface StatusPanel {
  container: HTMLElement;
  update: (status: ControlStatus, narrative: OrganismNarrative) => void;
  showError: (message?: string) => void;
  setBusy: (busy: boolean) => void;
  onRefresh: (callback: () => void) => void;
  setOffline: (message: string) => void;
}

export interface HeroView {
  container: HTMLElement;
  update: (narrative: OrganismNarrative) => void;
  setOffline: (message: string) => void;
}

export type MessageSender = (message: string) => Promise<string>;

export function createHero(): HeroView {
  const container = document.createElement("header");
  container.className = "neira-hero";

  const title = document.createElement("h1");
  title.textContent = "Нейра — живой программный организм";

  const subtitle = document.createElement("p");
  subtitle.className = "hero-subtitle";
  subtitle.textContent =
    "Интерфейс разговора с организмом, который сам управляет органами, клетками и безопасностью.";

  const badges = document.createElement("div");
  badges.className = "hero-badges";

  const organismBadge = document.createElement("span");
  organismBadge.className = "hero-badge";
  organismBadge.textContent = "Организм, а не нейросеть";

  const stateBadge = document.createElement("span");
  stateBadge.className = "hero-badge hero-badge--state";
  stateBadge.textContent = "Статус неизвестен";

  badges.append(organismBadge, stateBadge);
  container.append(title, subtitle, badges);

  return {
    container,
    update(narrative: OrganismNarrative) {
      subtitle.textContent = narrative.detail;
      stateBadge.textContent = narrative.headline;
      container.dataset.tone = narrative.tone;
    },
    setOffline(message: string) {
      stateBadge.textContent = "Нет связи";
      subtitle.textContent = message;
      container.dataset.tone = "paused";
    },
  };
}

export function createChatPanel(
  apiUrl: string,
  sendMessage: MessageSender,
): ChatPanel {
  const container = document.createElement("section");
  container.className = "panel chat-panel";

  const header = document.createElement("div");
  header.className = "chat-header";

  const title = document.createElement("h2");
  title.textContent = "Диалог с Нейрой";

  const context = document.createElement("p");
  context.className = "chat-context";
  context.textContent =
    "Задавайте вопросы — Нейра ответит как организм, опираясь на органы и клетки.";

  header.append(title, context);

  const messages = document.createElement("div");
  messages.id = "messages";
  messages.className = "chat-log";

  const form = document.createElement("form");
  form.className = "chat-form";

  const input = document.createElement("textarea");
  input.name = "message";
  input.rows = 3;
  input.placeholder = "Расскажите, что нужно организму…";
  input.required = true;

  const controls = document.createElement("div");
  controls.className = "chat-controls";

  const submit = document.createElement("button");
  submit.type = "submit";
  submit.className = "chat-submit";
  submit.textContent = "Отправить";

  const note = document.createElement("p");
  note.className = "chat-note";
  note.textContent = "Мы фиксируем request_id для трассировки каждой реплики.";

  controls.append(submit, note);
  form.append(input, controls);

  container.append(header, messages, form);

  const api: ChatPanel = {
    container,
    addMessage(role, text) {
      const entry = document.createElement("div");
      entry.className = `msg ${role}`;

      const body = document.createElement("p");
      body.className = "msg-body";
      body.textContent = text;

      const meta = document.createElement("span");
      meta.className = "msg-meta";
      meta.textContent = new Date().toLocaleTimeString();

      entry.append(body, meta);
      messages.append(entry);
      messages.scrollTop = messages.scrollHeight;
    },
    setPending(pending, pendingNote) {
      submit.disabled = pending;
      submit.textContent = pending ? "Ждём ответ…" : "Отправить";
      if (pendingNote) {
        note.textContent = pendingNote;
      }
    },
    updateContext(narrative) {
      context.textContent = narrative.detail;
    },
    setOffline(message) {
      input.disabled = true;
      submit.disabled = true;
      note.textContent = message;
    },
  };

  api.addMessage(
    "system",
    "Это не безликая нейросеть — Нейра собирает ответ целостным организмом. Подскажите ей задачу, и она задействует органы и клетки.",
  );

  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    const text = input.value.trim();
    if (!text) {
      return;
    }

    api.addMessage("user", text);
    input.value = "";
    api.setPending(true, "Отправили запрос в орган речи…");

    try {
      const reply = await sendMessage(text);
      api.addMessage("bot", reply);
      note.textContent = "Ответ получен и протоколирован.";
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      api.addMessage("error", `Ошибка: ${message}`);
      note.textContent = "Связь не установлена — проверьте API или токен.";
    } finally {
      api.setPending(false);
    }
  });

  if (!apiUrl) {
    api.setOffline("VITE_API_URL не указан — сообщения не отправляются.");
  }

  return api;
}

export function createStatusPanel(apiUrl: string): StatusPanel {
  const container = document.createElement("section");
  container.className = "panel status-panel";

  const header = document.createElement("div");
  header.className = "status-header";

  const title = document.createElement("h2");
  title.textContent = "Состояние организма";

  const refresh = document.createElement("button");
  refresh.type = "button";
  refresh.className = "status-refresh";
  refresh.textContent = "Обновить";

  header.append(title, refresh);

  const cards = document.createElement("div");
  cards.className = "status-cards";

  const modeCard = createStatusCard("Режим организма");
  const tasksCard = createStatusCard("Активных задач");
  const pressureCard = createStatusCard("Суммарное давление");
  const queueCard = createStatusCard("Очередь клеток");
  const pauseCard = createStatusCard("Длительность паузы");

  cards.append(modeCard.container, tasksCard.container, pressureCard.container, queueCard.container, pauseCard.container);

  const narrativeTitle = document.createElement("h3");
  narrativeTitle.className = "status-narrative-title";
  narrativeTitle.textContent = "Диагноз";

  const narrativeText = document.createElement("p");
  narrativeText.className = "status-narrative";
  narrativeText.textContent = "Ожидаем данные от позвоночника.";

  const footer = document.createElement("div");
  footer.className = "status-footer";

  const timestamp = document.createElement("span");
  timestamp.className = "status-updated";
  timestamp.textContent = "Нет обновлений";

  const errorBox = document.createElement("div");
  errorBox.className = "status-error hidden";

  const metricsWrap = document.createElement("div");
  metricsWrap.className = "status-metrics";

  const metricsTitle = document.createElement("h4");
  metricsTitle.textContent = "Пульс метрик";

  const canvas = document.createElement("canvas");
  canvas.width = 360;
  canvas.height = 180;
  canvas.className = "status-canvas";

  metricsWrap.append(metricsTitle, canvas);
  footer.append(timestamp);

  container.append(header, cards, narrativeTitle, narrativeText, metricsWrap, errorBox, footer);

  if (apiUrl) {
    void renderMetrics(canvas, { apiBase: apiUrl, pollMs: 6000, limit: 6 });
  } else {
    drawOfflineCanvas(canvas, "VITE_API_URL не указан — пульс недоступен");
  }

  let refreshHandler: (() => void) | null = null;

  refresh.addEventListener("click", () => {
    if (refreshHandler) {
      refreshHandler();
    }
  });

  const applyError = (message?: string) => {
    if (message) {
      errorBox.textContent = message;
      errorBox.classList.remove("hidden");
    } else {
      errorBox.textContent = "";
      errorBox.classList.add("hidden");
    }
  };

  return {
    container,
    update(status, narrative) {
      modeCard.setValue(status.paused ? "Пауза" : "В строю");
      tasksCard.setValue(String(status.active_tasks));
      pressureCard.setValue(String(status.backpressure));
      queueCard.setValue(String(totalQueueDepth(status)));
      pauseCard.setValue(status.paused ? formatDuration(status.paused_for_ms) : "0 сек");
      narrativeTitle.textContent = narrative.headline;
      narrativeText.textContent = narrative.detail;
      timestamp.textContent = `Обновлено ${new Date().toLocaleTimeString()}`;
      container.dataset.tone = narrative.tone;
    },
    showError(message) {
      applyError(message);
    },
    setBusy(busy) {
      refresh.disabled = busy;
      refresh.textContent = busy ? "Обновляем…" : "Обновить";
    },
    onRefresh(callback) {
      refreshHandler = callback;
    },
    setOffline(message) {
      applyError(message);
      refresh.disabled = true;
      metricsTitle.textContent = "Пульс недоступен";
      drawOfflineCanvas(canvas, message);
    },
  };
}

function createStatusCard(title: string): { container: HTMLElement; setValue: (value: string) => void } {
  const container = document.createElement("article");
  container.className = "status-card";

  const label = document.createElement("span");
  label.className = "status-card-label";
  label.textContent = title;

  const value = document.createElement("span");
  value.className = "status-card-value";
  value.textContent = "—";

  container.append(label, value);

  return {
    container,
    setValue(next) {
      value.textContent = next;
    },
  };
}

function drawOfflineCanvas(canvas: HTMLCanvasElement, message: string): void {
  const context = canvas.getContext("2d");
  if (!context) {
    return;
  }
  context.clearRect(0, 0, canvas.width, canvas.height);
  context.fillStyle = "#1e293b";
  context.fillRect(0, 0, canvas.width, canvas.height);
  context.fillStyle = "#f8fafc";
  context.font = "14px Inter, Arial, sans-serif";
  context.textBaseline = "top";
  context.fillText(message, 12, 12);
}
