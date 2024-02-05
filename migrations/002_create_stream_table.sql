CREATE TABLE IF NOT EXISTS streams (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  app VARCHAR(255),
  stream_name VARCHAR(255),
  url VARCHAR(255),
  user_id UUID,
  isLive BOOLEAN DEFAULT FALSE,
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id)
);
