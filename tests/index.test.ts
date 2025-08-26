import { hello } from '../src/index';

test('hello returns greeting', () => {
  expect(hello()).toBe('Hello, Neira!');
});
