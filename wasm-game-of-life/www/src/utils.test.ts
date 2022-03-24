import { getBit } from './utils';

describe('getBit', () => {
  const array = new Uint8Array(64).fill(0b10101010);

  test('first element', () => {
    expect(getBit(0x0, array)).toBe(false);
    expect(getBit(0x1, array)).toBe(true);
    expect(getBit(0x2, array)).toBe(false);
    expect(getBit(0x3, array)).toBe(true);
    expect(getBit(0x4, array)).toBe(false);
    expect(getBit(0x5, array)).toBe(true);
    expect(getBit(0x6, array)).toBe(false);
    expect(getBit(0x7, array)).toBe(true);
  });

  test('second element', () => {
    expect(getBit(0x10, array)).toBe(false);
    expect(getBit(0x11, array)).toBe(true);
    expect(getBit(0x12, array)).toBe(false);
    expect(getBit(0x13, array)).toBe(true);
    expect(getBit(0x14, array)).toBe(false);
    expect(getBit(0x15, array)).toBe(true);
    expect(getBit(0x16, array)).toBe(false);
    expect(getBit(0x17, array)).toBe(true);
  });
});
