package database

import "database/sql"

func Seed(db *sql.DB) {
	// err := godotenv.Load()
	// if err != nil {
	// 	log.Fatal("Error loading .env file")
	// }

	// connStr, err := fmt.Printf("postgres://%s:%s@%s/%s", os.Getenv("DB_USER"), os.Getenv("DB_PWD"), os.Getenv("DB_NAME"), os.Getenv("DB_HOST"))
	// db, err := sql.Open("postgres", strconv.Itoa(connStr))
	// if err != nil {
	// 	log.Fatal(err)
	// }

}
