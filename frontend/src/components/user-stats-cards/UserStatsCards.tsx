import React, { ReactElement } from 'react';

import { gql } from '@urql/core';
import { Area, AreaChart, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis } from 'recharts';

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

  const hourStats = Array.from({ length: 25 }, (_, i) => i).map(hour => {
    return parseFloat((user.hourStats[hour % 24] / 60).toFixed(1));
  });
  const maxHours = hourStats.reduce((p, c) => Math.max(p, c), 0);
  const totalHours = hourStats.reduce((p, c) => p + c, 0);

  return (
    <>
      <div className={styles.userStatsCardsWrapper}>
        <UserStatsCard title="Last session" content={getLastSessionText(user.recentSessions)} />
        <UserStatsCard title="Today" content={getTodayText(user.timeTodaySeconds)} />
        <UserStatsCard
          title="Total time"
          content={
            <>
              {prettyFromSeconds(user.totalTimeSeconds)}
              <br />
              {`(${prettyHoursFromSeconds(user.totalTimeSeconds)})`}
            </>
          }
        />
        <UserStatsCard title="Longest session" content={prettyFromSeconds(longestSessionSeconds)} />
      </div>
      <div className={styles.graphContainer}>
        <h2 className={styles.graphHeader}>Hour stats</h2>
        <div className={styles.graphContent}>
          <ResponsiveContainer aspect={2} maxHeight={500}>
            <AreaChart
              data={hourStats.map((hours, index) => {
                return {
                  hour: index,
                  hours,
                  percentage: hours / totalHours,
                };
              })}
              margin={{
                top: 0,
                right: 1,
                left: 0,
                bottom: 0,
              }}
            >
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="hour" />
              <YAxis domain={[0, Math.ceil((maxHours * 1.2) / 5) * 5]} />
              <Tooltip content={<CustomTooltip />} />
              <Area type="monotone" dataKey="hours" stroke="#389127" fill="#68d157" dot={true} />
            </AreaChart>
          </ResponsiveContainer>
        </div>
      </div>
    </>
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

const CustomTooltip = ({ active, payload }: any) => {
  if (active) {
    const p = payload[0].payload;
    return (
      <div style={{ background: 'white', padding: '1px 20px', margin: '0', border: '1px solid lightgrey' }}>
        <p>Hour: {p.hour}</p>
        <p>
          Hours: {p.hours} ({(100 * p.percentage).toFixed(1)}%)
        </p>
      </div>
    );
  }

  return null;
};

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
