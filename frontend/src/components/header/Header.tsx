import React from 'react';

import Link from 'next/link';
import { useRouter } from 'next/router';

import { useAuth } from '../../contexts/auth';

import styles from './Header.module.scss';

const MAIN_ENDPOINT = '/';
const DEVICES_ENDPOINT = '/devices';
const STATS_BASE_ENDPOINT = '/stats/';
const MY_STATS_BASE_ENDPOINT = '/users/';

const Header = () => {
  const { asPath } = useRouter();
  const { user } = useAuth();

  const MY_STATS_URL = `${MY_STATS_BASE_ENDPOINT}${user.cid}`;

  return (
    <header className={styles.hContainer}>
      <h1>
        <Link className={styles.title} href="/">
          Who is in the Hubb?
        </Link>
      </h1>
      <nav>
        <ul className={styles.menu}>
          <li className={asPath === DEVICES_ENDPOINT ? styles.active : ''}>
            <Link href={DEVICES_ENDPOINT}>DEVICES</Link>
          </li>
          <li className={asPath === MAIN_ENDPOINT ? styles.active : ''}>
            <Link href={MAIN_ENDPOINT}>SMURFS IN THE HUBB</Link>
          </li>
          <li className={asPath.startsWith(STATS_BASE_ENDPOINT) ? styles.active : ''}>
            <Link href={`${STATS_BASE_ENDPOINT}study-year`}>STATS</Link>
          </li>
          <li className={asPath.startsWith(MY_STATS_URL) ? styles.active : ''}>
            <Link href={MY_STATS_URL}>MY STATS</Link>
          </li>
        </ul>
      </nav>
    </header>
  );
};

export default Header;
