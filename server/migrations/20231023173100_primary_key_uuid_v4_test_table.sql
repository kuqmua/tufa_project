CREATE TABLE dogs (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  name VARCHAR (255) NOT NULL,
  color VARCHAR (255) NOT NULL
);