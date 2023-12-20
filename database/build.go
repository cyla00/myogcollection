package database

import (
	"database/sql"
	"errors"
	"log"

	_ "github.com/lib/pq"
)

func Build(db *sql.DB) error {

	usr_table_err := db.QueryRow(`CREATE TABLE [IF NOT EXISTS] users (
		id VARCHAR(50) PRIMARY KEY,
		username VARCHAR(100) NOT NULL,
		email VARCHAR(100) NOT NULL,
		password VARCHAR(500) NOT NULL,
		createdAt DATETIME NULL,
		active BOOLEAN DEFAULT false NOT NULL,
	)`)
	if usr_table_err != nil {
		log.Fatal(usr_table_err)
		return errors.New("db user table creation error")
	}

	pattern_table_err := db.QueryRow(`CREATE TABLE [IF NOT EXISTS] patterns (
		id VARCHAR(50) PRIMARY KEY,
		owner_id VARCHAR(100) NOT NULL,
		_path VARCHAR(100) NOT NULL,
		tags BOOLEAN DEFAULT false NOT NULL,
		createdAt DATETIME NULL,
	)`)
	if pattern_table_err != nil {
		log.Fatal(pattern_table_err)
		return errors.New("db patterns table creation error")
	}
	return nil
}
