package main

import (
	"myog/types"
	"time"
)

func main() {
	// err := godotenv.Load()
	// if err != nil {
	// 	log.Fatal("Error loading .env file")
	// }

	// connStr := fmt.Sprintf("postgres://%s:%s@%s/%s?sslmode=disable", os.Getenv("DB_USER"), os.Getenv("DB_PWD"), os.Getenv("DB_HOST"), os.Getenv("DB_NAME"))
	// db, err := sql.Open("postgres", connStr)
	// if err != nil {
	// 	log.Fatal(err)
	// }

	var testUser types.User
	testUser.Id = "id_123"
	testUser.Username = "username"
	testUser.Email = "email@mail.com"
	testUser.Password = "passwordhashed"
	testUser.CreatedAt = time.Now()
	testUser.Active = false

	var testPattern types.Pattern
	testPattern.Id = "id_pattern"
	testPattern.Owner_id = "id_123"
	testPattern.Title = "title"
	testPattern.Description = "some descriptio"
	testPattern.Gallery_paths = []string{"/some/image/path"}
	testPattern.Pattern_path = "/pattern/path/pdf"
	testPattern.Materials = []string{"material1", "material2"}
	testPattern.Tools = []string{"tool1", "tool2"}
	testPattern.Category = "category"
	testPattern.CreatedAt = time.Now()

	var testComment types.Comments
	testComment.Id = "id_comment"
	testComment.Owner_id = "id_123"
	testComment.Pattern_id = "id_pattern"
	testComment.Comment = "some comment here"
	testComment.Created_at = time.Now()
}
