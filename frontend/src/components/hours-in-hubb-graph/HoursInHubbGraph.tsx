import React from 'react';

import { gql } from '@urql/core';
import { Area, AreaChart, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis } from 'recharts';

import { UserHourStatsFragment } from '../../__generated__/graphql';

import styles from './HoursInHubbGraph.module.scss';

export const USER_HOUR_STATS_FRAGMENT = gql`
  fragment UserHourStats on User {
    hourStats
  }
`;

interface Props {
  user: UserHourStatsFragment;
}

const HoursInHubbGraph = ({ user }: Props) => {
  const hourStats = Array.from({ length: 25 }, (_, i) => i).map(hour => {
    return parseFloat((user.hourStats[hour % 24] / 60).toFixed(1));
  });

  const maxHours = hourStats.reduce((p, c) => Math.max(p, c), 0);
  const totalHours = hourStats.reduce((p, c) => p + c, 0);

  return (
    <div className={styles.graphContainer}>
      <div className={styles.graphHeader}>Hour stats</div>
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
  );
};

export default HoursInHubbGraph;

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
