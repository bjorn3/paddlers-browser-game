CREATE TYPE UNIT_TYPE AS ENUM ('basic', 'hero');
CREATE TYPE UNIT_COLOR AS ENUM ('yellow', 'white', 'camo');

CREATE TABLE units (
  id BIGSERIAL PRIMARY KEY,
  home BIGINT NOT NULL,
  unit_type UNIT_TYPE NOT NULL,
  color UNIT_COLOR,
  hp BIGINT NOT NULL,
  speed REAL NOT NULL
);