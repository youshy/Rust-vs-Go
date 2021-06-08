package types

import "github.com/gofrs/uuid"

type Tweet struct {
	ID     uuid.UUID
	Body   string
	Author string
	Likes  int
}
