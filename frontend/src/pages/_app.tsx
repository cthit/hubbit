import '../global-styles/styles.scss';
import '../global-styles/tables.scss';
import '../global-styles/groups.scss';
import '../global-styles/roboto-font.css';
import '../global-styles/silkscreen.css';

import React from 'react';

import { AppProps } from 'next/app';
import { Provider } from 'urql';

import { createClient } from '../client';
import Footer from '../components/footer/Footer';
import Header from '../components/header/Header';
import { AuthProvider } from '../contexts/auth';

function HubbitApp({ Component, pageProps }: AppProps) {
  return (
    <Provider value={createClient()}>
      <AuthProvider>
        <div className="pageWrapper">
          <Header />
          <div className="componentWrapper">
            <div className="wrapper">
              <Component {...pageProps} />
            </div>
          </div>
          <Footer />
        </div>
      </AuthProvider>
    </Provider>
  );
}

export default HubbitApp;
