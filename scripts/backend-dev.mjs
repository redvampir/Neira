#!/usr/bin/env node
/* neira:meta
id: NEI-20250831-backend-dev
intent: utility
summary: Запуск backend с выбором свободного порта через NEIRA_BIND_ADDR.
*/
/* global console, process */
import { spawn } from "node:child_process";
import { createServer } from "node:net";

export async function choosePort(start) {
  let port = Number(start);
  while (port < 65535) {
    const free = await new Promise((resolve) => {
      const srv = createServer()
        .once("error", () => {
          srv.close();
          resolve(false);
        })
        .listen(port, () => srv.close(() => resolve(true)));
    });
    if (free) return port;
    port++;
  }
  throw new Error("no free port found");
}

async function main() {
  const args = process.argv.slice(2);
  let port = Number(process.env.npm_config_port || "3000");
  const idx = args.indexOf("--port");
  if (idx !== -1 && args[idx + 1]) port = Number(args[idx + 1]);
  port = await choosePort(port);

  const env = { ...process.env, NEIRA_BIND_ADDR: `0.0.0.0:${port}` };
  if (args.includes("--dry-run")) {
    console.log(env.NEIRA_BIND_ADDR);
    return;
  }
  const child = spawn("cargo", ["run", "--manifest-path", "backend/Cargo.toml"], {
    stdio: "inherit",
    env,
  });
  child.on("exit", (code) => process.exit(code));
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main();
}
