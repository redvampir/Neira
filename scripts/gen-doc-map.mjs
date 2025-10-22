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

async function section(entries) {
  const lines = [];
  for (const entry of entries) {
    const full = path.join(DOCS_DIR, entry);
    const stat = await fs.stat(full);
    if (stat.isDirectory()) {
      lines.push(`- ${entry}/`);
      lines.push(...(await walk(full, 1)));
    } else if (stat.isFile() && entry.endsWith(".md")) {
      const title = entry.replace(/\.md$/, "");
      lines.push(`- [${title}](${entry})`);
    }
  }
  return lines.join("\n");
}

const groups = [
  {
    title: "Системы (органы)",
    entries: [
      "README.md",
      "immune_system.md",
      "design",
      "system",
      "roadmap.md",
    ],
  },
  {
    title: "Клеточные уровни",
    entries: ["cell-ids.md", "metrics_cells.md", "cells"],
  },
  {
    title: "Guides",
    entries: ["examples", "guides"],
  },
  {
    title: "Reference",
    entries: ["api", "api/spinal_cord.md", "pathways.md", "meta", "reference"],
  },
  {
    title: "Legacy",
    entries: ["legacy"],
  },
];

const sections = [];
for (const g of groups) {
  const lines = await section(g.entries);
  sections.push(`## ${g.title}\n${lines}`);
}

const content = `<!-- neira:meta\nid: ${metaId}\nintent: docs\nsummary: |\n  Автогенерированный список файлов документации.\n-->\n\n# Документация — оглавление\n\n${sections.join("\n\n")}\n`;

await fs.writeFile(path.join(DOCS_DIR, "index.md"), content);
