import React from 'react';

import { gql } from '@urql/core';
import dateFormat from 'dateformat';

import { UserStatsFragment, UserStatsQuery } from '../../__generated__/graphql';
import {
  dateDiffToAgoString,
  dateDiffToString,
  prettyFromSeconds,
  secondsToMinutesOrHours,
  timeBetween,
  timeSince,
} from '../../dateUtil';
import UserStatsCard from '../UserStatsCard/UserStatsCard';

import styles from './UserStatsCards.module.scss';

export const USER_STATS_FRAGMENT = gql`
  fragment UserStats on User {
    longestSession {
      startTime
      endTime
    }
    recentSessions {
      startTime
      endTime
    }
    averageTimePerDay
    totalTimeSeconds
    timeTodaySeconds
  }
`;

interface Props {
  user: UserStatsFragment;
}

const UserStatsCards = ({ user }: Props) => {
  let longestSessionSeconds = 0;
  if (user.longestSession) {
    const { startTime, endTime } = user.longestSession;
    const start = new Date(startTime);
    const end = new Date(endTime);
    longestSessionSeconds = (end.getTime() - start.getTime()) / 1000;
  }

  return (
    <div className={styles.userStatsCardsWrapper}>
      <UserStatsCard title="Last session" content={getLastSessionText(user.recentSessions)} />
      <UserStatsCard title="Today" content={getTodayText(user.timeTodaySeconds)} />
      <UserStatsCard title="Average time per day" content={prettyFromSeconds(user.averageTimePerDay)} />
      <UserStatsCard title="Total time" content={prettyFromSeconds(user.totalTimeSeconds)} />
      <UserStatsCard title="Longest session" content={prettyFromSeconds(longestSessionSeconds)} />
    </div>
  );
};

function getLastSessionText(recentSessions: UserStatsQuery['user']['recentSessions']): any {
  if (recentSessions.length === 0) {
    return 'Never been seen in the Hubb! :o';
  }

  const lastSession = recentSessions[0];
  const lastSessionStartTime = new Date(lastSession.startTime);
  const lastSessionEndTime = new Date(lastSession.endTime);
  const timeSinceStr = dateDiffToAgoString(timeSince(lastSessionEndTime));
  const dateStr = dateFormat(lastSessionEndTime, 'd mmm HH:MM');

  return (
    <>
      {timeSinceStr}
      <br />
      {dateStr}
      <br />
      {`For about ${dateDiffToString(timeBetween(lastSessionStartTime, lastSessionEndTime))}`}
    </>
  );
}

function getTodayText(totalSecondsToday: number): any {
  if (totalSecondsToday <= 0) {
    return `Not seen today`;
  }

  return `For about ${secondsToMinutesOrHours(totalSecondsToday)}`;
}

export default UserStatsCards;
