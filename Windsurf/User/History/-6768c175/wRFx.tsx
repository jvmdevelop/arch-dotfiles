"use client"

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { 
  BookOpen,
  Search,
  Filter,
  Download,
  Eye,
  Heart,
  Bookmark,
  FileText,
  Video,
  Headphones,
  Image as ImageIcon,
  Calendar,
  User,
  Star,
  TrendingUp,
  Clock
} from "lucide-react"
import { useState } from "react"

interface LibrarySectionProps {
  onNavigate?: (section: string) => void
}

export function LibrarySection({ onNavigate }: LibrarySectionProps) {
  const [searchQuery, setSearchQuery] = useState("")
  const [selectedCategory, setSelectedCategory] = useState("all")
  const [selectedType, setSelectedType] = useState("all")

  const categories = [
    { id: "textbooks", name: "Учебники", count: 234, icon: BookOpen },
    { id: "methodology", name: "Методические материалы", count: 156, icon: FileText },
    { id: "magazines", name: "Журналы и публикации", count: 89, icon: FileText },
    { id: "video", name: "Видео материалы", count: 67, icon: Video },
    { id: "audio", name: "Аудио материалы", count: 34, icon: Headphones },
    { id: "gallery", name: "Фотогалерея", count: 445, icon: ImageIcon }
  ]

  const libraryItems = [
    {
      id: 1,
      title: "Сборник задач по математике для лицеистов",
      author: "Коллектив авторов",
      type: "textbook",
      category: "Учебники",
      description: "Полный сборник задач по всем разделам школьной математики с решениями",
      cover: "/api/placeholder/200/280",
      size: "15.4 MB",
      pages: 456,
      format: "PDF",
      rating: 4.8,
      downloads: 1234,
      views: 3456,
      date: "01.03.2024",
      featured: true
    },
    {
      id: 2,
      title: "Лицейские новости 2024",
      author: "Редакция лицея",
      type: "magazine",
      category: "Журналы и публикации",
      description: "Ежемесячный журнал о жизни лицея, событиях и достижениях",
      cover: "/api/placeholder/200/280",
      size: "8.2 MB",
      pages: 32,
      format: "PDF",
      rating: 4.6,
      downloads: 567,
      views: 1234,
      date: "15.03.2024",
      featured: true
    },
    {
      id: 3,
      title: "Видеоуроки по физике",
      author: "Преподаватели физики",
      type: "video",
      category: "Видео материалы",
      description: "Полный курс видеоуроков по физике для 10-11 классов",
      cover: "/api/placeholder/200/280",
      size: "2.3 GB",
      duration: "12 часов",
      format: "MP4",
      rating: 4.9,
      downloads: 890,
      views: 2345,
      date: "10.03.2024",
      featured: false
    },
    {
      id: 4,
      title: "Методические рекомендации по проектной деятельности",
      author: "Методический кабинет",
      type: "methodology",
      category: "Методические материалы",
      description: "Пособие для учителей по организации проектной деятельности учащихся",
      cover: "/api/placeholder/200/280",
      size: "3.7 MB",
      pages: 78,
      format: "PDF",
      rating: 4.5,
      downloads: 234,
      views: 567,
      date: "05.03.2024",
      featured: false
    },
    {
      id: 5,
      title: "Фотогалерея: 25-летие лицея",
      author: "Фотоархив лицея",
      type: "gallery",
      category: "Фотогалерея",
      description: "Лучшие фотографии с торжественного мероприятия посвященного 25-летию",
      cover: "/api/placeholder/200/280",
      size: "156 MB",
      photos: 234,
      format: "JPG",
      rating: 4.7,
      downloads: 456,
      views: 1234,
      date: "28.02.2024",
      featured: true
    },
    {
      id: 6,
      title: "Аудиокурс английского языка",
      author: "Преподаватели английского",
      type: "audio",
      category: "Аудио материалы",
      description: "Полный аудиокурс для изучения английского языка",
      cover: "/api/placeholder/200/280",
      size: "890 MB",
      duration: "8 часов",
      format: "MP3",
      rating: 4.4,
      downloads: 345,
      views: 678,
      date: "20.02.2024",
      featured: false
    }
  ]

  const getTypeIcon = (type: string) => {
    switch (type) {
      case "textbook": return BookOpen
      case "methodology": return FileText
      case "magazine": return FileText
      case "video": return Video
      case "audio": return Headphones
      case "gallery": return ImageIcon
      default: return FileText
    }
  }

  const getTypeColor = (type: string) => {
    switch (type) {
      case "textbook": return "primary"
      case "methodology": return "secondary"
      case "magazine": return "warning"
      case "video": return "success"
      case "audio": return "danger"
      case "gallery": return "default"
      default: return "default"
    }
  }

  const filteredItems = libraryItems.filter(item => {
    const matchesSearch = item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         item.author.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         item.description.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesCategory = selectedCategory === "all" || item.category === selectedCategory
    const matchesType = selectedType === "all" || item.type === selectedType
    return matchesSearch && matchesCategory && matchesType
  })

  const featuredItems = filteredItems.filter(item => item.featured)
  const regularItems = filteredItems.filter(item => !item.featured)

  return (
    <div className="w-full max-w-7xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="text-center mb-12">
        <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-primary via-primary/80 to-primary/60 bg-clip-text text-transparent">
          Библиотека Лицея
        </h1>
        <p className="text-default-600 text-lg max-w-3xl mx-auto">
          Цифровая библиотека с учебными материалами, методическими пособиями, 
          журналами и мультимедийными ресурсами для учащихся и преподавателей.
        </p>
      </div>

      {/* Categories */}
      <div className="mb-8">
        <h2 className="text-2xl font-bold mb-4">Категории ресурсов</h2>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
          {categories.map((category) => {
            const Icon = category.icon
            return (
              <Card 
                key={category.id}
                isHoverable
                isPressable
                className="text-center border-1 border-default-200 dark:border-default-100"
                onPress={() => setSelectedCategory(category.name)}
              >
                <CardBody className="p-4">
                  <div className="flex justify-center mb-2">
                    <div className="p-2 rounded-lg bg-primary/10">
                      <Icon className="h-6 w-6 text-primary" />
                    </div>
                  </div>
                  <h3 className="font-semibold text-sm mb-1">{category.name}</h3>
                  <Chip size="sm" variant="flat" color="primary">
                    {category.count}
                  </Chip>
                </CardBody>
              </Card>
            )
          })}
        </div>
      </div>

      {/* Search and Filters */}
      <Card className="p-4 mb-8">
        <CardBody className="flex flex-col md:flex-row gap-4">
          <Input
            placeholder="Поиск в библиотеке..."
            startContent={<Search className="h-5 w-5 text-default-400" />}
            value={searchQuery}
            onValueChange={setSearchQuery}
            className="md:flex-1"
            size="lg"
            variant="bordered"
          />
          
          <Select
            placeholder="Категория"
            selectedKeys={selectedCategory === "all" ? [] : [selectedCategory]}
            onSelectionChange={(keys) => setSelectedCategory(Array.from(keys)[0] as string)}
            className="md:w-48"
            size="lg"
            variant="bordered"
          >
            {categories.map(category => (
              <SelectItem key={category.name} value={category.name}>
                {category.name}
              </SelectItem>
            ))}
          </Select>

          <Select
            placeholder="Тип материала"
            selectedKeys={selectedType === "all" ? [] : [selectedType]}
            onSelectionChange={(keys) => setSelectedType(Array.from(keys)[0] as string)}
            className="md:w-48"
            size="lg"
            variant="bordered"
          >
            <SelectItem key="all" value="all">Все типы</SelectItem>
            <SelectItem key="textbook" value="textbook">Учебники</SelectItem>
            <SelectItem key="methodology" value="methodology">Методические материалы</SelectItem>
            <SelectItem key="magazine" value="magazine">Журналы</SelectItem>
            <SelectItem key="video" value="video">Видео</SelectItem>
            <SelectItem key="audio" value="audio">Аудио</SelectItem>
            <SelectItem key="gallery" value="gallery">Галерея</SelectItem>
          </Select>
        </CardBody>
      </Card>

      {/* Featured Items */}
      {featuredItems.length > 0 && (
        <div className="mb-8">
          <div className="flex items-center gap-2 mb-4">
            <Star className="h-5 w-5 text-warning" />
            <h2 className="text-2xl font-bold">Рекомендуемые материалы</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {featuredItems.map((item) => {
              const TypeIcon = getTypeIcon(item.type)
              return (
                <Card key={item.id} className="border-1 border-default-200 dark:border-default-100">
                  <CardHeader className="p-0">
                    <div className="relative">
                      <Image
                        removeWrapper
                        alt={item.title}
                        className="w-full h-48 object-cover rounded-t-lg"
                        src={item.cover}
                      />
                      <div className="absolute top-2 right-2">
                        <Badge color="warning" variant="solid">
                          <Star className="h-3 w-3" />
                        </Badge>
                      </div>
                    </div>
                  </CardHeader>
                  <CardBody className="px-6 py-4">
                    <div className="flex items-center justify-between mb-2">
                      <Chip 
                        size="sm" 
                        variant="flat" 
                        color={getTypeColor(item.type)}
                        startContent={<TypeIcon className="h-3 w-3" />}
                      >
                        {item.category}
                      </Chip>
                      <div className="flex items-center gap-1">
                        <Star className="h-3 w-3 text-warning fill-warning" />
                        <span className="text-sm font-medium">{item.rating}</span>
                      </div>
                    </div>
                    
                    <h3 className="font-semibold text-lg mb-1 line-clamp-1">{item.title}</h3>
                    <p className="text-default-500 text-sm mb-2">{item.author}</p>
                    <p className="text-default-600 text-sm mb-3 line-clamp-2">{item.description}</p>
                    
                    <div className="flex items-center justify-between text-xs text-default-500 mb-3">
                      <span>{item.size}</span>
                      <span>{item.date}</span>
                    </div>
                    
                    <div className="flex gap-2">
                      <Button 
                        color="primary" 
                        size="sm" 
                        className="flex-1"
                        startContent={<Download className="h-3 w-3" />}
                      >
                        Скачать
                      </Button>
                      <Button 
                        variant="flat" 
                        size="sm" 
                        isIconOnly
                      >
                        <Bookmark className="h-3 w-3" />
                      </Button>
                    </div>
                  </CardBody>
                </Card>
              )
            })}
          </div>
        </div>
      )}

      {/* Regular Items */}
      {regularItems.length > 0 && (
        <div className="mb-8">
          <h2 className="text-2xl font-bold mb-4">Все материалы</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {regularItems.map((item) => {
              const TypeIcon = getTypeIcon(item.type)
              return (
                <Card key={item.id} className="border-1 border-default-200 dark:border-default-100">
                  <CardHeader className="p-0">
                    <Image
                      removeWrapper
                      alt={item.title}
                      className="w-full h-48 object-cover rounded-t-lg"
                      src={item.cover}
                    />
                  </CardHeader>
                  <CardBody className="px-6 py-4">
                    <div className="flex items-center justify-between mb-2">
                      <Chip 
                        size="sm" 
                        variant="flat" 
                        color={getTypeColor(item.type)}
                        startContent={<TypeIcon className="h-3 w-3" />}
                      >
                        {item.category}
                      </Chip>
                      <div className="flex items-center gap-1">
                        <Star className="h-3 w-3 text-warning fill-warning" />
                        <span className="text-sm font-medium">{item.rating}</span>
                      </div>
                    </div>
                    
                    <h3 className="font-semibold text-lg mb-1 line-clamp-1">{item.title}</h3>
                    <p className="text-default-500 text-sm mb-2">{item.author}</p>
                    <p className="text-default-600 text-sm mb-3 line-clamp-2">{item.description}</p>
                    
                    <div className="flex items-center justify-between text-xs text-default-500 mb-3">
                      <span>{item.size}</span>
                      <span>{item.date}</span>
                    </div>
                    
                    <div className="flex gap-2">
                      <Button 
                        color="primary" 
                        size="sm" 
                        className="flex-1"
                        startContent={<Download className="h-3 w-3" />}
                      >
                        Скачать
                      </Button>
                      <Button 
                        variant="flat" 
                        size="sm" 
                        isIconOnly
                      >
                        <Bookmark className="h-3 w-3" />
                      </Button>
                    </div>
                  </CardBody>
                </Card>
              )
            })}
          </div>
        </div>
      )}

      {/* Empty State */}
      {filteredItems.length === 0 && (
        <Card className="py-20">
          <CardBody className="flex flex-col items-center gap-4">
            <BookOpen className="h-16 w-16 text-default-300" />
            <h3 className="text-xl font-semibold">Материалы не найдены</h3>
            <p className="text-default-500 text-center max-w-md">
              Попробуйте изменить параметры поиска или выбрать другую категорию
            </p>
          </CardBody>
        </Card>
      )}

      {/* Call to Action */}
      <div className="text-center mt-12">
        <Card className="bg-gradient-to-r from-primary/10 to-secondary/10 border-1 border-primary/20">
          <CardBody className="py-8">
            <div className="flex items-center justify-center gap-3 mb-4">
              <BookOpen className="h-8 w-8 text-primary" />
              <h3 className="text-2xl font-bold">Хотите добавить материал?</h3>
            </div>
            <p className="text-default-600 mb-6 max-w-2xl mx-auto">
              Если у вас есть учебные материалы, методические пособия или другие ресурсы, 
              которыми можно поделиться с лицеистами, добавьте их в нашу библиотеку.
            </p>
            <Button color="primary" size="lg" className="font-medium">
              Добавить материал
            </Button>
          </CardBody>
        </Card>
      </div>
    </div>
  )
}
