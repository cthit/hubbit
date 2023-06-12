import { createClient as createWSClient } from 'graphql-ws';
import { createClient as createUrqlClient, defaultExchanges, subscriptionExchange } from 'urql';

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
      ...defaultExchanges,
      subscriptionExchange({
        forwardSubscription: operation => {
          return {
            subscribe: sink => ({
              unsubscribe: wsClient.subscribe(operation, sink as any),
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
  });
};

export const createClient = (headers?: Record<string, string>) => {
  if (typeof window === 'undefined') {
    return serverSideClient(headers);
  }

  return clientSideClient();
};
