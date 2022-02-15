import React from 'react';

import Link from 'next/link';
import { useRouter } from 'next/router';

import styles from './Header.module.scss';

const MAIN_ENDPOINT = '/';
const ME_ENDPOINT = '/me';
const STATS_BASE_ENDPOINT = '/stats/';
const MY_STATS_BASE_ENDPOINT = '/users/';
const USER_STATS_ME_ENDPOINT = '/users/me';

const Header = () => {
  const { pathname } = useRouter();

  return (
    <header className={styles.hContainer}>
      <h1>
        <a className={styles.title} href="/">
          Who is in the Hubb?
        </a>
        <div className={styles.debugContainer}>
          <div className={styles.debugCard}>
            <p>
              This is a development version of a rewrite of HubbIT. <br />
              If you find any issues or have any suggestions please create issues on our{' '}
              <Link href="https://github.com/cthit/hubbit2">
                <a>github repository</a>
              </Link>
            </p>
          </div>
        </div>
      </h1>
      <nav>
        <ul className={styles.menu}>
          <li className={pathname === ME_ENDPOINT ? styles.active : ''}>
            <Link href={ME_ENDPOINT}>
              <a>ME</a>
            </Link>
          </li>
          <li className={pathname === MAIN_ENDPOINT ? styles.active : ''}>
            <Link href={MAIN_ENDPOINT}>
              <a>SMURFS IN THE HUBB</a>
            </Link>
          </li>
          <li className={pathname.startsWith(STATS_BASE_ENDPOINT) ? styles.active : ''}>
            <Link href={`${STATS_BASE_ENDPOINT}study-year`}>
              <a>STATS</a>
            </Link>
          </li>
          <li className={pathname.startsWith(MY_STATS_BASE_ENDPOINT) ? styles.active : ''}>
            <Link href={USER_STATS_ME_ENDPOINT}>
              <a>USER STATS</a>
            </Link>
          </li>
        </ul>
      </nav>
    </header>
  );
};

export default Header;
