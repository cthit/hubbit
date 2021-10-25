import React, { ReactElement } from 'react';

import { gql } from '@urql/core';

import { UserStatsFragment, UserStatsQuery } from '../../__generated__/graphql';
import {
  dateDiffToAgoString,
  dateDiffToString,
  formatDate,
  prettyFromSeconds,
  prettyHoursFromSeconds,
  secondsToMinutesOrHours,
  timeBetween,
  timeSince,
} from '../../dateUtil';

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
    hourStats
    cid
    nick
    totalTimeSeconds
    longestSession {
      startTime
      endTime
    }
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
      <UserStatsCard title="Total time" content={prettyFromSeconds(user.totalTimeSeconds)} />
      <UserStatsCard title="Longest session" content={prettyFromSeconds(longestSessionSeconds)} />
    </div>
  );
};

interface UserStatCardProps {
  title: string;
  content: string | ReactElement;
}

const UserStatsCard = ({ title, content }: UserStatCardProps) => (
  <div className={styles.infoContainer}>
    <h2 className={styles.infoHeader}>{title}</h2>
    <div className={styles.infoText}>{content}</div>
  </div>
);

function getLastSessionText(recentSessions: UserStatsQuery['user']['recentSessions']): any {
  if (recentSessions.length === 0) {
    return 'Never been seen in the Hubb! :o';
  }

  const lastSession = recentSessions[0];
  const lastSessionStartTime = new Date(lastSession.startTime);
  const lastSessionEndTime = new Date(lastSession.endTime);
  const timeSinceStr = dateDiffToAgoString(timeSince(lastSessionEndTime));
  const dateStr = formatDate(lastSessionEndTime);

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
