package types

import "github.com/gofrs/uuid"

type Tweet struct {
	ID     uuid.UUID
	Author string
	Likes  int
}
