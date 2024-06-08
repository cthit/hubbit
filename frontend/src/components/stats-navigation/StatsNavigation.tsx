import React from 'react';

import Link from 'next/link';

import styles from '../../pages/stats/index.module.scss';

export const ALL_TIME = 'all-time';
export const STUDY_YEAR = 'study-year';
export const STUDY_PERIOD = 'study-period';
export const MONTH = 'month';
export const WEEK = 'week';
export const DAY = 'day';

export type StatsTab =
  | typeof ALL_TIME
  | typeof STUDY_YEAR
  | typeof STUDY_PERIOD
  | typeof MONTH
  | typeof WEEK
  | typeof DAY;

interface Props {
  activeFrame: StatsTab;
}

export const StatsNavigation = ({ activeFrame }: Props) => {
  return (
    <ul className={styles.inlineList}>
      <li className={activeFrame === ALL_TIME ? styles.selected : ''}>
        <Link href={getLink(ALL_TIME)}>All time</Link>
      </li>
      <li className={activeFrame === STUDY_YEAR ? styles.selected : ''}>
        <Link href={getLink(STUDY_YEAR)}>Study year</Link>
      </li>
      <li className={activeFrame === STUDY_PERIOD ? styles.selected : ''}>
        <Link href={getLink(STUDY_PERIOD)}>Study Period</Link>
      </li>
      <li className={activeFrame === MONTH ? styles.selected : ''}>
        <Link href={getLink(MONTH)}>Month</Link>
      </li>
      <li className={activeFrame === WEEK ? styles.selected : ''}>
        <Link href={getLink(WEEK)}>Week</Link>
      </li>
      <li className={activeFrame === DAY ? styles.selected : ''}>
        <Link href={getLink(DAY)}>Day</Link>
      </li>
    </ul>
  );
};

function getLink(stat: StatsTab): string {
  return `/stats/${stat}`;
}
