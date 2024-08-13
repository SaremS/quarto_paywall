package files

type ListLoader interface {
	LoadTargetPaths(targetPaths []string) (map[string]*string, error)
}

type ListFileLoader struct {
	fileLoader FileLoader
}

func NewListFileLoader(fileLoader FileLoader) *ListFileLoader {
	return &ListFileLoader{
		fileLoader: fileLoader,
	}
}

func (l *ListFileLoader) LoadTargetPaths(targetPaths []string) (map[string]string, error) {
	targetMap := make(map[string]string)
	for _, targetPath := range targetPaths {
		content, err := l.fileLoader.ReadFileToString(targetPath)
		if err != nil {
			return nil, err
		}
		targetMap[targetPath] = content
	}
	return targetMap, nil
}
