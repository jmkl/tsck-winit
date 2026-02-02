/*
 *
 * map range for piprange tricolor
 * from 720 into 4096
 *
 */
export function mapTo4096(value: number) {
  return Math.round((value / 720) * 4096);
}
