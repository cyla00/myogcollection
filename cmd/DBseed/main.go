package main

import (
	"database/sql"
	"fmt"
	"log"
	"myog/types"
	"os"
	"time"

	"github.com/joho/godotenv"
	"github.com/lib/pq"
	_ "github.com/lib/pq"
)

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	var testUser types.User
	testUser.Id = "022dc094-bad7-4268-b2c9-bcc16439545a"
	testUser.Username = "username"
	testUser.Email = "ikhayam000@protonmail.com"
	testUser.Password = "JGFyZ29uMmlkJHY9MTkkbT0xNix0PTIscD0xJGNHRnpjM2R2Y21RJDh2RFMzcnNlek9qcnVyMDFkRjEyRUE=" // password, salt password
	testUser.CreatedAt = time.Now()
	testUser.Active = false

	var testPattern types.Pattern
	testPattern.Id = "cebaddd7-401c-49a6-accb-b97574352a13"
	testPattern.Owner_id = "022dc094-bad7-4268-b2c9-bcc16439545a"
	testPattern.Title = "title"
	testPattern.Pattern_description = "some description"
	testPattern.Gallery_paths = []string{"/images/img.jpg"}
	testPattern.Pattern_path = "/patterns/doc.pdf"
	testPattern.Materials = []string{"material1", "material2"}
	testPattern.Tools = []string{"tool1", "tool2"}
	testPattern.Category = "category"
	testPattern.CreatedAt = time.Now()

	var testComment types.Comments
	testComment.Id = "436cec1a-3764-41db-875c-f62746ef3942"
	testComment.Owner_id = "022dc094-bad7-4268-b2c9-bcc16439545a"
	testComment.Pattern_id = "cebaddd7-401c-49a6-accb-b97574352a13"
	testComment.Comment = "some comment here about the pattern"
	testComment.Created_at = time.Now()

	connStr := fmt.Sprintf("postgres://%s:%s@%s/%s?sslmode=disable", os.Getenv("DB_USER"), os.Getenv("DB_PWD"), os.Getenv("DB_HOST"), os.Getenv("DB_NAME"))
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		log.Fatal(err)
	}

	user_creation_err := db.QueryRow(`
		INSERT INTO users (
			id,
			username,
			email,
			user_password,
			created_at,
			active
		) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING
	`,
		testUser.Id,
		testUser.Email,
		testUser.Username,
		testUser.Password,
		testUser.CreatedAt,
		testUser.Active)
	if user_creation_err != nil {
		fmt.Println(user_creation_err.Err())
	}

	pattern_creation_err := db.QueryRow(`
		INSERT INTO patterns (
			id,
			owner_id,
			title,
			pattern_description,
			gallery_paths,
			pattern_path,
			materials,
			tools,
			category,
			created_at
		) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT DO NOTHING
	`,
		testPattern.Id,
		testPattern.Owner_id,
		testPattern.Title,
		testPattern.Pattern_description,
		pq.Array(testPattern.Gallery_paths),
		testPattern.Pattern_path,
		pq.Array(testPattern.Materials),
		pq.Array(testPattern.Tools),
		testPattern.Category,
		testPattern.CreatedAt)
	if user_creation_err != nil {
		fmt.Println(pattern_creation_err.Err())
	}

	comment_creation_err := db.QueryRow(`
		INSERT INTO comments (
			id,
			owner_id,
			pattern_id,
			comment,
			created_at
		) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING
	`,
		testComment.Id,
		testComment.Owner_id,
		testComment.Pattern_id,
		testComment.Comment,
		testComment.Created_at)
	if user_creation_err != nil {
		fmt.Println(comment_creation_err.Err())
	}

	println("database seeded successfully")
}
