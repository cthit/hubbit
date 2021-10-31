export interface DateDiff {
  daysSince: number;
  hoursSince: number;
  minutesSince: number;
  secondsSince: number;
}

const ONE_SECOND = 1000;
const SECONDS_PER_MINUTE = 60;
const MINUTES_PER_HOUR = 60;
const HOURS_PER_DAY = 24;

const now = new Date(Date.now());

export function dateDiffToAgoString(dateDiff: DateDiff) {
  if (dateDiff.daysSince === 0 && dateDiff.hoursSince === 0 && dateDiff.minutesSince === 0) {
    return 'Just now';
  }

  return `${dateDiffToString(dateDiff)} ago`;
}

export function dateDiffToString(dateDiff: DateDiff) {
  return dateDiff.daysSince > 0
    ? `${dateDiff.daysSince} day${dateDiff.daysSince > 1 ? 's' : ''}`
    : dateDiff.hoursSince > 0
    ? `${dateDiff.hoursSince} hour${dateDiff.hoursSince > 1 ? 's' : ''}`
    : dateDiff.minutesSince > 0
    ? `${dateDiff.minutesSince} minute${dateDiff.minutesSince === 1 ? '' : 's'}`
    : `${dateDiff.secondsSince} second${dateDiff.secondsSince === 1 ? '' : 's'}`;
}

export function timeSince(date: Date): DateDiff {
  return timeBetween(date, now);
}

export function timeBetween(dateA: Date, dateB: Date): DateDiff {
  const milliSecDiff = Math.abs(dateA.getTime() - dateB.getTime());
  const secondsAgo = Math.round(milliSecDiff / ONE_SECOND);
  const minutesAgo = Math.round(secondsAgo / SECONDS_PER_MINUTE);
  const hoursAgo = Math.round(minutesAgo / MINUTES_PER_HOUR);
  const daysAgo = (hoursAgo - (hoursAgo % HOURS_PER_DAY)) / HOURS_PER_DAY;

  return {
    daysSince: daysAgo,
    hoursSince: hoursAgo,
    minutesSince: minutesAgo,
    secondsSince: secondsAgo,
  };
}

export function isToday(date: Date): boolean {
  return (
    date.getUTCFullYear() === now.getUTCFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getUTCDate() === now.getUTCDate()
  );
}

export function formatDate(date: Date): string {
  return `${date.getDate()} ${date.toLocaleString('default', {
    month: 'short',
  })} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
}

export function prettyFromSeconds(seconds: number) {
  let pretty = `${seconds % SECONDS_PER_MINUTE}s`;
  if (seconds >= SECONDS_PER_MINUTE) {
    const minutes = Math.floor(seconds / SECONDS_PER_MINUTE) % SECONDS_PER_MINUTE;
    pretty = `${minutes}m ${pretty}`;
  }

  if (seconds >= SECONDS_PER_MINUTE * MINUTES_PER_HOUR) {
    const hours = Math.floor(seconds / SECONDS_PER_MINUTE / MINUTES_PER_HOUR) % HOURS_PER_DAY;
    pretty = `${hours}h ${pretty}`;
  }

  if (seconds >= SECONDS_PER_MINUTE * MINUTES_PER_HOUR * HOURS_PER_DAY) {
    const days = Math.floor(seconds / SECONDS_PER_MINUTE / MINUTES_PER_HOUR / HOURS_PER_DAY);
    pretty = `${days}d ${pretty}`;
  }

  return pretty;
}

export function getHoursDiff(a: Date, b: Date): string {
  const diffTime = Math.abs(a.getTime() - b.getTime());
  const diffHours = Math.floor(diffTime / (MINUTES_PER_HOUR * SECONDS_PER_MINUTE * ONE_SECOND));

  if (diffHours >= 1) {
    return `(${diffHours} hour${diffHours === 1 ? '' : 's'})`;
  }

  const minutes = Math.floor(diffTime / (SECONDS_PER_MINUTE * ONE_SECOND));
  if (minutes < 1) {
    return 'Just now';
  }
  return `(${minutes} minute${minutes === 1 ? '' : 's'})`;
}

export function secondsToMinutesOrHours(seconds: number) {
  if (seconds < SECONDS_PER_MINUTE) {
    return `${seconds} seconds`;
  }

  const minutes = Math.floor(seconds / SECONDS_PER_MINUTE);

  if (minutes < MINUTES_PER_HOUR) {
    return `${minutes} minutes`;
  }

  const hours = Math.floor(minutes / MINUTES_PER_HOUR);
  return `${hours} hours`;
}

export function prettyHoursFromSeconds(seconds: number) {
  let pretty = `${seconds % SECONDS_PER_MINUTE}s`;
  if (seconds >= SECONDS_PER_MINUTE) {
    const minutes = Math.floor(seconds / SECONDS_PER_MINUTE) % SECONDS_PER_MINUTE;
    pretty = `${minutes}m ${pretty}`;
  }

  if (seconds >= SECONDS_PER_MINUTE * MINUTES_PER_HOUR) {
    const hours = Math.floor(seconds / SECONDS_PER_MINUTE / MINUTES_PER_HOUR);
    pretty = `${hours}h ${pretty}`;
  }

  return pretty;
}
