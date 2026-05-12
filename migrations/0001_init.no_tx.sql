--
-- PostgreSQL database dump
--

--
-- Name: citext; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS citext WITH SCHEMA public;


--
-- Name: EXTENSION citext; Type: COMMENT; Schema: -; Owner:
--

COMMENT ON EXTENSION citext IS 'data type for case-insensitive character strings';


--
-- Name: channel_type; Type: TYPE; Schema: public; Owner: owo
--

CREATE TYPE public.channel_type AS ENUM (
    'dm',
    'group',
    'server_channel'
    );


ALTER TYPE public.channel_type OWNER TO owo;

--
-- Name: friend_status; Type: TYPE; Schema: public; Owner: owo
--

CREATE TYPE public.friend_status AS ENUM (
    'pending',
    'accepted'
    );


ALTER TYPE public.friend_status OWNER TO owo;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: channel; Type: TABLE; Schema: public; Owner: owo
--

CREATE TABLE public.channel (
                                id uuid DEFAULT gen_random_uuid() NOT NULL,
                                name text NOT NULL,
                                type public.channel_type NOT NULL,
                                created_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.channel OWNER TO owo;

--
-- Name: friend; Type: TABLE; Schema: public; Owner: owo
--

CREATE TABLE public.friend (
                               user_one_id uuid NOT NULL,
                               user_two_id uuid NOT NULL,
                               requested_by uuid NOT NULL,
                               status public.friend_status DEFAULT 'pending'::public.friend_status NOT NULL,
                               created_at timestamp with time zone DEFAULT now() NOT NULL,
                               updated_at timestamp with time zone DEFAULT now() NOT NULL,
                               CONSTRAINT friend_check CHECK ((user_one_id < user_two_id)),
                               CONSTRAINT friend_check1 CHECK (((requested_by = user_one_id) OR (requested_by = user_two_id)))
);


ALTER TABLE public.friend OWNER TO owo;

--
-- Name: message; Type: TABLE; Schema: public; Owner: owo
--

CREATE TABLE public.message (
                                id uuid DEFAULT gen_random_uuid() NOT NULL,
                                channel_id uuid,
                                user_id uuid,
                                content text NOT NULL,
                                parent_id uuid,
                                edited_at timestamp with time zone,
                                deleted_at timestamp with time zone,
                                created_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.message OWNER TO owo;

--
-- Name: participant; Type: TABLE; Schema: public; Owner: owo
--

CREATE TABLE public.participant (
                                    channel_id uuid NOT NULL,
                                    user_id uuid NOT NULL,
                                    joined_at timestamp with time zone DEFAULT now(),
                                    last_read_at timestamp with time zone,
                                    is_admin boolean DEFAULT false NOT NULL
);


ALTER TABLE public.participant OWNER TO owo;

--
-- Name: user_block; Type: TABLE; Schema: public; Owner: owo
--

CREATE TABLE public.user_block (
                                   blocker_id uuid NOT NULL,
                                   blocked_id uuid NOT NULL,
                                   created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.user_block OWNER TO owo;

--
-- Name: users; Type: TABLE; Schema: public; Owner: owo
--

CREATE TABLE public.users (
                              id uuid DEFAULT uuidv7() CONSTRAINT user_id_not_null NOT NULL,
                              created_at timestamp(6) with time zone DEFAULT now() CONSTRAINT user_created_at_not_null NOT NULL,
                              username text CONSTRAINT user_username_not_null NOT NULL,
                              email public.citext CONSTRAINT user_email_not_null NOT NULL,
                              password text CONSTRAINT user_password_not_null NOT NULL
);


ALTER TABLE public.users OWNER TO owo;

--
-- Name: channel channel_pkey; Type: CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.channel
    ADD CONSTRAINT channel_pkey PRIMARY KEY (id);


--
-- Name: friend friend_pkey; Type: CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_pkey PRIMARY KEY (user_one_id, user_two_id);


--
-- Name: message message_pkey; Type: CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_pkey PRIMARY KEY (id);


--
-- Name: participant participant_pkey; Type: CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.participant
    ADD CONSTRAINT participant_pkey PRIMARY KEY (channel_id, user_id);


--
-- Name: user_block user_block_pkey; Type: CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.user_block
    ADD CONSTRAINT user_block_pkey PRIMARY KEY (blocker_id, blocked_id);


--
-- Name: users user_pkey; Type: CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- Name: users_email_unique; Type: INDEX; Schema: public; Owner: owo
--

CREATE UNIQUE INDEX users_email_unique ON public.users USING btree (email);


--
-- Name: users_username_unique; Type: INDEX; Schema: public; Owner: owo
--

CREATE UNIQUE INDEX users_username_unique ON public.users USING btree (username);


--
-- Name: friend friend_requested_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_requested_by_fkey FOREIGN KEY (requested_by) REFERENCES public.users(id);


--
-- Name: friend friend_user_one_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_user_one_id_fkey FOREIGN KEY (user_one_id) REFERENCES public.users(id);


--
-- Name: friend friend_user_two_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.friend
    ADD CONSTRAINT friend_user_two_id_fkey FOREIGN KEY (user_two_id) REFERENCES public.users(id);


--
-- Name: message message_channel_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES public.channel(id) ON DELETE CASCADE;


--
-- Name: message message_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.message(id);


--
-- Name: message message_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: participant participant_channel_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.participant
    ADD CONSTRAINT participant_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES public.channel(id) ON DELETE CASCADE;


--
-- Name: participant participant_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.participant
    ADD CONSTRAINT participant_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_block user_block_blocked_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.user_block
    ADD CONSTRAINT user_block_blocked_id_fkey FOREIGN KEY (blocked_id) REFERENCES public.users(id);


--
-- Name: user_block user_block_blocker_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: owo
--

ALTER TABLE ONLY public.user_block
    ADD CONSTRAINT user_block_blocker_id_fkey FOREIGN KEY (blocker_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--