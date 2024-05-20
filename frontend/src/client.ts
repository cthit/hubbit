import { createClient as createWSClient } from 'graphql-ws';
import { cacheExchange, createClient as createUrqlClient, fetchExchange, subscriptionExchange } from 'urql';

export const clientSideClient = () => {
  // Default to secure websockets, and only downgrade if page is unsecure too
  // e.g. during development
  let websocketProtocol = 'wss';
  if (window.location.protocol === 'http:') {
    websocketProtocol = 'ws';
  }

  const wsClient = createWSClient({
    url: `${websocketProtocol}://${window.location.host}/api/graphql`,
  });

  return createUrqlClient({
    url: '/api/graphql',
    exchanges: [
      cacheExchange,
      fetchExchange,
      subscriptionExchange({
        forwardSubscription: operation => {
          const input = { ...operation, query: operation.query || '' };
          return {
            subscribe: sink => ({
              unsubscribe: wsClient.subscribe(input, sink as any),
            }),
          };
        },
      }),
    ],
  });
};

export const serverSideClient = (headers?: Record<string, string>) => {
  return createUrqlClient({
    url: `${process.env.BACKEND_ADDRESS}/api/graphql`,
    fetchOptions: {
      headers,
    },
    exchanges: [cacheExchange, fetchExchange],
  });
};

export const createClient = (headers?: Record<string, string>) => {
  if (typeof window === 'undefined') {
    return serverSideClient(headers);
  }

  return clientSideClient();
};
