#!/usr/bin/env node
/* neira:meta
id: NEI-20250831-backend-dev
intent: utility
summary: Запуск backend с выбором адреса и порта через NEIRA_BIND_ADDR.
*/
/* global process */
import { spawn } from "node:child_process";

const args = process.argv.slice(2);
const addr = process.env.NEIRA_BIND_ADDR ?? "127.0.0.1:3000";
const sep = addr.lastIndexOf(":");
const host = sep !== -1 ? addr.slice(0, sep) : addr;
const defaultPort = sep !== -1 ? addr.slice(sep + 1) : "3000";

let port = process.env.npm_config_port || defaultPort;
const idx = args.indexOf("--port");
if (idx !== -1 && args[idx + 1]) port = args[idx + 1];

const env = { ...process.env, NEIRA_BIND_ADDR: `${host}:${port}` };
const child = spawn("cargo", ["run", "--manifest-path", "backend/Cargo.toml"], {
  stdio: "inherit",
  env,
});
child.on("exit", (code) => process.exit(code));
