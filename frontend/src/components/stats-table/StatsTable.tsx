import React from 'react';

import { gql } from '@urql/core';
import Link from 'next/link';

import {
  Maybe,
  StatsTableActiveFragment,
  StatsTableMeFragment,
  StatsTableStatFragment,
} from '../../__generated__/graphql';
import { formatNick } from '../../util';
import styles from "./StatsTable.module.scss";
import Image from 'next/image';

interface Props {
  stats: StatsTableStatFragment[];
  me: StatsTableMeFragment;
  hideChange?: boolean;
  currentActive: StatsTableActiveFragment[];
}

export const STATS_TABLE_STAT_FRAGMENT = gql`
  fragment StatsTableStat on Stat {
    currentPosition
    durationSeconds
    prevPosition
    user {
      cid
      nick
    }
  }
`;

export const STATS_TABLE_ME_FRAGMENT = gql`
  fragment StatsTableMe on User {
    cid
  }
`;

export const STATS_TABLE_ACTIVE_NOW_FRAGMENT = gql`
  fragment StatsTableActive on ActiveSession {
    user {
      cid
    }
  }
`;

const StatsTable = ({ stats, me, hideChange = false, currentActive }: Props) => {
  const active = currentActive.map(a => a.user.cid);

  return (
    <div>
      <a href={`#${me.cid}`}>Find me!</a>
      <table className="data-table card-shadow">
        <thead>
          <tr className="header-row">
            <th className="active-now-header" />
            {!hideChange && <th>Change</th>}
            <th className="position-column">#</th>
            <th className="name-column">Name</th>
            <th>Total time</th>
          </tr>
        </thead>
        <tbody>
          {stats.map((stat, index) => {
            const nick = formatNick(stat.user.cid, stat.user.nick);

            return (
              <tr
                key={stat.user.cid}
                id={stat.user.cid}
                className={`data-table-row ${stat.user.cid === me.cid ? 'active-row' : ''}`}
              >
                <td
                  className={`${
                    active.includes(stat.user.cid) ? 'active-now-col' : 'inactive-now-col'
                  } active-now-header`}
                />
                {!hideChange && (
                  <td>
                    <Image
                      title={getChangeTitle(stat.currentPosition, stat.prevPosition)}
                      src={getChangeImageName(stat.currentPosition, stat.prevPosition)}
                      alt="position change icon"
                      className={styles.changeIcon}
                      width={16}
                      height={16}
                    />
                  </td>
                )}
                <td className="position-column">{index + 1}</td>
                <td className="name-column">
                  <Link href={`/users/${stat.user.cid}`}>{nick}</Link>
                </td>
                <td>{convertSecondsToString(stat.durationSeconds)}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

function getChangeTitle(currPosition: number, prevPosition: Maybe<number> | undefined): string {
  if (!prevPosition) {
    return 'No data from previous period';
  }

  const change = prevPosition - currPosition;

  if (change === 0) {
    return 'Unchanged from previous period';
  }

  return `${change > 0 ? 'Up' : 'Down'} ${Math.abs(change)} steps from ${prevPosition}`;
}

function getChangeImageName(currPosition: number, prevPosition: Maybe<number> | undefined): string {
  if (!prevPosition) {
    return '/up-arrow.svg';
  }

  const change = prevPosition - currPosition;

  if (change > 0) {
    return '/up-arrow.svg';
  }
  if (change < 0) {
    return '/down-arrow.svg';
  }
  return '/dash.svg';
}

function convertSecondsToString(totalSeconds: number): string {
  const seconds = totalSeconds % 60;
  const minutes = Math.floor((totalSeconds / 60) % 60);
  const hours = Math.floor(totalSeconds / 3600);

  return `${numToStr(hours)}:${numToStr(minutes)}:${numToStr(seconds)}`;
}

function numToStr(num: number): string {
  return num.toString().padStart(2, '0');
}

export default StatsTable;
