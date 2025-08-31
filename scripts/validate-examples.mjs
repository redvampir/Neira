/* neira:meta
id: NEI-20250101-000002-validate-examples
intent: utility
summary: |
  Validate example organ specs and Markdown docs.
  Checks URLs for корректность портов, поддерживает --watch режим.
*/
/* global console, process, URL */
import { promises as fs } from "node:fs";
import path from "node:path";
import Ajv from "ajv/dist/2020.js";
import chokidar from "chokidar";

async function collectJsonFiles(dir) {
  const dirents = await fs.readdir(dir, { withFileTypes: true });
  const files = await Promise.all(
    dirents.map((dirent) => {
      const res = path.resolve(dir, dirent.name);
      if (dirent.isDirectory()) return collectJsonFiles(res);
      if (
        dirent.isFile() &&
        dirent.name.startsWith("organ.") &&
        dirent.name.endsWith(".json")
      )
        return [res];
      return [];
    }),
  );
  return files.flat();
}

async function collectMarkdownFiles(dir) {
  const dirents = await fs.readdir(dir, { withFileTypes: true });
  const files = await Promise.all(
    dirents.map((dirent) => {
      const res = path.resolve(dir, dirent.name);
      if (dirent.isDirectory()) return collectMarkdownFiles(res);
      if (dirent.isFile() && dirent.name.endsWith(".md")) return [res];
      return [];
    }),
  );
  return files.flat();
}

function validateUrlPorts(obj) {
  function scan(value) {
    if (typeof value === "string" && /^https?:\/\//.test(value)) {
      let url;
      try {
        url = new URL(value);
      } catch {
        throw new Error(`Invalid URL: ${value}`);
      }
      if (url.port) {
        const portNum = Number(url.port);
        if (!Number.isInteger(portNum) || portNum < 1 || portNum > 65535) {
          throw new Error(`Invalid port in URL: ${value}`);
        }
      }
    } else if (Array.isArray(value)) {
      value.forEach(scan);
    } else if (value && typeof value === "object") {
      Object.values(value).forEach(scan);
    }
  }
  scan(obj);
}

function validateMarkdownPorts(text, file) {
  const re = /https?:\/\/[^\s)]+/g;
  for (const match of text.matchAll(re)) {
    const raw = match[0].replace(/[`,;'">\]]+$/, "");
    let url;
    try {
      url = new URL(raw);
    } catch {
      throw new Error(`Invalid URL in ${file}: ${raw}`);
    }
    if (url.port) {
      const portNum = Number(url.port);
      if (!Number.isInteger(portNum) || portNum < 1 || portNum > 65535) {
        throw new Error(`Invalid port in ${file}: ${raw}`);
      }
    }
  }
}

async function validateFile(validate, file) {
  const data = JSON.parse(await fs.readFile(file, "utf8"));
  if (!validate(data)) {
    console.error(`\u274c  ${file}`);
    console.error(validate.errors);
    return false;
  }
  try {
    validateUrlPorts(data);
  } catch (e) {
    console.error(`\u274c  ${file}`);
    console.error(e.message);
    return false;
  }
  console.log(`\u2705  ${file}`);
  return true;
}

async function main() {
  const schema = JSON.parse(
    await fs.readFile("schemas/organ-template.schema.json", "utf8"),
  );
  const ajv = new Ajv({ strict: false });
  const validate = ajv.compile(schema);

  async function runAll() {
    const files = await collectJsonFiles("examples");
    let ok = true;
    for (const file of files) {
      if (!(await validateFile(validate, file))) ok = false;
    }
    const mdfiles = await collectMarkdownFiles("docs");
    for (const file of mdfiles) {
      try {
        const text = await fs.readFile(file, "utf8");
        validateMarkdownPorts(text, file);
        console.log(`\u2705  ${file}`);
      } catch (e) {
        console.error(`\u274c  ${file}`);
        console.error(e.message);
        ok = false;
      }
    }
    if (!ok) process.exit(1);
  }

  if (process.argv.includes("--watch")) {
    await runAll();
    console.log("Watching example organs...");
    chokidar
      .watch("examples", { ignoreInitial: true })
      .on("add", (file) => {
        if (path.basename(file).startsWith("organ.") && file.endsWith(".json"))
          validateFile(validate, file);
      })
      .on("change", (file) => {
        if (path.basename(file).startsWith("organ.") && file.endsWith(".json"))
          validateFile(validate, file);
      });
  } else {
    await runAll();
  }
}

main();
