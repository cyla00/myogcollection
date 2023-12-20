package types

import "time"

type User struct {
	id        string
	username  string
	email     string
	password  string
	createdAt time.Time
	active    bool
}
