/* neira:meta
id: NEI-20250101-000003-generate-node-docs
intent: docs
summary: |
  Generate markdown listing organs with their node identifiers.
*/
import { promises as fs } from 'node:fs';
import path from 'node:path';

async function collectJsonFiles(dir) {
  const dirents = await fs.readdir(dir, { withFileTypes: true });
  const files = await Promise.all(
    dirents.map((dirent) => {
      const res = path.resolve(dir, dirent.name);
      if (dirent.isDirectory()) return collectJsonFiles(res);
      if (dirent.isFile() && dirent.name.startsWith('organ.') && dirent.name.endsWith('.json'))
        return [res];
      return [];
    })
  );
  return files.flat();
}

async function main() {
  const files = await collectJsonFiles('examples');
  const lines = [
    '<!-- neira:meta',
    'id: NEI-20250101-000005-node-ids-doc',
    'intent: docs',
    'summary: |',
    '  Node identifiers generated from organ specs.',
    '-->',
    '',
    '# Node IDs',
    ''
  ];
  for (const file of files) {
    const data = JSON.parse(await fs.readFile(file, 'utf8'));
    lines.push(`## ${data.id}`, '');
    for (const node of data.nodes ?? []) {
      lines.push(`- ${node}`);
    }
    lines.push('');
  }
  await fs.writeFile('docs/node-ids.md', lines.join('\n'));
}

main();
