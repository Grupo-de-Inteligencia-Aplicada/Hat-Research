export function stripLines(s: string) {
  return s.split('\n').map(l => l.trim()).join('\n');
}

export function validateTime(time: string): string | null {

  const parts = time.trim().split(':').map(s => s.trim());

  if (parts.length < 1 || parts[0].length == 0) {
    return null;
  }

  let hrs = parseInt(parts[0]);

  if (hrs < 0 || hrs > 23 || isNaN(hrs)) {
    hrs = 0;
  }

  let mins = 0;

  if (parts.length > 1) {
    mins = parseInt(parts[1]);
    if (mins < 0 || mins > 59 || isNaN(mins)) {
      mins = 0;
    }
  }

  let secs = 0;

  if (parts.length > 2) {
    secs = parseInt(parts[2]);
    if (secs < 0 || secs > 59 || isNaN(secs)) {
      secs = 0;
    }
  }

  return `${hrs.toString().padStart(2, '0')}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;

}
