DO $$ postd.elog(NOTICE, 'this', 'is', 'inline', 'code') $$ LANGUAGE postd;
DO $$ postd.return_next(new Object());$$ LANGUAGE postd;
