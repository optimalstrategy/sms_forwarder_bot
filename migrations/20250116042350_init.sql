-- Exported from the original auto-generated Django schema
CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE IF NOT EXISTS public.bot_tguser (
    username CITEXT NOT NULL,
    telegram_id BIGINT NOT NULL,
    codes TEXT NOT NULL
);

ALTER TABLE "public".bot_tguser ALTER COLUMN "username" SET DATA TYPE CITEXT;
ALTER TABLE "public".bot_tguser ALTER COLUMN "codes" SET DATA TYPE TEXT;
ALTER TABLE "public".bot_tguser ALTER COLUMN "telegram_id" SET DATA TYPE BIGINT;

ALTER TABLE public.bot_tguser OWNER TO postgres;

ALTER TABLE ONLY public.bot_tguser
    DROP CONSTRAINT IF EXISTS bot_tguser_pkey;

ALTER TABLE ONLY public.bot_tguser
    DROP CONSTRAINT IF EXISTS bot_tguser_username_key;

ALTER TABLE ONLY public.bot_tguser
    ADD CONSTRAINT bot_tguser_pkey PRIMARY KEY (telegram_id);

ALTER TABLE ONLY public.bot_tguser
    ADD CONSTRAINT bot_tguser_username_key UNIQUE (username);
