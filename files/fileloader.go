package files

import (
	"io/ioutil"
	log "github.com/go-pkgz/lgr"
)

type FileLoader interface {
	ReadFileToString(path string) (string, error)
}

type DiskFileLoader struct{}

func NewDiskFileLoader() *DiskFileLoader {
	return &DiskFileLoader{}
}

func (d *DiskFileLoader) ReadFileToString(path string) (string, error) {
	data, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatalf("Failed to fetch and load content in file %s: %v", path, err)
		return "", err
	}
	return string(data), nil
}

type DummyFileLoader struct{
	pseudoString string
}

func NewDummyFileLoader(pseudoString string) *DummyFileLoader {
	return &DummyFileLoader{
		pseudoString: pseudoString,
	}
}

func (d *DummyFileLoader) ReadFileToString(path string) (string, error) {
	return d.pseudoString, nil
}
