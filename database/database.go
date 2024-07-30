package database

type Database interface {
	Connect() error
	Disconnect() error
	InsertArticle(user string, article string) error
	FindUser(user string) ([]string, error)
}


type MemoryDatabase struct {
	UserArticles map[string][]string
}

func NewMemoryDatabase() *MemoryDatabase {
	return &MemoryDatabase{
		UserArticles: map[string][]string{},
	}
}	

func (f *MemoryDatabase) Connect() error {
	return nil
}

func (f *MemoryDatabase) Disconnect() error {
	return nil
}

func (f *MemoryDatabase) InsertArticle(user string, article string) error {
	if _, ok := f.UserArticles[user]; !ok {
		f.UserArticles[user] = []string{}
	} 
	f.UserArticles[user] = append(f.UserArticles[user], article)
	return nil	
}

func (f *MemoryDatabase) FindUser(user string) ([]string, error) {
	if _, ok := f.UserArticles[user]; !ok {
		return []string{}, nil
	}else{
		return f.UserArticles[user], nil
	}
}
