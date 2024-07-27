package main

import (
	//"html/template"
	"net/http"
	"time"

	"github.com/go-chi/chi/v5"

	"github.com/go-pkgz/auth"
	"github.com/go-pkgz/auth/avatar"
	"github.com/go-pkgz/auth/token"
	"github.com/go-pkgz/rest/logger"

	log "github.com/go-pkgz/lgr"

	"path/filepath"
	"strings"
)

func main() {
	// Set up environment variables
	githubClientID := "Ov23liLV2BeIRdGdqHsD"
	githubClientSecret := "71c8ce540fbbe7130cbccd24275b8cc071f418b8"
	siteURL := "http://localhost:8080"

	// Create auth service
	authService := auth.NewService(auth.Opts{
		SecretReader: token.SecretFunc(func(id string) (string, error) { // secret key for JWT
			return "secret", nil
		}),
		Issuer:         "my-demo-service",
		Logger:         nil,
		AvatarStore:    avatar.NewLocalFS("/tmp"),
		TokenDuration:  time.Hour * 24,
		CookieDuration: time.Hour * 24,
		DisableXSRF:    true,
		URL:            siteURL,
		Validator: token.ValidatorFunc(func(_ string, claims token.Claims) bool {
			return claims.User != nil
		}),
	})

	// Add GitHub provider
	authService.AddProvider("github", githubClientID, githubClientSecret)
	m := authService.Middleware()

	//tmpl := template.Must(template.ParseFiles("templates/index.html"))

	// Set up router
	r := chi.NewRouter()
	r.Use(logger.New(logger.Log(log.Default()), logger.WithBody, logger.Prefix("[INFO]")).Handler)

	paywallStatic, _ := LoadPaywallStatic("static")
	paywall := NewPaywall("_site", &RecursiveFilePathLoader{}, paywallStatic)

	r.Group(func(ro chi.Router) {
		ro.Use(m.Trace)
		ro.Get("/*", func(w http.ResponseWriter, r *http.Request) {
			path := r.URL.Path
			userInfo, err := token.GetUserInfo(r)
			if err != nil {
				log.Printf("failed to get user info, %s", err)
			}
			data := struct {
				Name     string
				LoggedIn bool
				HasPaid     bool
				PaywallTemplate *PaywallTemplate
			}{
				Name:     userInfo.Name,
				LoggedIn: userInfo.Name != "",
				HasPaid:     true,
				PaywallTemplate: nil,
			}

			//if main site, serve index.html from paywall
		if path == "/" {
				tmpl, ok := paywall.tmpl_map["/index.html"]
				if !ok {
					http.Error(w, "404 not found", http.StatusNotFound)
					return
				}
				data.PaywallTemplate = tmpl
				err := tmpl.Template.Execute(w, data)
				if err != nil {
					log.Fatalf("Error executing template: %v", err)
				}
				return
			}

			if strings.HasSuffix(path, ".html") {
				tmpl, ok := paywall.tmpl_map[path]
				if !ok {
					http.Error(w, "404 not found", http.StatusNotFound)
					return
				}
				data.PaywallTemplate = tmpl
				err := tmpl.Template.Execute(w, data)
				if err != nil {
					log.Fatalf("Error executing template: %v", err)
				}
				return
				//else, if no file extension, also serve template
			} else if filepath.Ext(path) == "" {
				tmpl, ok := paywall.tmpl_map[path+".html"]
				if !ok {
					http.Error(w, "404 not found", http.StatusNotFound)
					return
				}
				data.PaywallTemplate = tmpl
				err := tmpl.Template.Execute(w, data)
				if err != nil {
					log.Fatalf("Error executing template: %v", err)
				}
				return
				// if not html file or no file extension, serve from file server
			} else {
				http.FileServer(http.Dir("_site")).ServeHTTP(w, r)
			}
		})
	})

	// Mount auth routes
	authRoutes, avaRoutes := authService.Handlers()
	r.Mount("/auth", authRoutes)
	r.Mount("/avatar", avaRoutes)

	// Start the server
	log.Printf("Starting server on :8080")
	httpServer := &http.Server{
		Addr:              ":8080",
		ReadHeaderTimeout: 5 * time.Second,
		Handler:           r,
	}

	if err := httpServer.ListenAndServe(); err != nil {
		log.Printf("[PANIC] failed to start http server, %v", err)
	}
}
