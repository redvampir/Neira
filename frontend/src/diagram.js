/* neira:meta
id: NEI-20250219-metrics-diagram-component
intent: example
summary: |
  Компонент диаграммы метрик: fetch /metrics и рисует простую гистограмму.
*/

/* global fetch, setInterval */
export async function renderMetrics(canvas) {
  async function load() {
    try {
      const res = await fetch('/metrics');
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const text = await res.text();
      const data = parseMetrics(text).slice(0, 5);
      drawChart(data);
    } catch (err) {
      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.fillText(`Ошибка: ${err.message}`, 10, 20);
    }
  }

  function parseMetrics(text) {
    return text
      .split('\n')
      .filter(line => line && !line.startsWith('#'))
      .map(line => {
        const [name, value] = line.split(/\s+/);
        return { name, value: Number(value) };
      });
  }

  function drawChart(metrics) {
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const width = canvas.width / metrics.length;
    metrics.forEach((m, i) => {
      const h = Math.min(canvas.height, m.value);
      ctx.fillStyle = '#4caf50';
      ctx.fillRect(i * width + 2, canvas.height - h, width - 4, h);
      ctx.fillStyle = '#000';
      ctx.fillText(m.name, i * width + 4, canvas.height - h - 4);
    });
  }

  await load();
  setInterval(load, 5000);
}
