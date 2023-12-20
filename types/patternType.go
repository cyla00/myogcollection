package types

import "time"

type Pattern struct {
	id        string
	owner_id  string
	path      string
	tags      []string
	createdAt time.Time
}
