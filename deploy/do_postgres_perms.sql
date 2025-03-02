-- Run these manually after the database is created, using psql from your local machine
-- you will need to disable Trusted sources for the DB to be accessible from your local machine
-- psql -h <host> -U <user> -d <database> -f deploy/do_postgres_perms.sql
-- Allow newsletter to use and create objects in public schema
GRANT USAGE, CREATE ON SCHEMA public TO newsletter;

-- Allow newsletter to modify existing tables in public schema
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO newsletter;

-- Allow newsletter to modify sequences (needed for auto-incrementing IDs)
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO newsletter;

-- Allow newsletter to use existing functions
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public TO newsletter;

-- Ensure future objects created in public schema are accessible
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO newsletter;
