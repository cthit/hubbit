-- Create hubbit client
INSERT INTO internal_text (id, sv, en)
VALUES
  (
    'dc989ea3-c80b-4fbf-97d3-dbb6869cdd26',
    'hubbit', 'hubbit'
  );
INSERT INTO itclient (
    id, client_id, client_secret, web_server_redirect_uri,
    access_token_validity, refresh_token_validity,
    auto_approve, name, description
  )
VALUES
  (
    '714ee306-e904-4978-bb2b-cd1a3478062c',
    'hubbit', '{noop}hubbit', 'http://localhost:3000/api/auth/gamma/callback',
    3600, 500000000, true, 'hubbit', 'dc989ea3-c80b-4fbf-97d3-dbb6869cdd26'
  );

 -- Create hubbit api key
INSERT INTO apikey (id, name, description, key)
VALUES
  (
    'c0f26d1b-9e70-4218-bb58-62ba2da72ce5',
    'hubbit', 'dc989ea3-c80b-4fbf-97d3-dbb6869cdd26',
    'hubbit'
  );
