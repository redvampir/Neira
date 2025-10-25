/* neira:meta
id: NEI-20250301-120700-interface-refresh
intent: feature
summary: |
  Переработан интерфейс органа общения: добавлен герой-блок с повествованием об
  организме, панель статуса с живыми метриками и улучшенный чат. Поддержаны
  опрос контрольного статуса, автообновление и ручное обновление с обработкой
  ошибок.
*/

import "./style.css";
import {
  createChatPanel,
  createHero,
  createStatusPanel,
  type ChatPanel,
  type MessageRole,
} from "./ui-components";
import {
  setupStatusPolling,
  sendToNeira,
  type PollingController,
  type PollingOptions,
} from "./services";

const API_URL: string = import.meta.env.VITE_API_URL ?? "";
const POLL_INTERVAL_MS = 15000;

const appRoot = document.getElementById("app");
if (!appRoot) {
  throw new Error("Контейнер приложения #app не найден");
}

appRoot.innerHTML = "";
appRoot.classList.add("neira-shell");

const hero = createHero();
const layout = document.createElement("div");
layout.className = "neira-layout";

const chatPanel = createChatPanel(API_URL, (message) => sendToNeira(API_URL, message));
const statusPanel = createStatusPanel(API_URL);

layout.append(chatPanel.container, statusPanel.container);
appRoot.append(hero.container, layout);

const chatFacade: ChatPanel | null = chatPanel;

const pollingOptions: PollingOptions = {
  apiUrl: API_URL,
  intervalMs: POLL_INTERVAL_MS,
  onStatus(status, narrative) {
    statusPanel.update(status, narrative);
    statusPanel.showError();
    hero.update(narrative);
    chatPanel.updateContext(narrative);
  },
  onError(message) {
    statusPanel.showError(message);
  },
  onBusy(busy) {
    statusPanel.setBusy(busy);
  },
  onOffline(message) {
    statusPanel.setOffline(message);
    hero.setOffline(message);
    chatPanel.setOffline(message);
  },
};

const poller: PollingController = setupStatusPolling(pollingOptions);
statusPanel.onRefresh(() => {
  void poller.refresh();
});

export function addMessage(role: MessageRole, text: string): void {
  if (!chatFacade) {
    throw new Error("Чат ещё не инициализирован");
  }
  chatFacade.addMessage(role, text);
}
