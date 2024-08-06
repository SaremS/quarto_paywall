package files

import (
	log "github.com/go-pkgz/lgr"
	"os"
	"path/filepath"
	"strings"
)


type RecursiveLoader interface {
	WalkTarget(targetPath string, fileType string) (map[string]*string, error)
}

type RecursiveFilePathLoader struct {
	fileLoader FileLoader
}

func NewRecursiveFilePathLoader(fileLoader FileLoader) *RecursiveFilePathLoader {
	return &RecursiveFilePathLoader{
		fileLoader: fileLoader,
	}
}

type DummyPathLoader struct {
	fileLoader DummyFileLoader
}

func NewDummyPathLoader(pseudoString string) *DummyPathLoader {
	return &DummyPathLoader{
		fileLoader: *NewDummyFileLoader(pseudoString),
	}
}

func (r *RecursiveFilePathLoader) WalkTarget(targetPath string, fileType string) (map[string]string, error) {
	targetMap := make(map[string]string)
	err := filepath.Walk(targetPath, func(path string, info os.FileInfo, err error) error {
		if strings.HasSuffix(path, fileType) {
			log.Printf("Loaded path: %s", path)

			content, err := r.fileLoader.ReadFileToString(path)
			if err != nil {
				return err
			}

			targetMap[path] = content
			return nil
		}
		if err != nil {
			return err
		}
		return nil
	})
	return targetMap, err
}

func (r *DummyPathLoader) WalkTarget(targetPath string, fileType string) (map[string]*string, error) {
	targetMap := make(map[string]*string)

	content, err := r.fileLoader.ReadFileToString(targetPath)
	if err != nil {
		return targetMap, err
	}

	targetMap[targetPath] = &content

	return targetMap, nil
}
