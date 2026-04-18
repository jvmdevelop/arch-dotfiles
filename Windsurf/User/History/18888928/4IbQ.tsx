"use client"

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { 
  History,
  GraduationCap,
  Users,
  Trophy,
  BookOpen,
  Video,
  Star,
  Calendar,
  Award,
  Heart,
  MessageCircle
} from "lucide-react"

interface MuseumSectionProps {
  onNavigate?: (section: string) => void
}

export function MuseumSection({ onNavigate }: MuseumSectionProps) {
  const museumCategories = [
    {
      id: "memories",
      title: "Воспоминания лицеистов",
      description: "Истории и воспоминания выпускников",
      icon: Heart,
      count: 156
    },
    {
      id: "history",
      title: "Немного истории",
      description: "История создания и развития лицея",
      icon: History,
      count: 89
    },
    {
      id: "graduates",
      title: "Выпускные классы",
      description: "Фотографии и информация о выпусках",
      icon: Users,
      count: 234
    },
    {
      id: "life",
      title: "Лицейская жизнь",
      description: "Повседневная жизнь и события лицея",
      icon: Star,
      count: 312
    },
    {
      id: "tv",
      title: "Лицейское TV",
      description: "Видео материалы и трансляции",
      icon: Video,
      count: 67
    },
    {
      id: "records",
      title: "Книга рекордов и побед",
      description: "Достижения и рекорды лицея",
      icon: Trophy,
      count: 145
    }
  ]

  const featuredStories = [
    {
      id: 1,
      title: "Первый выпуск лицея",
      excerpt: "История о первом выпуске лицея 1998 года и пути первых лицеистов",
      author: "Мария Иванова",
      date: "15.03.2024",
      category: "Выпускные классы",
      imageUrl: "/api/placeholder/400/250",
      likes: 45,
      comments: 12
    },
    {
      id: 2,
      title: "Победа на Республиканской олимпиаде",
      excerpt: "Как наши ученики завоевали первые места на олимпиаде по математике",
      author: "Александр Петров",
      date: "10.03.2024",
      category: "Наша гордость",
      imageUrl: "/api/placeholder/400/250",
      likes: 67,
      comments: 23
    },
    {
      id: 3,
      title: "День основания лицея",
      excerpt: "Торжественное мероприятие посвященное 25-летию лицея",
      author: "Елена Сидорова",
      date: "05.03.2024",
      category: "Лицейская жизнь",
      imageUrl: "/api/placeholder/400/250",
      likes: 89,
      comments: 34
    }
  ]

  return (
    <div className="w-full max-w-7xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="text-center mb-12">
        <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-primary via-primary/80 to-primary/60 bg-clip-text text-transparent">
          Музей Лицея
        </h1>
        <p className="text-default-600 text-lg max-w-3xl mx-auto">
          Хранилище истории, традиций и достижений нашего лицея. Здесь собраны воспоминания, 
          фотографии и документы, рассказывающие о жизни лицея.
        </p>
      </div>

      {/* Categories Grid */}
      <div className="mb-12">
        <h2 className="text-3xl font-bold mb-6">Разделы музея</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {museumCategories.map((category) => {
            const Icon = category.icon
            return (
              <Card 
                key={category.id}
                isHoverable
                isPressable
                className="border-1 border-default-200 dark:border-default-100"
                onPress={() => onNavigate?.(category.id)}
              >
                <CardHeader className="pb-2">
                  <div className="flex items-center justify-between w-full">
                    <div className="flex items-center gap-3">
                      <div className={`p-2 rounded-lg bg-${category.color}-100 dark:bg-${category.color}-900/20`}>
                        <Icon className={`h-6 w-6 text-${category.color}-600 dark:text-${category.color}-400`} />
                      </div>
                      <div>
                        <h3 className="font-semibold text-lg">{category.title}</h3>
                        <p className="text-default-500 text-sm">{category.description}</p>
                      </div>
                    </div>
                    <Chip size="sm" variant="flat" color={category.color}>
                      {category.count}
                    </Chip>
                  </div>
                </CardHeader>
              </Card>
            )
          })}
        </div>
      </div>

      {/* Featured Stories */}
      <div className="mb-12">
        <h2 className="text-3xl font-bold mb-6">Избранные истории</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {featuredStories.map((story) => (
            <Card key={story.id} className="border-1 border-default-200 dark:border-default-100">
              <CardHeader className="p-0">
                <Image
                  removeWrapper
                  alt={story.title}
                  className="w-full h-48 object-cover rounded-t-lg"
                  src={story.imageUrl}
                />
              </CardHeader>
              <CardBody className="px-6 py-4">
                <div className="flex items-center justify-between mb-2">
                  <Chip size="sm" variant="flat" color="primary">
                    {story.category}
                  </Chip>
                  <div className="flex gap-2 text-default-500 text-sm">
                    <span className="flex items-center gap-1">
                      <Heart className="h-3 w-3" />
                      {story.likes}
                    </span>
                    <span className="flex items-center gap-1">
                      <MessageCircle className="h-3 w-3" />
                      {story.comments}
                    </span>
                  </div>
                </div>
                
                <h3 className="text-xl font-bold mb-2">{story.title}</h3>
                <p className="text-default-600 text-sm mb-3 line-clamp-2">
                  {story.excerpt}
                </p>
                
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Avatar size="sm" name={story.author} className="w-6 h-6 text-tiny" />
                    <span className="text-default-500 text-sm">{story.author}</span>
                  </div>
                  <span className="text-default-400 text-xs">{story.date}</span>
                </div>
              </CardBody>
            </Card>
          ))}
        </div>
      </div>

      {/* Call to Action */}
      <div className="text-center">
        <Card className="bg-gradient-to-r from-primary/10 to-primary/5 border-1 border-primary/20">
          <CardBody className="py-8">
            <div className="flex items-center justify-center gap-3 mb-4">
              <BookOpen className="h-8 w-8 text-primary" />
              <h3 className="text-2xl font-bold">Хотите поделиться историей?</h3>
            </div>
            <p className="text-default-600 mb-6 max-w-2xl mx-auto">
              Если у вас есть воспоминания, фотографии или документы о жизни лицея, 
              поделитесь ими с нами и станьте частью нашей истории.
            </p>
            <Button color="primary" size="lg" className="font-medium">
              Добавить историю в музей
            </Button>
          </CardBody>
        </Card>
      </div>
    </div>
  )
}
