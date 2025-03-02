-- Purpose: Grant permissions to the newsletter user on the newsletter schema.
-- This script should be run with the doadmin user
CREATE schema IF NOT EXISTS newsletter;

-- Allow newsletter to use and create objects in newsletter schema
GRANT USAGE, CREATE ON SCHEMA newsletter TO newsletter;

-- Allow newsletter to modify existing tables in newsletter schema
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA newsletter TO newsletter;

-- Allow newsletter to modify sequences (needed for auto-incrementing IDs)
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA newsletter TO newsletter;

-- Allow newsletter to use existing functions
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA newsletter TO newsletter;

-- Ensure future objects created in newsletter schema are accessible
ALTER DEFAULT PRIVILEGES IN SCHEMA newsletter GRANT ALL ON TABLES TO newsletter;