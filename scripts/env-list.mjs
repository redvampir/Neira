#!/usr/bin/env node
/* neira:meta
id: NEI-20250930-env-list
intent: utility
summary: Выводит актуальные значения переменных окружения Neira.
*/
/* global console, process */
import { promises as fs } from "node:fs";
import path from "node:path";

async function collectVars() {
  const text = await fs.readFile(path.resolve("docs/reference/env.md"), "utf8");
  const vars = [];
  for (const line of text.split("\n")) {
    const m = /^\|\s*([A-Z0-9_]+)\s*\|/.exec(line);
    if (m) vars.push(m[1]);
  }
  return Array.from(new Set(vars)).sort();
}

async function main() {
  const vars = await collectVars();
  for (const v of vars) {
    const val = process.env[v] ?? "";
    console.log(`${v}=${val}`);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main();
}
