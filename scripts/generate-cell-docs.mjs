/* neira:meta
id: NEI-20250101-000003-generate-cell-docs
intent: docs
summary: |
  Generate markdown listing organs with their cell identifiers.
*/
import { promises as fs } from "node:fs";
import path from "node:path";

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

async function main() {
  const files = (await collectJsonFiles("examples")).sort();
  const lines = [
    "<!-- neira:meta",
    "id: NEI-20250101-000005-cell-ids-doc",
    "intent: docs",
    "summary: |",
    "  Cell identifiers generated from organ specs.",
    "-->",
    "",
    "# Cell IDs",
    "",
  ];
  for (const file of files) {
    const data = JSON.parse(await fs.readFile(file, "utf8"));
    lines.push(`## ${data.id}`, "");
    for (const cell of data.cells ?? []) {
      lines.push(`- ${cell}`);
    }
    lines.push("");
  }
  await fs.writeFile("docs/cell-ids.md", lines.join("\n"));
}

main();
