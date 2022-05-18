import React from 'react';

import { gql } from '@urql/core';
import dateFormat from 'dateformat';

import { UserRecentSessionsFragment } from '../../__generated__/graphql';
import { getHoursDiff } from '../../dateUtil';

import styles from './UserRecentSessionsList.module.scss';

export const USER_RECENT_SESSIONS_FRAGMENT = gql`
  fragment UserRecentSessions on Session {
    endTime
    startTime
  }
`;

interface Props {
  recentSessions: UserRecentSessionsFragment[];
}

const UserRecentSessionsList = ({ recentSessions }: Props) => {
  const year = new Date().getFullYear();

  return (
    <div className={styles.recentSessionsContainer}>
      <div className={styles.recentSessionsInnerContainer}>
        <table className={'data-table card-shadow'}>
          <thead>
            <tr className={'header-row'}>
              <th>Recent Session</th>
            </tr>
          </thead>
          <tbody>
            {recentSessions.map((session, index) => {
              const startTime = new Date(session.startTime);
              const endTime = new Date(session.endTime);
              const fmt = startTime.getFullYear() == year ? 'dddd d mmmm HH:MM' : 'dddd d mmmm HH:MM yyyy';

              return (
                <tr key={index}>
                  <td>
                    {dateFormat(startTime, fmt)}
                    <br />
                    for about {getHoursDiff(startTime, endTime)}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default UserRecentSessionsList;
