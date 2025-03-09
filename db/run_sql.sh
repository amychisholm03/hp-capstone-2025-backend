#!/bin/sh

DB_PATH="./database.db3"
SQL_DIR="./sql"

if [ ! -f "$DB_PATH" ]; then
  echo "Warning: Database file not found at $DB_PATH"
  echo "This may be because it is the first time running the server." 
fi

# Check if the SQL directory exists
if [ ! -d "$SQL_DIR" ]; then
  echo "SQL directory not found at $SQL_DIR"
  exit 1
fi

# Loop through each .sql file in the current_sql directory
for sql_file in "$SQL_DIR"/*.sql; do
  if [ -f "$sql_file" ]; then
    sqlite3 "$DB_PATH" < "$sql_file"
    if [ $? -ne 0 ]; then
      echo "Error running SQL file: $sql_file"
      exit 1
    fi
  fi
done

echo "SQL ran successfully."
