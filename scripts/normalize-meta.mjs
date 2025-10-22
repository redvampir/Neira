/* neira:meta
id: NEI-20250904-121200-normalize-meta
intent: chore
summary: |
  Скрипт нормализации neira:meta: выравнивает id по формату NEI-YYYYMMDD-HHMMSS-<slug>,
  приводит intent к допустимым (docs и др.), может добавлять отсутствующие блоки.
*/

/* neira:meta
id: NEI-20250904-162001-eslint-env
intent: chore
summary: |
  Удалён параметр opts, добавлен eslint-env node
  и помечен пустой catch.
*/

/* eslint-env node */

import { execSync } from 'node:child_process';
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import path from 'node:path';

const { console, process } = globalThis;

function run(cmd) {
  return execSync(cmd, { stdio: ['ignore', 'pipe', 'ignore'] }).toString();
}

function gitFiles(scope) {
  const out = run(`git ls-files -z -- ${scope}`);
  return out.split('\x00').filter(Boolean);
}

function nowUTC() {
  const d = new Date();
  const p = (n, l = 2) => String(n).padStart(l, '0');
  return `${d.getUTCFullYear()}${p(d.getUTCMonth() + 1)}${p(d.getUTCDate())}-${p(d.getUTCHours())}${p(d.getUTCMinutes())}${p(d.getUTCSeconds())}`;
}

function slugify(s) {
  return String(s || '')
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 40) || 'meta';
}

function isMetaFile(f) {
  const norm = f.replaceAll('\\', '/');
  return norm.startsWith('docs/');
}

function extractBlocks(text) {
  const blocks = [];
  const html = /<!--\s*neira:meta[\s\S]*?-->/g; let m;
  while ((m = html.exec(text))) blocks.push({ raw: m[0], kind: 'html' });
  const block = /\/\*\s*neira:meta[\s\S]*?\*\//g; m = null;
  while ((m = block.exec(text))) blocks.push({ raw: m[0], kind: 'block' });
  const lines = text.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    if (/^\s*#\s*neira:meta/.test(lines[i])) {
      const buf = [lines[i]]; let j = i + 1;
      while (j < lines.length && /^\s*#/.test(lines[j])) { buf.push(lines[j]); j++; }
      i = j - 1; blocks.push({ raw: buf.join('\n'), kind: 'hash' });
    }
  }
  return blocks;
}

const INTENT_SET = new Set(['feature','fix','refactor','docs','perf','security','chore','ci','build']);

function normalizeBlockRaw(raw, kind, file) {
  // Prepare body
  let body = raw;
  if (kind === 'html') body = body.replace(/^<!--\s*neira:meta\s*/i, '').replace(/-->\s*$/i, '');
  else if (kind === 'block') body = body.replace(/^\/\*\s*neira:meta\s*/i, '').replace(/\*\/\s*$/i, '');
  else if (kind === 'hash') body = body.split(/\r?\n/).map((l) => l.replace(/^\s*#\s?/, '')).join('\n');
  const lines = body.split(/\r?\n/);
  const map = {};
  for (const ln of lines) {
    const m = /^([a-zA-Z_][a-zA-Z0-9_-]*):\s*(.*)$/.exec(ln.trim());
    if (m) map[m[1]] = m[2];
  }
  const base = path.basename(file).replace(/\.[^.]+$/, '');
  const idRe = /^NEI-\d{8}-\d{6}-[a-z0-9-]+$/i;
  let id = map.id && idRe.test(map.id) ? map.id : `NEI-${nowUTC()}-${slugify(base)}`;

  let intent = map.intent || '';
  const intentLower = String(intent).toLowerCase();
  if (!INTENT_SET.has(intentLower)) intent = 'docs';

  // Rebuild body with updated keys, preserving summary and rest as-is
  let newBody = body
    .replace(/(^|\n)\s*id:\s*.*(?=\n|$)/i, `$1id: ${id}`)
    .replace(/(^|\n)\s*intent:\s*.*(?=\n|$)/i, `$1intent: ${intent}`);
  if (!/\nsummary\s*:/.test(newBody) && !/^summary\s*:/m.test(newBody)) {
    newBody += `\nsummary: |\n  Updated by normalize-meta`;
  }

  if (kind === 'html') return `<!-- nei${''}ra:meta\n${newBody}\n-->`;
  if (kind === 'block') return `/* nei${''}ra:meta\n${newBody}\n*/`;
  if (kind === 'hash') return newBody.split(/\r?\n/).map((l) => '# ' + l).join('\n');
  return raw;
}

function addBlockForFile(file) {
  const base = path.basename(file).replace(/\.[^.]+$/, '');
  const id = `NEI-${nowUTC()}-${slugify(base)}`;
  return `<!-- nei${''}ra:meta\nid: ${id}\nintent: docs\nsummary: |\n  Добавлен первичный neira:meta блок.\n-->\n\n`;
}

function main(argv) {
  const args = argv.slice(2);
  const scopeIdx = args.indexOf('--scope');
  const scope = scopeIdx !== -1 ? args[scopeIdx + 1] : 'docs';
  const write = args.includes('--write');
  const addMissing = args.includes('--add-missing');

  const files = gitFiles(scope).filter((f) => isMetaFile(f) && /\.(md|mdx|html?)$/i.test(f));
  const changes = [];
  for (const f of files) {
    try {
      if (!existsSync(f)) continue;
      const text = readFileSync(f, 'utf8');
      const blocks = extractBlocks(text);
      if (!blocks.length) {
        if (addMissing) {
          const updated = addBlockForFile(f) + text;
          changes.push({ file: f, action: 'add' });
          if (write) writeFileSync(f, updated, 'utf8');
        }
        continue;
      }
      const b = blocks[0];
      const newRaw = normalizeBlockRaw(b.raw, b.kind, f);
      if (newRaw !== b.raw) {
        const updated = text.replace(b.raw, newRaw);
        changes.push({ file: f, action: 'update' });
        if (write) writeFileSync(f, updated, 'utf8');
      }
    } catch { /* ignore */ }
  }
  const summary = `normalize-meta: ${changes.length} file(s) ${write ? 'changed' : 'would change'}`;
  console.log(summary);
  for (const c of changes) console.log(` - ${c.action}: ${c.file}`);
}

main(process.argv);

