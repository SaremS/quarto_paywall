package main

import (
	//"html/template"
	"gowall/config"
	"gowall/files"
	"gowall/paywall"
	"net/http"
	"path/filepath"
	"strings"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-pkgz/auth"
	"github.com/go-pkgz/auth/avatar"
	"github.com/go-pkgz/auth/token"
	log "github.com/go-pkgz/lgr"
	"github.com/go-pkgz/rest/logger"
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

	// tmpl := template.Must(template.ParseFiles("templates/index.html"))

	// Set up router
	r := chi.NewRouter()
	r.Use(logger.New(logger.Log(log.Default()), logger.WithBody, logger.Prefix("[INFO]")).Handler)

	fileLoader := files.NewDiskFileLoader()
	recursiveLoader := files.NewRecursiveFilePathLoader(fileLoader)

	htmlFiles, err := recursiveLoader.WalkTarget("_site", ".html")
	if err != nil {
		panic(err)
	}

	paywallContent, _ := fileLoader.ReadFileToString("static/paywall.html")
	registerwallContent, _ := fileLoader.ReadFileToString("static/registerwall.html")
	loginscriptContent, _ := fileLoader.ReadFileToString("static/login_github.html")
	navbarLoginButton, _ := fileLoader.ReadFileToString("static/navbar_login_button.html")

	paywallStaticContent := paywall.PaywallStaticContent{
		Paywall:           paywallContent,
		Registerwall:      registerwallContent,
		LoginScriptGithub: loginscriptContent,
		NavbarLoginButton: navbarLoginButton,
	}

	csv := `name, path, id, price, currency, cutoffClassname
  test, _site/posts/paywalled.html, abcd, 12.34, EUR, PAYWALL`

	conf, err := config.NewPaywallConfigFromCsvString(csv)
	if err != nil {
		panic(err)
	}

	htmlConfigMap, err := paywall.NewHtmlPaywallConfigFromMap(htmlFiles, conf)
	if err != nil {
		panic(err)
	}

	pw, err := paywall.NewPaywallFromStringDocs(htmlConfigMap, paywallStaticContent)

	pw.StripPrefixFromPaths("_site")

	if err != nil {
		panic(err)
	}

	r.Group(func(ro chi.Router) {
		ro.Use(m.Trace)
		ro.Get("/*", func(w http.ResponseWriter, r *http.Request) {
			path := r.URL.Path
			uInfo, err := token.GetUserInfo(r)
			if err != nil {
				log.Printf("failed to get user info, %s", err)
			}
			uInfoHasPaid := paywall.NewUserInfoHasPaid(uInfo.Name, uInfo.Name != "", false)

			if path == "/" {
				pw.WriteHtmlReponse(w, "/index.html", uInfoHasPaid)
				return
			}

			if strings.HasSuffix(path, ".html") {
				pw.WriteHtmlReponse(w, path, uInfoHasPaid)
				return
				// else, if no file extension, also serve template
			} else if filepath.Ext(path) == "" {
				pw.WriteHtmlReponse(w, path+".html", uInfoHasPaid)
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
