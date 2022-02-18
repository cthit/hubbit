import React from 'react';

import { gql } from '@urql/core';

import { ActiveGroupFragment } from '../../__generated__/graphql';
import { formatNick } from '../../util';

import styles from './ActiveGroupList.module.scss';

export const ACTIVE_GROUP_FRAGMENT = gql`
  fragment ActiveGroup on ActiveSession {
    user {
      cid
      nick
      groups {
        name
        prettyName
      }
    }
  }
`;

interface Props {
  sessions: ActiveGroupFragment[];
}

interface GroupUsers {
  prettyName: string;
  users: ActiveGroupFragment['user'][];
}

const ActiveGroupList = ({ sessions }: Props) => {
  const groupsMap = new Map<string, GroupUsers>();
  sessions.forEach(session => {
    session.user.groups.forEach(group => {
      const groupUsers = groupsMap.get(group.name);
      const user = {
        ...session.user,
        nick: formatNick(session.user.cid, session.user.nick),
      };
      if (groupUsers) {
        groupsMap.set(group.name, { ...groupUsers, users: [...groupUsers.users, user] });
      } else {
        groupsMap.set(group.name, {
          prettyName: group.prettyName,
          users: [user],
        });
      }
    });
  });
  const groups = Array.from(groupsMap)
    .map(([group, groupUsers]) => {
      return {
        name: group,
        prettyName: groupUsers.prettyName,
        users: groupUsers.users.sort((left, right) => left.nick.localeCompare(right.nick)),
      };
    })
    .sort((left, right) => left.name.localeCompare(right.name));

  return (
    <div className={styles.activeGroupsContainer}>
      {groups.map(group => (
        <div key={group.name} className={styles.groupBoxContainer}>
          {/*TODO(vidarm): Rewrite without table */}
          <table className="data-table card-shadow">
            <tbody>
              <tr className="header-row" id={group.name}>
                <th>{group.name == 'hookit' ? '⚔️WANTED🍺' : group.prettyName}</th>
              </tr>
              {group.users.map(user => (
                <tr key={user.cid} className="data-table-row">
                  <td className={styles.userRow}>
                    <a href={`/users/${user.cid}`}>{user.nick}</a>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      ))}
    </div>
  );
};

export default ActiveGroupList;
