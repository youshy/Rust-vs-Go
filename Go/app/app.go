package app

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"strconv"

	"github.com/gofrs/uuid"
	"github.com/gorilla/mux"
	"github.com/youshy/Rust-vs-Go/Go/psql"
	"github.com/youshy/Rust-vs-Go/Go/types"
	"github.com/youshy/logger"
	"go.uber.org/zap"
)

type App struct {
	Router   *mux.Router
	Database Database
	log      *zap.SugaredLogger
}

type Database interface {
	GetAllTweets() ([]types.Tweet, error)
	PostTweet(types.Tweet) (uuid.UUID, error)
	GetSingleTweet(uuid.UUID) (types.Tweet, error)
	DeleteTweet(uuid.UUID) error
	GetSingleTweetLikes(uuid.UUID) (int, error)
	PostTweetLike(uuid.UUID) (int, error)
	DeleteTweetLike(uuid.UUID) (int, error)
}

func (a *App) Initialize() error {
	// Set up logger
	logLevel := os.Getenv("LOG_LEVEL")
	if logLevel == "" {
		log.Fatal("LOG_LEVEL not set")
	}
	logJsonOk := os.Getenv("LOG_JSON")
	if logJsonOk == "" {
		log.Fatal("LOG_JSON not set")
	}
	logJson, err := strconv.ParseBool(logJsonOk)
	if err != nil {
		log.Fatalf("Unable to parse bool: %v", err)
	}

	logger := logger.NewLogger(logLevel, logJson)
	a.log = logger
	a.log.Debug("logger set up")

	// Setup database
	host := os.Getenv("PG_HOST")
	if host == "" {
		log.Fatal("PG_HOST not set")
	}
	port := os.Getenv("PG_PORT")
	if port == "" {
		log.Fatal("PG_PORT not set")
	}
	name := os.Getenv("PG_NAME")
	if name == "" {
		log.Fatal("PG_NAME not set")
	}
	username := os.Getenv("PG_USERNAME")
	if username == "" {
		log.Fatal("PG_USERNAME not set")
	}
	password := os.Getenv("PG_PASSWORD")
	if password == "" {
		log.Fatal("PG_PASSWORD not set")
	}
	connect := psql.BuildDBUri(host, port, name, username, password)
	psqlDb, err := psql.SetPostgres(connect)
	if err != nil {
		a.log.Error(err)
		return err
	}
	a.Database = &psqlDb
	a.log.Debug("postgres database connected")

	// Set up endpoints
	prefix := "/api"
	router := mux.NewRouter()

	// TODO: Setup handlers
	router.Handle(prefix+"/health", health()).Methods(http.MethodGet)

	fmt.Printf("Available routes:\n")
	router.Walk(func(route *mux.Route, router *mux.Router, ancestors []*mux.Route) error {
		t, err := route.GetPathTemplate()
		if err != nil {
			return err
		}
		m, err := route.GetMethods()
		if err != nil {
			return err
		}
		fmt.Printf("%s\t%s\n", m, t)
		return nil
	})
	a.Router = router

	return nil
}

func (a *App) Run() {
	// TODO: Add cors
	host := os.Getenv("HOST")
	if host == "" {
		log.Fatal("HOST is not provided")
	}
	port := os.Getenv("PORT")
	if port == "" {
		log.Fatal("PORT is not provided")
	}
	addr := fmt.Sprintf("%s:%s", host, port)

	err := http.ListenAndServe(addr, a.Router)
	if err != nil {
		panic(err)
	}
}

func JSONResponse(w http.ResponseWriter, code int, output interface{}) {
	response, _ := json.Marshal(output)
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(code)
	w.Write(response)
}

// Return notion if API is alive
func health() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		payload := map[string]string{
			"response": "alive",
		}
		response, _ := json.Marshal(payload)

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write(response)
	})
}
