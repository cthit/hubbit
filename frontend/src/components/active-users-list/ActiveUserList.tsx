import React, { useEffect, useMemo, useState } from 'react';

import { gql } from '@urql/core';

import { ActiveUserFragment } from '../../__generated__/graphql';
import { getHoursDiff } from '../../dateUtil';
import { formatNick } from '../../util';

import styles from './ActiveUserList.module.scss';

export const ACTIVE_USER_FRAGMENT = gql`
  fragment ActiveUser on ActiveSession {
    user {
      cid
      nick
    }
    startTime
  }
`;

interface Props {
  sessions: ActiveUserFragment[];
  loggedInUser: string;
}

const ActiveUserList = ({ sessions, loggedInUser }: Props) => {
  const [currTime, setCurrTime] = useState(new Date());

  useEffect(() => {
    setCurrTime(new Date());
    const interval = setInterval(() => {
      setCurrTime(new Date());
    }, 60 * 1000);

    return () => {
      clearInterval(interval);
    };
  }, [sessions]);

  const sortedSessions = useMemo(
    () =>
      sessions.sort((left, right) => {
        const leftTime = new Date(left.startTime).getTime();
        const rightTime = new Date(right.startTime).getTime();

        if (leftTime === rightTime) {
          const leftNick = formatNick(left.user.cid, left.user.nick);
          const rightNick = formatNick(right.user.cid, right.user.nick);
          return leftNick.localeCompare(rightNick);
        }

        return leftTime - rightTime;
      }),
    [sessions],
  );

  return (
    <div className={styles.activeSmurfsWrapper}>
      <div>
        There are {sessions.length} smurfs in the Hubb right now!
        <table className={'data-table card-shadow ' + styles.activeSmurfsTable}>
          <thead>
            <tr className="header-row">
              <th className={styles.userRow}>User</th>
              <th className={styles.statusRow}>Current Status</th>
            </tr>
          </thead>
          <tbody>
            {sortedSessions.map(session => {
              const startTime = new Date(session.startTime);

              return (
                <tr
                  key={session.user.cid}
                  id={session.user.cid}
                  className={`data-table-row ${session.user.cid === loggedInUser && 'active-row'}`}
                >
                  <td className={styles.userRow}>
                    <a href={`/users/${session.user.cid}`}>{formatNick(session.user.cid, session.user.nick)}</a>
                  </td>
                  <td className={styles.timeCell}>
                    {formatTime(startTime)}
                    <time>{getHoursDiff(startTime, currTime)}</time>
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

function formatTime(time: Date): string {
  return `Since ${String(time.getHours()).padStart(2, '0')}:${String(time.getMinutes()).padStart(2, '0')} `;
}

export default ActiveUserList;
