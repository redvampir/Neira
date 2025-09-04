/* neira:meta
id: NEI-20250904-121200-gen-id
intent: chore
summary: Добавлен генератор идентификаторов NEI-YYYYMMDD-HHMMSS-<slug> (UTC).
*/
/* eslint-env node */

/* global console, process */

import crypto from 'node:crypto';

function tsUTC(date = new Date()) {
  const pad = (n, l = 2) => String(n).padStart(l, '0');
  const y = date.getUTCFullYear();
  const m = pad(date.getUTCMonth() + 1);
  const d = pad(date.getUTCDate());
  const hh = pad(date.getUTCHours());
  const mm = pad(date.getUTCMinutes());
  const ss = pad(date.getUTCSeconds());
  return `${y}${m}${d}-${hh}${mm}${ss}`;
}

function slugify(s) {
  return String(s || '')
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 40) || crypto.randomBytes(3).toString('hex');
}

function main(argv) {
  const args = argv.slice(2);
  const slugArgIdx = args.indexOf('--slug');
  const raw = slugArgIdx !== -1 ? args[slugArgIdx + 1] : (args[0] && !args[0].startsWith('--') ? args[0] : 'change');
  const id = `NEI-${tsUTC()}-${slugify(raw)}`;
  console.log(id);
}

main(process.argv);

