import React from 'react';

import gql from 'graphql-tag';

import { UserRecentSessionsFragment } from '../../__generated__/graphql';
import { getHoursDiff } from '../../dateUtil';

import styles from './UserRecentSessionsList.module.scss';

export const USER_RECENT_SESSIONS_FRAGMENT = gql`
  fragment UserRecentSessions on User {
    recentSessions {
      endTime
      startTime
    }
  }
`;

interface Props {
  user: UserRecentSessionsFragment;
}

const UserRecentSessionsList = ({ user }: Props) => {
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
            {user.recentSessions.map((session, index) => {
              const startTime = new Date(session.startTime);
              const endTime = new Date(session.endTime);
              return (
                <tr key={index}>
                  <td>
                    {formatDate(startTime)}
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

export function formatDate(startTime: Date): string {
  return `${startTime.toLocaleDateString('default', {
    weekday: 'long',
  })} ${startTime.getDate()}
   ${startTime.toLocaleDateString('default', {
     month: 'long',
   })} ${startTime.getFullYear()}`;
}

export default UserRecentSessionsList;
