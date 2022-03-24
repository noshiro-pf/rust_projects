export const getBit = (index: number, array: Uint8Array): boolean => {
  const byte = Math.floor(index / 8);
  const mask = 1 << index % 8;
  return ((array[byte] ?? 0) & mask) === mask;
};
