CREATE TABLE "user" (
  "id" BIGSERIAL PRIMARY KEY NOT NULL,
  "name" varchar(64) NOT NULL UNIQUE,
  "email" varchar(64) NOT NULL UNIQUE,
  "password" varchar(1024) NOT NULL,
  "avatar_id" varchar(64),
  "first_name" varchar(64),
  "last_name" varchar(64),
  "email_confirmed_at" timestamp with TIME ZONE,
  "updated_at" timestamp with TIME ZONE NOT NULL DEFAULT NOW(),
  "created_at" timestamp with TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT manage_updated_at('user');

CREATE UNIQUE INDEX idx_email on "user"("email");

CREATE UNIQUE INDEX idx_username on "user"("name");
