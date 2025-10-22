/* neira:meta
id: NEI-20250904-120100-check-meta
intent: chore
summary: |
  Добавлен скрипт проверки покрытия neira:meta по изменённым файлам
  (staged или относительно базовой ветки). Интегрируется в pre-commit и CI.
*/
/* eslint-env node */
/* global console, process */

// Lightweight neira:meta coverage checker.
// Usage:
//   node scripts/check-meta.mjs --staged
//   node scripts/check-meta.mjs --since <git-ref>
//   node scripts/check-meta.mjs <file1> <file2> ...
//   Add --strict to require valid YAML in meta blocks

import { execSync } from "node:child_process";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

let YAMLmod = null;
try {
  // Optional dependency; used in --strict mode when available
  const m = await import("yaml");
  YAMLmod = m.default || m;
} catch {
  /* ignore */
}

function run(cmd) {
  return execSync(cmd, { stdio: ["ignore", "pipe", "ignore"] })
    .toString()
    .trim();
}

function getChangedFilesFromStaged() {
  try {
    const out = run("git diff --cached --name-only");
    return out ? out.split(/\r?\n/).filter(Boolean) : [];
  } catch {
    return [];
  }
}

function getChangedFilesSince(ref) {
  try {
    const out = run(`git diff --name-only ${ref}...HEAD`);
    return out ? out.split(/\r?\n/).filter(Boolean) : [];
  } catch {
    return [];
  }
}

const IGNORED_DIRS = [
  "node_modules/",
  ".git/",
  "llvm-project/",
  "binutils-gdb/",
  "nasm/",
  "generated/",
  "logs/",
  "patches/",
  "target/",
  "cargo/",
];

const CODE_EXTENSIONS = new Set([
  ".rs",
  ".ts",
  ".tsx",
  ".js",
  ".mjs",
  ".c",
  ".h",
  ".cpp",
  ".hpp",
]);
const DOC_EXTENSIONS = new Set([".md", ".mdx", ".markdown", ".html", ".htm"]);

function isIgnored(file) {
  const norm = file.replaceAll("\\", "/");
  return IGNORED_DIRS.some((p) => norm.startsWith(p));
}

function requiresMeta(file) {
  if (isIgnored(file)) return false;
  const ext = path.extname(file).toLowerCase();
  const norm = file.replaceAll("\\", "/");

  // Skip tests and fixtures by default
  if (/^(tests?|test)\//.test(norm)) return false;
  if (/(^|\/)__tests__\//.test(norm)) return false;
  if (/\.(spec|test)\.(ts|tsx|js|mjs|rs)$/i.test(norm)) return false;

  // Only enforce for docs and core runtime/scripts
  const inDocs = norm.startsWith("docs/");
  const inCore =
    norm.startsWith("src/") ||
    norm.startsWith("spinal_cord/") ||
    norm.startsWith("sensory_organs/") ||
    norm.startsWith("scripts/");
  if (!inDocs && !inCore) return false;

  // Only enforce for known doc/code types
  if (!(CODE_EXTENSIONS.has(ext) || DOC_EXTENSIONS.has(ext))) return false;

  return true;
}

function extractMetaBlocks(text) {
  const blocks = [];
  // HTML style <!-- neira:meta ... -->
  {
    const re = /<!--\s*neira:meta[\s\S]*?-->/g;
    let m;
    while ((m = re.exec(text))) blocks.push({ raw: m[0], kind: "html" });
  }
  // Block comment /* neira:meta ... */
  {
    const re = /\/\*\s*neira:meta[\s\S]*?\*\//g;
    let m;
    while ((m = re.exec(text))) blocks.push({ raw: m[0], kind: "block" });
  }
  // Hash comment lines starting with # neira:meta then subsequent # lines
  {
    const lines = text.split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
      if (/^\s*#\s*neira:meta/.test(lines[i])) {
        const buf = [lines[i]];
        let j = i + 1;
        while (j < lines.length && /^\s*#/.test(lines[j])) {
          buf.push(lines[j]);
          j++;
        }
        i = j - 1;
        blocks.push({ raw: buf.join("\n"), kind: "hash" });
      }
    }
  }
  return blocks;
}

function parseMetaLight(block) {
  // Strip wrappers
  let body = block.raw;
  if (block.kind === "html") {
    body = body.replace(/^<!--\s*neira:meta\s*/i, "").replace(/-->\s*$/i, "");
  } else if (block.kind === "block") {
    body = body.replace(/^\/\*\s*neira:meta\s*/i, "").replace(/\*\/\s*$/i, "");
  } else if (block.kind === "hash") {
    body = body
      .split(/\r?\n/)
      .map((l) => l.replace(/^\s*#\s?/, ""))
      .join("\n");
  }
  const lines = body.split(/\r?\n/);
  const out = {};
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    if (!line) continue;
    const m = /^([a-zA-Z_][a-zA-Z0-9_-]*):\s*(.*)$/.exec(line);
    if (!m) continue;
    const key = m[1];
    let value = m[2];
    if (key === "summary") {
      out.summary = value || "present";
    } else if (key === "id") {
      out.id = value;
    } else if (key === "intent") {
      out.intent = value;
    }
  }
  return out;
}

const INTENT_SET = new Set([
  "feature",
  "fix",
  "refactor",
  "docs",
  "perf",
  "security",
  "chore",
  "ci",
  "build",
]);

function validateMetaObject(obj) {
  const errors = [];
  if (!obj || typeof obj !== "object") errors.push("invalid_yaml");
  const id = obj?.id;
  const intent = obj?.intent;
  const summary = obj?.summary;
  if (!id) errors.push("missing_id");
  const idRe = /^NEI-\d{8}-\d{6}-[a-z0-9-]+$/i;
  if (id && !idRe.test(id)) errors.push("invalid_id");
  if (!intent) errors.push("missing_intent");
  if (intent && !INTENT_SET.has(String(intent).toLowerCase()))
    errors.push("invalid_intent");
  if (!summary) errors.push("missing_summary");
  return { valid: errors.length === 0, errors };
}

function main(argv) {
  const args = argv.slice(2);
  let files = [];
  const sinceIdx = args.indexOf("--since");
  const staged = args.includes("--staged");

  if (staged) {
    files = getChangedFilesFromStaged();
  } else if (sinceIdx !== -1 && args[sinceIdx + 1]) {
    files = getChangedFilesSince(args[sinceIdx + 1]);
  } else if (args.length) {
    files = args.filter((a) => !a.startsWith("--"));
  } else {
    // Fallback: try merge-base with origin/main if available
    try {
      const base = run("git merge-base HEAD origin/main");
      files = getChangedFilesSince(base);
    } catch {
      files = [];
    }
  }

  const toCheck = files.filter(requiresMeta);
  if (!toCheck.length) {
    console.log("neira:meta coverage: nothing to check");
    return 0;
  }

  const missing = [];
  const invalid = [];
  for (const f of toCheck) {
    try {
      if (!existsSync(f)) continue; // deleted or renamed
      const buf = readFileSync(f);
      const text = buf.toString("utf8");
      const blocks = extractMetaBlocks(text);
      if (!blocks.length) {
        missing.push(f);
        continue;
      }
      // Validate first block that parses into required keys
      let checked = false;
      const strict = args.includes("--strict");
      for (const b of blocks) {
        if (strict) {
          if (!YAMLmod) {
            invalid.push({ file: f, errors: ["invalid_yaml"] });
            checked = true;
            break;
          }
          // Strict: parse YAML body and validate keys
          let body = b.raw;
          if (b.kind === "html")
            body = body
              .replace(/^<!--\s*neira:meta\s*/i, "")
              .replace(/-->\s*$/i, "");
          else if (b.kind === "block")
            body = body
              .replace(/^\/\*\s*neira:meta\s*/i, "")
              .replace(/\*\/\s*$/i, "");
          else if (b.kind === "hash")
            body = body
              .split(/\r?\n/)
              .map((l) => l.replace(/^\s*#\s?/, ""))
              .join("\n");
          try {
            const data = YAMLmod.parse(body);
            const v = validateMetaObject(data);
            invalid.push(...(v.valid ? [] : [{ file: f, errors: v.errors }]));
          } catch {
            invalid.push({ file: f, errors: ["invalid_yaml"] });
          }
          checked = true;
          break;
        } else {
          const obj = parseMetaLight(b);
          const v = validateMetaObject(obj);
          invalid.push(...(v.valid ? [] : [{ file: f, errors: v.errors }]));
          checked = true;
          break;
        }
      }
      if (!checked)
        invalid.push({ file: f, errors: ["missing_required_keys"] });
    } catch {
      // If can't read, skip from failure rather than block commits
      /* ignore */
    }
  }
  const hasFailures = missing.length || invalid.length;

  // Reporting options
  // reuse args from the beginning of main()
  const outIdx = args.indexOf("--out");
  const outPath = outIdx !== -1 ? args[outIdx + 1] : null;
  const reportFmt = args.includes("--report")
    ? args[args.indexOf("--report") + 1]
    : null;

  const report = { total: toCheck.length, missing, invalid };
  if (outPath) {
    try {
      writeFileSync(outPath, JSON.stringify(report, null, 2));
    } catch {
      /* ignore */
    }
  }

  if (reportFmt === "summary" || reportFmt === "github") {
    const lines = [];
    lines.push("### neira:meta coverage report");
    lines.push("");
    lines.push(`Checked files: ${report.total}`);
    lines.push(`Missing: ${missing.length} | Invalid: ${invalid.length}`);
    if (missing.length) {
      lines.push("");
      lines.push("Missing blocks:");
      for (const f of missing) lines.push(`- ${f}`);
    }
    if (invalid.length) {
      lines.push("");
      lines.push("Invalid blocks:");
      for (const it of invalid)
        lines.push(`- ${it.file}: ${it.errors.join(", ")}`);
    }
    const summary = lines.join("\n");
    if (reportFmt === "github" && process.env.GITHUB_STEP_SUMMARY) {
      try {
        writeFileSync(process.env.GITHUB_STEP_SUMMARY, summary + "\n", {
          flag: "a",
        });
      } catch {
        /* ignore */
      }
    } else {
      console.log(summary);
    }
  }

  if (hasFailures) {
    console.error("neira:meta coverage check failed. See details above.");
    console.error("See COMMENTING.md and META_COVERAGE.md for rules.");
    process.exitCode = 1;
    return 1;
  }

  console.log(`neira:meta coverage OK (${toCheck.length} file(s) checked)`);
  return 0;
}

main(process.argv);
