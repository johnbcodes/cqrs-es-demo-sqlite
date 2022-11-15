CREATE TABLE account_query
(
  view_id text                         NOT NULL,
  version integer CHECK (version >= 0) NOT NULL,
  payload text                         NOT NULL,
  PRIMARY KEY (view_id)
) STRICT;
