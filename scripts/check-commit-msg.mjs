/* neira:meta
id: NEI-20250904-120200-commit-msg-check
intent: chore
summary: |
  Добавлен валидатор Conventional Commits для сообщения коммита.
  Подключается через .husky/commit-msg.
*/

/* neira:meta
id: NEI-20250904-163139-change
intent: chore
summary: |
  Удалён блок const TYPES, добавлен eslint-env для Node
  и объявлены глобальные console и process.
*/

/* eslint-env node */

import { readFileSync } from "node:fs";

const { console, process } = globalThis;

const TYPE_RE =
  /^(feat|fix|docs|style|refactor|perf|test|chore|ci|build|revert)(\([^)]+\))?(!)?:\s.+$/;

function fail(msg) {
  console.error("Conventional Commits check failed:");
  console.error(" - " + msg);
  console.error("\nExample:");
  console.error(" - feat(spinal_cord): add meta coverage checker");
  process.exit(1);
}

function main(argv) {
  const file = argv[2];
  if (!file) return fail("No commit message file provided");
  const text = readFileSync(file, "utf8");
  let first = text.split(/\r?\n/)[0] || "";
  if (first.charCodeAt(0) === 0xfeff) first = first.slice(1);

  if (first.startsWith("Merge ")) return process.exit(0);
  if (/^Revert\s+"/.test(first)) return process.exit(0);

  if (!TYPE_RE.test(first)) {
    return fail('First line must match "type(scope?): subject"');
  }

  // Length check for subject (<=72 chars total line is recommended)
  if (first.length > 72) {
    return fail("First line should be <= 72 chars");
  }

  // Optional: disallow trailing period
  if (/\.$/.test(first)) {
    return fail("Subject should not end with a period");
  }

  process.exit(0);
}

main(process.argv);
