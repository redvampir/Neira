/**
 * neira:meta
 * id: NEI-20250831-factory-shim-cli
 * intent: tool
 * summary: Внешний CLI-оркестратор фабрики с режимом LLM-агента (локальная модель), безопасные команды dry-run/create/approve/rollback.
 */

/* global fetch, process, console */
import fs from 'node:fs';

// Minimal Node.js (>=18) CLI without external deps.
// Env: FACTORY_BASE_URL, FACTORY_TOKEN, LLM_PROVIDER, LLM_BASE_URL, LLM_MODEL

const BASE = process.env.FACTORY_BASE_URL || 'http://127.0.0.1:3000';
const TOKEN = process.env.FACTORY_TOKEN || '';

function authHeaders() {
  const h = { 'Content-Type': 'application/json' };
  if (TOKEN) h.Authorization = `Bearer ${TOKEN}`;
  return h;
}

async function http(method, path, body) {
  const url = `${BASE}${path}`;
  const res = await fetch(url, {
    method,
    headers: authHeaders(),
    body: body ? JSON.stringify(body) : undefined,
  });
  const text = await res.text();
  let json;
  try { json = text ? JSON.parse(text) : {}; } catch { json = { raw: text }; }
  if (!res.ok) {
    const err = new Error(`HTTP ${res.status} ${res.statusText}`);
    err.status = res.status;
    err.body = json;
    throw err;
  }
  return json;
}

function parseArgs(argv) {
  const args = {};
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a.startsWith('--')) {
      const k = a.slice(2);
      const next = argv[i + 1];
      if (!next || next.startsWith('--')) {
        args[k] = true; // boolean flag
      } else {
        args[k] = next; i++;
      }
    } else if (!args._) {
      args._ = [a];
    } else {
      args._.push(a);
    }
  }
  return args;
}

function readJsonFileSync(path) {
  const data = fs.readFileSync(path, 'utf-8');
  return JSON.parse(data);
}

function print(obj) {
  console.log(JSON.stringify(obj, null, 2));
}

async function cmdDryrunCell({ spec }) {
  if (!spec) throw new Error('--spec <file.json> is required');
  const tpl = readJsonFileSync(spec);
  const body = { backend: 'adapter', ...tpl }; // Flattened CellTemplate per backend API
  const resp = await http('POST', '/factory/cells/dryrun', body);
  print(resp);
}

async function cmdCreateCell({ spec, hitl }) {
  if (!spec) throw new Error('--spec <file.json> is required');
  const tpl = readJsonFileSync(spec);
  const body = { ...tpl };
  if (hitl) body.hitl = true;
  const resp = await http('POST', '/factory/cells', body);
  print(resp);
}

function requireYes(args, action) {
  if (args.yes === true || args.yes === 'true') return Promise.resolve();
  return new Promise((resolve, reject) => {
    process.stdout.write(`${action} — confirm with 'yes': `);
    process.stdin.setEncoding('utf-8');
    process.stdin.once('data', (d) => {
      const ans = String(d).trim().toLowerCase();
      if (ans === 'yes') resolve(); else reject(new Error('Cancelled'));
    });
  });
}

async function cmdApproveCell(args) {
  const id = args.id || args.cell || args._?.[1];
  if (!id) throw new Error('--id <cell_id> is required');
  await requireYes(args, `Approve cell ${id}`);
  const resp = await http('POST', `/factory/cells/${encodeURIComponent(id)}/approve`);
  print(resp);
}

async function cmdDisableCell(args) {
  const id = args.id || args.cell || args._?.[1];
  if (!id) throw new Error('--id <cell_id> is required');
  await requireYes(args, `Disable cell ${id}`);
  const resp = await http('POST', `/factory/cells/${encodeURIComponent(id)}/disable`);
  print(resp);
}

async function cmdRollbackCell(args) {
  const id = args.id || args.cell || args._?.[1];
  if (!id) throw new Error('--id <cell_id> is required');
  await requireYes(args, `Rollback cell ${id}`);
  const resp = await http('POST', `/factory/cells/${encodeURIComponent(id)}/rollback`);
  print(resp);
}

async function cmdOrganBuild({ template, dryrun }) {
  if (!template) throw new Error('--template <file.json> is required');
  const organ_template = readJsonFileSync(template);
  const resp = await http('POST', '/organs/build', { organ_template, dryrun: !!dryrun });
  print(resp);
}

async function cmdOrganStatus({ id }) {
  if (!id) throw new Error('--id <organ_id> is required');
  const resp = await http('GET', `/organs/${encodeURIComponent(id)}/status`);
  print(resp);
}

// --- LLM Provider (minimal) ---
async function llmChat({ provider, baseUrl, model, messages }) {
  provider = provider || process.env.LLM_PROVIDER || 'ollama';
  baseUrl = baseUrl || process.env.LLM_BASE_URL || 'http://localhost:11434';
  model = model || process.env.LLM_MODEL || 'llama3';
  if (provider === 'ollama') {
    const url = `${baseUrl.replace(/\/$/, '')}/api/chat`;
    const res = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ model, messages, stream: false }),
    });
    if (!res.ok) throw new Error(`Ollama HTTP ${res.status}`);
    const j = await res.json();
    return j?.message?.content || '';
  }
  if (provider === 'openai') {
    const url = `${baseUrl.replace(/\/$/, '')}/v1/chat/completions`;
    const res = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${process.env.LLM_API_KEY || ''}` },
      body: JSON.stringify({ model, messages, temperature: 0 }),
    });
    if (!res.ok) throw new Error(`OpenAI-like HTTP ${res.status}`);
    const j = await res.json();
    return j?.choices?.[0]?.message?.content || '';
  }
  throw new Error(`Unsupported LLM_PROVIDER: ${provider}`);
}

const TOOLS = {
  dryrun_cell: {
    run: async ({ spec }) => { await cmdDryrunCell({ spec }); return { ok: true }; },
    schema: { type: 'object', required: ['spec'], properties: { spec: { type: 'string' } } },
  },
  create_cell: {
    run: async ({ spec, hitl }) => { await cmdCreateCell({ spec, hitl }); return { ok: true }; },
    schema: { type: 'object', required: ['spec'], properties: { spec: { type: 'string' }, hitl: { type: 'boolean' } } },
  },
  approve_cell: {
    run: async ({ id }) => { await cmdApproveCell({ id, yes: true }); return { ok: true }; },
    schema: { type: 'object', required: ['id'], properties: { id: { type: 'string' } } },
  },
  organ_build: {
    run: async ({ template, dryrun }) => { await cmdOrganBuild({ template, dryrun }); return { ok: true }; },
    schema: { type: 'object', required: ['template'], properties: { template: { type: 'string' }, dryrun: { type: 'boolean' } } },
  },
  organ_status: {
    run: async ({ id }) => { await cmdOrganStatus({ id }); return { ok: true }; },
    schema: { type: 'object', required: ['id'], properties: { id: { type: 'string' } } },
  },
};

function toolsSpecString() {
  const keys = Object.keys(TOOLS);
  return `You are a factory orchestrator. Choose a single tool and return ONLY a JSON object with fields: ` +
    `{"tool": one of [${keys.join(', ')}], "args": {..}, "reason": "short"}. No prose.`;
}

function safeJsonParse(s) {
  try { return JSON.parse(s); } catch { return null; }
}

async function cmdAgent(args) {
  const goal = args.goal || args._?.slice(1).join(' ');
  if (!goal) throw new Error('--goal "..." is required');
  const sys = toolsSpecString();
  const messages = [
    { role: 'system', content: sys },
    { role: 'user', content: goal },
  ];
  const out = await llmChat({ messages });
  const parsed = safeJsonParse(out.trim());
  if (!parsed || !parsed.tool || !TOOLS[parsed.tool]) {
    console.error('LLM did not return a valid tool call. Output:');
    console.error(out);
    process.exit(2);
  }
  // Safety: require explicit allow for approve/disable/rollback
  const sensitive = ['approve_cell'];
  if (sensitive.includes(parsed.tool) && !(args.yes === true || args.yes === 'true')) {
    console.error(`Refusing to execute sensitive tool ${parsed.tool} without --yes`);
    process.exit(3);
  }
  console.error(`LLM selected: ${parsed.tool} — ${parsed.reason || ''}`);
  const result = await TOOLS[parsed.tool].run(parsed.args || {});
  print({ ok: true, tool: parsed.tool, result });
}

async function main() {
  const argv = process.argv.slice(2);
  const cmd = argv[0];
  const args = parseArgs(argv.slice(1));
  try {
    switch (cmd) {
      case 'dryrun-cell': return await cmdDryrunCell(args);
      case 'create-cell': return await cmdCreateCell(args);
      case 'approve-cell': return await cmdApproveCell(args);
      case 'disable-cell': return await cmdDisableCell(args);
      case 'rollback-cell': return await cmdRollbackCell(args);
      case 'organ-build': return await cmdOrganBuild(args);
      case 'organ-status': return await cmdOrganStatus(args);
      case 'agent': return await cmdAgent(args);
      default:
        console.log('Usage:');
        console.log('  dryrun-cell --spec <file.json>');
        console.log('  create-cell --spec <file.json> [--hitl]');
        console.log('  approve-cell --id <cell_id> --yes');
        console.log('  disable-cell --id <cell_id> --yes');
        console.log('  rollback-cell --id <cell_id> --yes');
        console.log('  organ-build --template <file.json> [--dryrun]');
        console.log('  organ-status --id <organ_id>');
        console.log('  agent --goal "..." [--yes]');
    }
  } catch (e) {
    if (e.status && e.body) {
      console.error(`Error: ${e.message}`);
      print(e.body);
    } else {
      console.error(e.message || String(e));
    }
    process.exit(1);
  }
}

main();
