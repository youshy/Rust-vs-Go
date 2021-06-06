package psql

import (
	"fmt"

	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/postgres"
	"github.com/youshy/Rust-vs-Go/Go/types"
)

type Postgres struct {
	db *gorm.DB
}

func RecordNotFound(entity string) error {
	return fmt.Errorf("%v: Record Not Found", entity)
}

// BuildDBUri prebuilds the string for establishing database connection
func BuildDBUri(host, port, dbname, username, password string) string {
	return fmt.Sprintf("host=%s port=%s user=%s dbname=%s password=%s sslmode=disable", host, port, username, dbname, password)
}

// SetPostgres takes an interface to allow it to be tested.
// Although this function seems to be idiotic for now,
// it might be extended in the future for better setup.
func SetPostgres(dbUri interface{}) (Postgres, error) {
	p := Postgres{}
	conn, err := gorm.Open("postgres", dbUri)
	if err != nil {
		return p, err
	}

	p.db = conn
	p.setMigrations()

	return p, nil
}

func (p *Postgres) setMigrations() {
	p.db.LogMode(true)
	// TODO: Migrations!
	p.db.Debug().AutoMigrate(
		&types.Tweet{},
	)
}
