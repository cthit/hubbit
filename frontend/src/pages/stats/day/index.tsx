import React from 'react';

import { gql } from '@urql/core';
import { GetServerSidePropsContext, NextPage } from 'next';
import Head from 'next/head';
import { useRouter } from 'next/router';

import { StatsDayQuery } from '../../../__generated__/graphql';
import Error from '../../../components/error/Error';
import { DAY, StatsNavigation } from '../../../components/stats-navigation/StatsNavigation';
import StatsTable, {
  STATS_TABLE_ACTIVE_NOW_FRAGMENT,
  STATS_TABLE_ME_FRAGMENT,
  STATS_TABLE_STAT_FRAGMENT,
} from '../../../components/stats-table/StatsTable';
import { StatsTimespanSelect } from '../../../components/stats-timespan-select/StatsTimespanSelect';
import { createTitle, defaultGetServerSideProps, PageProps } from '../../../util';

const STATS_DAY_QUERY = gql`
  query StatsDay($input: StatsDayInput) {
    statsDay(input: $input) {
      stats {
        ...StatsTableStat
      }
      curr {
        year
        month
        day
      }
      next {
        year
        month
        day
      }
      prev {
        year
        month
        day
      }
    }
    me {
      ...StatsTableMe
    }
    currentSessions {
      ...StatsTableActive
    }
  }

  ${STATS_TABLE_STAT_FRAGMENT}
  ${STATS_TABLE_ME_FRAGMENT}
  ${STATS_TABLE_ACTIVE_NOW_FRAGMENT}
`;

const StatsDay: NextPage<PageProps<StatsDayQuery>> = ({ data }) => {
  const { pathname: path } = useRouter();

  if (!data) {
    return <Error />;
  }

  const date = new Date(data.statsDay.curr.year, data.statsDay.curr.month, data.statsDay.curr.day);
  const formattedDate = date.toLocaleString('default', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });

  return (
    <>
      <Head>
        <title>{createTitle(`Stats for ${formattedDate}`)}</title>
      </Head>
      <div className={'statsWrapper'}>
        <StatsNavigation activeFrame={DAY} />
        <StatsTimespanSelect
          current={`${data.statsDay.curr.year}-${data.statsDay.curr.month
            .toString()
            .padStart(2, '0')}-${data.statsDay.curr.day.toString().padStart(2, '0')}`}
          prev={`${path}?year=${data.statsDay.prev.year}&month=${data.statsDay.prev.month}&day=${data.statsDay.prev.day}`}
          next={`${path}?year=${data.statsDay.next.year}&month=${data.statsDay.next.month}&day=${data.statsDay.next.day}`}
        />
        <StatsTable stats={data.statsDay.stats} me={data.me} currentActive={data.currentSessions} />
      </div>
    </>
  );
};

export default StatsDay;

function getInputProps(context: GetServerSidePropsContext) {
  let year = NaN;
  const yearString = context.query['year'];
  if (yearString) {
    year = parseInt(yearString.toString(), 10);
  }

  let month = NaN;
  const monthString = context.query['month'];
  if (monthString) {
    month = parseInt(monthString.toString(), 10);
  }

  let day = NaN;
  const dayString = context.query['day'];
  if (dayString) {
    day = parseInt(dayString.toString(), 10);
  }

  if (isNaN(year) || isNaN(month) || isNaN(day)) {
    return {};
  }

  return {
    input: {
      year: year,
      month: month,
      day: day,
    },
  };
}

export const getServerSideProps = defaultGetServerSideProps<StatsDayQuery>(STATS_DAY_QUERY, getInputProps);
