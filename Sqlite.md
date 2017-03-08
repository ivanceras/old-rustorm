A sqlite rough equivalent for table with uuid
and timestamp with time zone now()

```sql
CREATE TABLE users
(
  user_id uuid NOT NULL PRIMARY KEY DEFAULT (substr(lower(hex(randomblob(16))),1,8)||'-'||substr(lower(hex(randomblob(16))),9,4)||'-4'||substr(lower(hex(randomblob(16))),13,3)|| '-'||substr('89ab',abs(random()) % 4 + 1, 1)||substr(lower(hex(randomblob(16))),17,3)||'-'||substr(lower(hex(randomblob(16))),21,12))
,
  username text,
  password text,
  email text,
  created date NOT NULL DEFAULT current_timestamp,
  created_by uuid,
  updated date NOT NULL DEFAULT current_timestamp,
  updated_by uuid,
  active boolean NOT NULL DEFAULT true
)
```
