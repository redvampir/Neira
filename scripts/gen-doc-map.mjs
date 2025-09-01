#!/usr/bin/env node

/* neira:meta
id: NEI-20250225-120000-gen-doc-map
intent: feature
summary: |
  Генерирует Markdown-оглавление по содержимому каталога docs и обновляет docs/index.md.
*/

import { promises as fs } from "fs";
import path from "path";

const DOCS_DIR = path.resolve("docs");

const pad = (n) => String(n).padStart(2, "0");
const d = new Date();
const metaId = `NEI-${d.getFullYear()}${pad(d.getMonth() + 1)}${pad(d.getDate())}-${pad(d.getHours())}${pad(d.getMinutes())}${pad(d.getSeconds())}-doc-map`;

async function walk(dir, depth = 0) {
  const entries = await fs.readdir(dir, { withFileTypes: true });
  entries.sort((a, b) => a.name.localeCompare(b.name));
  const lines = [];
  for (const entry of entries) {
    if (entry.name === "index.md") continue;
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      lines.push(`${"  ".repeat(depth)}- ${entry.name}/`);
      lines.push(...(await walk(full, depth + 1)));
    } else if (entry.isFile() && entry.name.endsWith(".md")) {
      const rel = path.relative(DOCS_DIR, full).replace(/\\/g, "/");
      const title = entry.name.replace(/\.md$/, "");
      lines.push(`${"  ".repeat(depth)}- [${title}](${rel})`);
    }
  }
  return lines;
}

const lines = await walk(DOCS_DIR);
const content = `<!-- neira:meta\nid: ${metaId}\nintent: docs\nsummary: |\n  Автогенерированный список файлов документации.\n-->\n\n# Документация — оглавление\n\n${lines.join("\n")}\n`;

await fs.writeFile(path.join(DOCS_DIR, "index.md"), content);
