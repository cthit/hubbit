import React from 'react';

import { gql } from '@urql/core';
import { NextPage } from 'next';
import Head from 'next/head';
import Link from 'next/link';

import { MeCidQueryQuery, UserStatsQuery, UserStatsQueryVariables } from '../../__generated__/graphql';
import Error from '../../components/error/Error';
import HoursInHubbGraph, { USER_HOUR_STATS_FRAGMENT } from '../../components/hours-in-hubb-graph/HoursInHubbGraph';
import UserRecentSessionsList, {
  USER_RECENT_SESSIONS_FRAGMENT,
} from '../../components/user-last-sessions-list/UserRecentSessionsList';
import UserStatsCards, { USER_STATS_FRAGMENT } from '../../components/user-stats-cards/UserStatsCards';
import { createTitle, defaultGetServerSideProps, formatNick, PageProps } from '../../util';

import styles from './[cid].module.scss';

const USER_STATS_QUERY = gql`
  query UserStats($input: UserUniqueInput!) {
    user(input: $input) {
      ...UserStats
      ...UserHourStats
      ...UserRecentSessions

      nick
      cid

      currAlltimePosition
      currStudyYearPosition
    }
  }

  ${USER_STATS_FRAGMENT}
  ${USER_HOUR_STATS_FRAGMENT}
  ${USER_RECENT_SESSIONS_FRAGMENT}
`;

const UserStats: NextPage<PageProps<UserStatsQuery>> = ({ data }) => {
  if (!data) {
    return <Error />;
  }

  return (
    <>
      <Head>
        <title>{createTitle(formatNick(data.user.cid, data.user.nick))}</title>
      </Head>
      <div className={styles.showSection}>
        <h1>{data.user.nick}</h1>
        <Link href={`/stats/study-year#${data.user.cid}`}>
          <a>Study Year Position: {data.user.currStudyYearPosition}</a>
        </Link>
        <br />
        <Link href={`/stats/all-time#${data.user.cid}`}>
          <a>All Time Position: {data.user.currAlltimePosition}</a>
        </Link>
        <div className={styles.showSection}>
          <UserStatsCards user={data.user} />
          <HoursInHubbGraph user={data.user} />
          <UserRecentSessionsList user={data.user} />
        </div>
      </div>
    </>
  );
};

export default UserStats;

type Params = {
  cid: string;
};

export const getServerSideProps = defaultGetServerSideProps<UserStatsQuery, UserStatsQueryVariables, Params>(
  USER_STATS_QUERY,
  context => {
    const cid = context.params?.cid || '';
    return {
      input: {
        cid,
      },
    };
  },
);
