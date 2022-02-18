import React, { createContext, useContext } from 'react';

import { gql } from '@urql/core';

import { AuthUserQuery, useAuthUserQuery } from '../__generated__/graphql';

gql`
  query AuthUser {
    me {
      cid
    }
  }
`;

interface State {
  user: AuthUserQuery['me'];
}

const AuthContext = createContext<State>({
  user: {} as any,
});

export const AuthProvider = ({ children }: React.PropsWithChildren<any>) => {
  const [{ data }] = useAuthUserQuery();
  if (!data) {
    return null;
  }

  return (
    <AuthContext.Provider
      value={{
        user: data.me,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  return useContext(AuthContext);
};
