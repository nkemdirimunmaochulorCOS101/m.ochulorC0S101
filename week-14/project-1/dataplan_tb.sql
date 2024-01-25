--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    dataid integer NOT NULL,
    datasize text NOT NULL,
    dataduration text NOT NULL,
    dataprice text NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (dataid, datasize, dataduration, dataprice) FROM stdin;
1	350MB	2DAYS	200NAIRA
2	1.8GB	14DAYS	500NAIRA
4	7.5GB	30DAYS	1500NAIRA
5	9.2GB	30DAYS	2000NAIRA
6	10.8GB	30DAYS	2500NAIRA
7	14GB	30DAYS	3000NAIRA
8	18GB	30DAYS	4000NAIRA
9	24GB	30DAYS	5000NAIRA
10	29.9GB	30DAYS	8000NAIRA
11	50GB	30DAYS	10000NAIRA
3	3.9GB	30DAYS	1000NAIRA
\.


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (dataid);


--
-- PostgreSQL database dump complete
--

