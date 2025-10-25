/* neira:meta
id: NEI-20250301-120600-metrics-diagram-upd
intent: refactor
summary: |
  Обновлён компонент диаграммы: типизация на TypeScript, поддержка повторного
  подключения, опции для базового URL и интервалов опроса. Добавляет очистку
  таймера при повторной инициализации.
*/

const DEFAULT_LIMIT = 5;
const DEFAULT_POLL_MS = 5000;

export interface MetricsOptions {
  apiBase?: string;
  pollMs?: number;
  limit?: number;
}

export async function renderMetrics(
  canvas: HTMLCanvasElement,
  options: MetricsOptions = {},
): Promise<void> {
  const context = canvas.getContext("2d");
  if (!context) {
    throw new Error("Не удалось получить контекст canvas для диаграммы метрик");
  }

  const apiBase = options.apiBase ?? import.meta.env.VITE_API_URL ?? "";
  const pollMs = options.pollMs ?? DEFAULT_POLL_MS;
  const limit = options.limit ?? DEFAULT_LIMIT;

  if (!apiBase) {
    drawMessage(context, canvas, "Нет адреса API. Укажите VITE_API_URL.");
    return;
  }

  const existingTimer = canvas.dataset.metricsTimer;
  if (existingTimer) {
    window.clearInterval(Number(existingTimer));
  }

  async function load(): Promise<void> {
    try {
      const response = await fetch(`${apiBase}/metrics`);
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      const text = await response.text();
      const metrics = parseMetrics(text).slice(0, limit);
      drawChart(context, canvas, metrics);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      drawMessage(context, canvas, `Ошибка: ${message}`);
    }
  }

  await load();
  const timer = window.setInterval(load, pollMs);
  canvas.dataset.metricsTimer = String(timer);

  if (typeof MutationObserver !== "undefined" && canvas.ownerDocument) {
    const observer = new MutationObserver(() => {
      if (!canvas.isConnected) {
        window.clearInterval(timer);
        observer.disconnect();
      }
    });
    observer.observe(canvas.ownerDocument, { childList: true, subtree: true });
  }
}

interface MetricEntry {
  name: string;
  value: number;
}

function parseMetrics(payload: string): MetricEntry[] {
  return payload
    .split("\n")
    .filter((line) => line && !line.startsWith("#"))
    .map((line) => {
      const [name, value] = line.split(/\s+/);
      return { name, value: Number(value) };
    })
    .filter((entry) => Number.isFinite(entry.value));
}

function drawChart(
  context: CanvasRenderingContext2D,
  canvas: HTMLCanvasElement,
  metrics: MetricEntry[],
): void {
  context.clearRect(0, 0, canvas.width, canvas.height);
  if (metrics.length === 0) {
    drawMessage(context, canvas, "Нет данных метрик");
    return;
  }

  const barWidth = canvas.width / metrics.length;
  const maxValue = Math.max(...metrics.map((metric) => metric.value), 1);
  context.font = "12px Inter, Arial, sans-serif";
  context.textBaseline = "bottom";

  metrics.forEach((metric, index) => {
    const normalizedHeight = Math.max(0, metric.value) / maxValue;
    const height = normalizedHeight * (canvas.height - 24);
    const x = index * barWidth + 4;
    const y = canvas.height - height - 12;

    context.fillStyle = "#38bdf8";
    context.fillRect(x, y, barWidth - 8, height);

    context.fillStyle = "#0f172a";
    context.fillText(metric.name, x, canvas.height - 2);
    context.fillText(String(metric.value), x, y - 4);
  });
}

function drawMessage(
  context: CanvasRenderingContext2D,
  canvas: HTMLCanvasElement,
  message: string,
): void {
  context.clearRect(0, 0, canvas.width, canvas.height);
  context.fillStyle = "#0f172a";
  context.font = "14px Inter, Arial, sans-serif";
  context.textBaseline = "top";
  wrapText(context, message, 8, 8, canvas.width - 16, 18);
}

function wrapText(
  context: CanvasRenderingContext2D,
  text: string,
  x: number,
  y: number,
  maxWidth: number,
  lineHeight: number,
): void {
  const words = text.split(" ");
  let line = "";

  words.forEach((word) => {
    const testLine = line ? `${line} ${word}` : word;
    if (context.measureText(testLine).width > maxWidth) {
      context.fillText(line, x, y);
      line = word;
      y += lineHeight;
    } else {
      line = testLine;
    }
  });

  if (line) {
    context.fillText(line, x, y);
  }
}
