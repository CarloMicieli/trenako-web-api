-- migrate:up
CREATE TABLE public.users (
    user_id varchar(25) NOT NULL,
    username varchar(25) NOT NULL,
    email varchar(100) NOT NULL,
    password varchar(25) NOT NULL,
    login_session varchar(36) NOT NULL DEFAULT '',
    gravatar_hash varchar(32) NULL,
    CONSTRAINT "PK_users" PRIMARY KEY (user_id)
);

CREATE UNIQUE INDEX "Idx_users_username"
    ON users USING btree
    (username ASC NULLS LAST);

-- migrate:down
DROP INDEX public."Idx_users_username";
DROP TABLE public.users;