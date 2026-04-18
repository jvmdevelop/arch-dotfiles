"use client"

import { 
  Button,
  Input,
  Select,
  SelectItem,
  Pagination,
  Spinner,
  Chip,
  Tabs,
  Tab,
  Card,
  CardBody,
  Badge
} from "@nextui-org/react"
import { 
  Search,
  Filter,
  Plus,
  Newspaper,
  Calendar,
  TrendingUp,
  Clock,
  Star
} from "lucide-react"
import { useState, useMemo } from "react"
import { HeroCard } from "./hero-card"

interface NewsItem {
  id: number
  title: string
  content: string
  imageUrl?: string
  authorName?: string
  authorAvatar?: string
  createdAt: string
  views?: number
  likes?: number
  comments?: number
  category?: string
  featured?: boolean
}

interface HeroSectionProps {
  news: NewsItem[]
  loading?: boolean
  onSearch?: (query: string) => void
  onFilter?: (filter: string) => void
  onCreateNew?: () => void
  onReadMore?: (id: number) => void
  onLike?: (id: number) => void
  onComment?: (id: number) => void
  onShare?: (id: number) => void
}

export function HeroSection({ 
  news, 
  loading, 
  onSearch, 
  onFilter, 
  onCreateNew, 
  onReadMore,
  onLike,
  onComment,
  onShare
}: HeroSectionProps) {
  const [searchQuery, setSearchQuery] = useState("")
  const [currentPage, setCurrentPage] = useState(1)
  const [selectedCategory, setSelectedCategory] = useState("all")
  const [sortBy, setSortBy] = useState("newest")
  const itemsPerPage = 9

  const categories = useMemo(() => {
    const cats = ["all", ...new Set(news.map(item => item.category).filter(Boolean))]
    return cats
  }, [news])

  const filteredAndSortedNews = useMemo(() => {
    let filtered = news.filter(item =>
      item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      item.content.toLowerCase().includes(searchQuery.toLowerCase())
    )

    if (selectedCategory !== "all") {
      filtered = filtered.filter(item => item.category === selectedCategory)
    }

    // Сортировка
    filtered.sort((a, b) => {
      switch (sortBy) {
        case "newest":
          return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
        case "oldest":
          return new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime()
        case "popular":
          return (b.views || 0) - (a.views || 0)
        case "liked":
          return (b.likes || 0) - (a.likes || 0)
        default:
          return 0
      }
    })

    return filtered
  }, [news, searchQuery, selectedCategory, sortBy])

  const totalPages = Math.ceil(filteredAndSortedNews.length / itemsPerPage)
  const paginatedNews = filteredAndSortedNews.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  )

  const featuredNews = filteredAndSortedNews.filter(item => item.featured).slice(0, 3)
  const regularNews = paginatedNews.filter(item => !item.featured)

  const handleSearch = (value: string) => {
    setSearchQuery(value)
    onSearch?.(value)
    setCurrentPage(1)
  }

  const handleCategoryChange = (category: string) => {
    setSelectedCategory(category)
    setCurrentPage(1)
  }

  return (
    <div className="w-full max-w-7xl mx-auto px-4 py-8">
      {/* Header Section */}
      <div className="mb-8">
        <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4 mb-6">
          <div>
            <h1 className="text-5xl font-bold mb-2 bg-gradient-to-r from-primary via-primary/80 to-primary/60 bg-clip-text text-transparent">
              Новости Лицея
            </h1>
            <p className="text-default-600 text-lg">
              Актуальные события и объявления нашей образовательной платформы
            </p>
            <div className="flex gap-2 mt-3">
              <Chip size="sm" variant="flat" color="primary" startContent={<Newspaper className="h-3 w-3" />}>
                {news.length} новостей
              </Chip>
              <Chip size="sm" variant="flat" color="success" startContent={<TrendingUp className="h-3 w-3" />}>
                {featuredNews.length} избранных
              </Chip>
            </div>
          </div>
          
          {onCreateNew && (
            <Button
              color="primary"
              size="lg"
              startContent={<Plus className="h-5 w-5" />}
              onClick={onCreateNew}
              className="shrink-0 font-medium"
            >
              Создать новость
            </Button>
          )}
        </div>

        {/* Search and Filters */}
        <Card className="p-4">
          <CardBody className="flex flex-col md:flex-row gap-4">
            <Input
              placeholder="Поиск новостей..."
              startContent={<Search className="h-5 w-5 text-default-400" />}
              value={searchQuery}
              onValueChange={handleSearch}
              className="md:flex-1"
              size="lg"
              variant="bordered"
            />
            
            <Select
              placeholder="Категория"
              selectedKeys={selectedCategory === "all" ? [] : [selectedCategory]}
              onSelectionChange={(keys) => handleCategoryChange(Array.from(keys)[0] as string)}
              className="md:w-48"
              size="lg"
              variant="bordered"
            >
              {categories.map(category => (
                <SelectItem key={category} value={category}>
                  {category === "all" ? "Все категории" : category}
                </SelectItem>
              ))}
            </Select>

            <Select
              placeholder="Сортировка"
              selectedKeys={[sortBy]}
              onSelectionChange={(keys) => {
                const newSort = Array.from(keys)[0] as string
                setSortBy(newSort)
                onFilter?.(newSort)
              }}
              className="md:w-48"
              size="lg"
              variant="bordered"
              startContent={<Filter className="h-4 w-4" />}
            >
              <SelectItem key="newest" value="newest" startContent={<Clock className="h-4 w-4" />}>
                Сначала новые
              </SelectItem>
              <SelectItem key="oldest" value="oldest" startContent={<Clock className="h-4 w-4 rotate-180" />}>
                Сначала старые
              </SelectItem>
              <SelectItem key="popular" value="popular" startContent={<TrendingUp className="h-4 w-4" />}>
                Популярные
              </SelectItem>
              <SelectItem key="liked" value="liked" startContent={<Star className="h-4 w-4" />}>
                По лайкам
              </SelectItem>
            </Select>
          </CardBody>
        </Card>
      </div>

      {/* Featured News */}
      {featuredNews.length > 0 && (
        <div className="mb-8">
          <div className="flex items-center gap-2 mb-4">
            <Star className="h-5 w-5 text-warning" />
            <h2 className="text-2xl font-bold">Избранные новости</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {featuredNews.map((item) => (
              <HeroCard
                key={item.id}
                {...item}
                onReadMore={onReadMore}
                onLike={onLike}
                onComment={onComment}
                onShare={onShare}
              />
            ))}
          </div>
        </div>
      )}

      {/* Content */}
      {loading ? (
        <div className="flex justify-center items-center py-20">
          <Spinner size="lg" color="primary" label="Загрузка новостей..." />
        </div>
      ) : filteredAndSortedNews.length === 0 ? (
        <Card className="py-20">
          <CardBody className="flex flex-col items-center gap-4">
            <Newspaper className="h-16 w-16 text-default-300" />
            <h3 className="text-xl font-semibold">Новостей не найдено</h3>
            <p className="text-default-500 text-center max-w-md">
              Попробуйте изменить параметры поиска или выбрать другую категорию
            </p>
          </CardBody>
        </Card>
      ) : (
        <>
          {featuredNews.length > 0 && regularNews.length > 0 && (
            <div className="flex items-center gap-2 mb-4">
              <Newspaper className="h-5 w-5 text-primary" />
              <h2 className="text-2xl font-bold">Все новости</h2>
              <Badge color="primary" variant="flat">{regularNews.length}</Badge>
            </div>
          )}
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
            {regularNews.map((item) => (
              <HeroCard
                key={item.id}
                {...item}
                onReadMore={onReadMore}
                onLike={onLike}
                onComment={onComment}
                onShare={onShare}
              />
            ))}
          </div>

          {/* Pagination */}
          {totalPages > 1 && (
            <div className="flex justify-center">
              <Pagination
                total={totalPages}
                page={currentPage}
                onChange={setCurrentPage}
                color="primary"
                size="lg"
                showControls
                showShadow
                variant="bordered"
              />
            </div>
          )}
        </>
      )}
    </div>
  )
}
