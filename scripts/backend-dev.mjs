#!/usr/bin/env node
/* neira:meta
id: NEI-20250831-backend-dev
intent: utility
summary: Запуск backend с выбором порта через NEIRA_BIND_ADDR.
*/
/* global process */
import { spawn } from "node:child_process";

const args = process.argv.slice(2);
let port = process.env.npm_config_port || "3000";
const idx = args.indexOf("--port");
if (idx !== -1 && args[idx + 1]) port = args[idx + 1];

const env = { ...process.env, NEIRA_BIND_ADDR: `0.0.0.0:${port}` };
const child = spawn("cargo", ["run", "--manifest-path", "backend/Cargo.toml"], {
  stdio: "inherit",
  env,
});
child.on("exit", (code) => process.exit(code));
