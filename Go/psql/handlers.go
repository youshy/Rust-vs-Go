package psql

import (
	"github.com/gofrs/uuid"
	"github.com/youshy/Rust-vs-Go/Go/types"
)

func (p *Postgres) GetAllTweets() ([]types.Tweet, error) {
	// TODO: Implement!
	return nil, nil
}

func (p *Postgres) PostTweet(tweet types.Tweet) (uuid.UUID, error) {
	return uuid.Nil, nil
}

func (p *Postgres) GetSingleTweet(id uuid.UUID) (types.Tweet, error) {
	return types.Tweet{}, nil
}

func (p *Postgres) DeleteTweet(id uuid.UUID) error {
	return nil
}

func (p *Postgres) GetSingleTweetLikes(id uuid.UUID) (int, error) {
	return 0, nil
}

func (p *Postgres) PostTweetLike(id uuid.UUID) (int, error) {
	return 0, nil
}

func (p *Postgres) DeleteTweetLike(id uuid.UUID) (int, error) {
	return 0, nil
}
