import {
  buildOrganismNarrative,
  computeLoadState,
  formatDuration,
  normalizeControlStatus,
  totalQueueDepth,
  type ControlStatus,
} from '../sensory_organs/src/status-utils';

describe('status-utils', () => {
  let warnSpy: jest.SpyInstance;

  beforeAll(() => {
    warnSpy = jest.spyOn(console, 'warn').mockImplementation(() => {});
  });

  afterAll(() => {
    warnSpy.mockRestore();
  });

  const baseStatus: ControlStatus = {
    paused: false,
    paused_for_ms: 0,
    paused_since_ts_ms: Date.now(),
    reason: '',
    active_tasks: 2,
    backpressure: 5,
    queues: { fast: 1, standard: 2, long: 0 },
  };

  test('normalizeControlStatus tolerates missing numbers', () => {
    const normalized = normalizeControlStatus({
      paused: true,
      paused_for_ms: '1500',
      paused_since_ts_ms: undefined,
      reason: 'maintenance',
      active_tasks: '3',
      backpressure: null,
      queues: { fast: '2', standard: '4', long: undefined },
    });

    expect(normalized.paused).toBe(true);
    expect(normalized.paused_for_ms).toBe(1500);
    expect(normalized.active_tasks).toBe(3);
    expect(normalized.backpressure).toBe(0);
    expect(normalized.queues.standard).toBe(4);
  });

  test('formatDuration renders human-readable strings', () => {
    expect(formatDuration(0)).toBe('0 сек');
    expect(formatDuration(15_000)).toBe('15 сек');
    expect(formatDuration(90_000)).toBe('1 мин 30 сек');
    expect(formatDuration(3_600_000)).toBe('1 ч');
    expect(formatDuration(27_000_000)).toBe('7 ч 30 мин');
  });

  test('computeLoadState reacts to queue depth and pause', () => {
    expect(computeLoadState(baseStatus)).toBe('calm');
    expect(
      computeLoadState({
        ...baseStatus,
        backpressure: 80,
        queues: { fast: 20, standard: 15, long: 10 },
      }),
    ).toBe('focused');
    expect(
      computeLoadState({
        ...baseStatus,
        paused: true,
        reason: 'safety',
      }),
    ).toBe('paused');
    expect(
      computeLoadState({
        ...baseStatus,
        backpressure: 500,
        queues: { fast: 200, standard: 90, long: 10 },
      }),
    ).toBe('stressed');
  });

  test('buildOrganismNarrative emphasises organism nature', () => {
    const calm = buildOrganismNarrative(baseStatus);
    expect(calm.headline).toContain('Организм');
    expect(calm.detail).toContain('организм');

    const paused = buildOrganismNarrative({ ...baseStatus, paused: true, paused_for_ms: 5_000, reason: 'update' });
    expect(paused.headline).toContain('паузе');
    expect(paused.detail).toContain('Пауза длится');
  });

  test('totalQueueDepth sums only positive values', () => {
    expect(totalQueueDepth({ ...baseStatus, queues: { fast: 2, standard: 3, long: 4 } })).toBe(9);
    expect(totalQueueDepth({ ...baseStatus, queues: { fast: -5, standard: 3, long: 0 } })).toBe(3);
  });
});
