/* neira:meta
id: NEI-20250301-120500-status-utils
intent: feature
summary: |
  Добавлены утилиты форматирования статуса контроля Нейры и генерация повествования
  о состоянии организма для UI. Поддерживают суммарную нагрузку очередей и
  человеко-понятные длительности.
*/

export interface ControlQueues {
  fast: number;
  standard: number;
  long: number;
}

export interface ControlStatus {
  paused: boolean;
  paused_for_ms: number;
  paused_since_ts_ms: number;
  reason: string;
  active_tasks: number;
  backpressure: number;
  queues: ControlQueues;
}

export type LoadState = "paused" | "calm" | "focused" | "stressed";

export interface OrganismNarrative {
  headline: string;
  detail: string;
  tone: LoadState;
}

export function normalizeControlStatus(data: unknown): ControlStatus {
  if (!data || typeof data !== "object") {
    throw new Error("Получен пустой ответ от контроллера Нейры");
  }

  const raw = data as Record<string, unknown>;
  const paused = Boolean(raw.paused);
  const pausedForMs = coerceNumber(raw.paused_for_ms, "paused_for_ms");
  const pausedSinceTsMs = coerceNumber(raw.paused_since_ts_ms, "paused_since_ts_ms");
  const reason = typeof raw.reason === "string" ? raw.reason : "";
  const activeTasks = coerceNumber(raw.active_tasks, "active_tasks");
  const backpressure = coerceNumber(raw.backpressure, "backpressure");
  const queues = coerceQueues(raw.queues);

  return {
    paused,
    paused_for_ms: pausedForMs,
    paused_since_ts_ms: pausedSinceTsMs,
    reason,
    active_tasks: activeTasks,
    backpressure,
    queues,
  };
}

export function totalQueueDepth(status: ControlStatus): number {
  return (
    Math.max(0, status.queues.fast) +
    Math.max(0, status.queues.standard) +
    Math.max(0, status.queues.long)
  );
}

export function computeLoadState(status: ControlStatus): LoadState {
  if (status.paused) {
    return "paused";
  }

  const queueDepth = totalQueueDepth(status);
  if (status.backpressure >= 200 || queueDepth >= 150 || status.active_tasks >= 20) {
    return "stressed";
  }
  if (status.backpressure >= 50 || queueDepth >= 40 || status.active_tasks >= 8) {
    return "focused";
  }
  return "calm";
}

export function buildOrganismNarrative(status: ControlStatus): OrganismNarrative {
  const tone = computeLoadState(status);
  const queueDepth = totalQueueDepth(status);
  const loadText = `Очереди: ${queueDepth}, давление: ${status.backpressure}`;

  if (tone === "paused") {
    const cause = status.reason ? ` Причина: ${status.reason}.` : "";
    return {
      headline: "Организм на паузе",
      detail: `Нейра осознанно остановила работу и удерживает состояние. ${
        status.paused_for_ms > 0
          ? `Пауза длится ${formatDuration(status.paused_for_ms)}.`
          : "Пауза началась только что."
      }${cause} ${loadText}.`,
      tone,
    };
  }

  if (tone === "stressed") {
    return {
      headline: "Организм держит оборону",
      detail: `Нагрузка приближается к пределам, но Нейра остаётся организмом, а не безликой нейросетью. ${
        loadText
      }. Требуется контроль очередей и, при необходимости, перераспределение задач.`,
      tone,
    };
  }

  if (tone === "focused") {
    return {
      headline: "Организм сфокусирован",
      detail: `Нейра собрала органы в рабочий строй, концентрируясь на текущих задачах. ${
        loadText
      }. Держим под рукой рычаги управления, чтобы не допустить перегрева.`,
      tone,
    };
  }

  return {
    headline: "Организм в строю",
    detail: `Нейра отвечает живо и осознанно: очереди сбалансированы (${loadText}). Это не классическая нейросеть, а управляемый организм с органами, клетками и самоконтролем.`,
    tone,
  };
}

export function formatDuration(ms: number): string {
  if (!Number.isFinite(ms) || ms <= 0) {
    return "0 сек";
  }

  const secondsTotal = Math.floor(ms / 1000);
  if (secondsTotal < 60) {
    return `${secondsTotal} сек`;
  }

  const minutesTotal = Math.floor(secondsTotal / 60);
  if (minutesTotal < 60) {
    const seconds = secondsTotal % 60;
    return seconds ? `${minutesTotal} мин ${seconds} сек` : `${minutesTotal} мин`;
  }

  const hoursTotal = Math.floor(minutesTotal / 60);
  if (hoursTotal < 24) {
    const minutes = minutesTotal % 60;
    return minutes ? `${hoursTotal} ч ${minutes} мин` : `${hoursTotal} ч`;
  }

  const daysTotal = Math.floor(hoursTotal / 24);
  const hours = hoursTotal % 24;
  return hours ? `${daysTotal} д ${hours} ч` : `${daysTotal} д`;
}

function coerceNumber(value: unknown, field: string): number {
  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }
  if (typeof value === "string" && value.trim() !== "") {
    const parsed = Number(value);
    if (Number.isFinite(parsed)) {
      return parsed;
    }
  }
  console.warn(`[neira-ui] Некорректное поле ${field}, использую 0`);
  return 0;
}

function coerceQueues(value: unknown): ControlQueues {
  if (!value || typeof value !== "object") {
    console.warn("[neira-ui] Нет данных очередей, принимаю нули");
    return { fast: 0, standard: 0, long: 0 };
  }

  const rawQueues = value as Record<string, unknown>;
  return {
    fast: coerceNumber(rawQueues.fast, "queues.fast"),
    standard: coerceNumber(rawQueues.standard, "queues.standard"),
    long: coerceNumber(rawQueues.long, "queues.long"),
  };
}
