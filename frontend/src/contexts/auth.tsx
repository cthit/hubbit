import React, { createContext, useContext } from 'react';

import { gql } from '@urql/core';

import { AuthUserQuery, useAuthUserQuery } from '../__generated__/graphql';
import Error from '../components/error/Error';

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
  const [{ data, error }] = useAuthUserQuery();

  if (!data) {
    return null;
  }

  if (error) {
    console.log('Error retrieving auth context', error);
    return <Error />;
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
