package main

import (
	"io/ioutil"
	"path/filepath"
	"os"
	"strings"
	log "github.com/go-pkgz/lgr"
)

type RecursiveLoader interface {
	WalkTarget(target string, fileType string, contentFunc func(content string) PaywallTemplate) (map[string]*PaywallTemplate, error)
}

type RecursiveFilePathLoader struct{}
type SingleTestStringLoader struct{}

func (r RecursiveFilePathLoader) WalkTarget(target string, fileType string, contentFunc func(content string) PaywallTemplate) (map[string]*PaywallTemplate, error) {
	target_map := make(map[string]*PaywallTemplate)
	err := filepath.Walk(target, func(path string, info os.FileInfo, err error) error {
		if strings.HasSuffix(path, fileType) {
			log.Printf("loaded path: %s", path)

			//read file from path as string
			content, err := ioutil.ReadFile(path)

			if err != nil {
				log.Fatalf("failed to fetch and load content: %v", err)
			}

			content_processed := contentFunc(string(content))

			//create path_stripped with leading slash and first folder removed
			path_stripped := strings.TrimPrefix(path, target)
			target_map[path_stripped] = &content_processed
			return nil
		}
		if err != nil {
			return err
		}
		return nil
	})
	return target_map, err
}

func (r SingleTestStringLoader) WalkTarget(target string, fileType string, contentFunc func(content string) PaywallTemplate) (map[string]*PaywallTemplate, error) {
	target_map := make(map[string]*PaywallTemplate)
	
	result := contentFunc(target)
	target_map[target] = &result 

	return target_map, nil
}
