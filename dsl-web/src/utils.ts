export function stripLines(s: string) {
  return s.split('\n').map(l => l.trim()).join('\n');
}
