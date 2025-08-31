/* neira:meta
id: NEI-20250930-backend-dev-test
intent: test
summary: Проверяет автоматический подбор свободного порта в backend-dev.
*/
import { createServer } from 'net';
import { execFile } from 'child_process';
import { promisify } from 'util';

const exec = promisify(execFile);

test('choosePort returns requested port if free', async () => {
  const { stdout } = await exec('node', ['scripts/backend-dev.mjs', '--port', '5200', '--dry-run']);
  expect(stdout.trim()).toBe('0.0.0.0:5200');
});

test('choosePort skips occupied port', async () => {
  const srv = createServer().listen(5300);
  const { stdout } = await exec('node', ['scripts/backend-dev.mjs', '--port', '5300', '--dry-run']);
  expect(stdout.trim()).not.toBe('0.0.0.0:5300');
  srv.close();
});
