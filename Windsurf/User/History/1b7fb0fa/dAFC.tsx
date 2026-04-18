"use client"

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { 
  Trophy,
  Award,
  Brain,
  Heart,
  Users,
  Star,
  TrendingUp,
  Calendar,
  MapPin
} from "lucide-react"

interface AchievementsSectionProps {
  onNavigate?: (section: string) => void
}

export function AchievementsSection({ onNavigate }: AchievementsSectionProps) {
  const achievementCategories = [
    {
      id: "academic",
      title: "Академические достижения",
      description: "Олимпиады, конкурсы, научные проекты",
      icon: Brain,
      color: "primary",
      stats: { total: 156, thisYear: 23, growth: 15 }
    },
    {
      id: "sports",
      title: "Спортивные победы",
      description: "Соревнования, чемпионаты, рекорды",
      icon: Trophy,
      color: "success",
      stats: { total: 89, thisYear: 12, growth: 8 }
    },
    {
      id: "creative",
      title: "Творческие конкурсы",
      description: "Искусство, музыка, литература",
      icon: Palette,
      color: "warning",
      stats: { total: 67, thisYear: 15, growth: 25 }
    },
    {
      id: "social",
      title: "Социальные проекты",
      description: "Волонтерство, общественная деятельность",
      icon: Heart,
      color: "danger",
      stats: { total: 45, thisYear: 8, growth: 12 }
    }
  ]

  const recentAchievements = [
    {
      id: 1,
      title: "Гран-при Республиканской олимпиады по математике",
      category: "Академические",
      date: "15.03.2024",
      participants: ["Иван Петров", "Мария Сидорова", "Александр Козлов"],
      level: "Республиканский",
      place: "г. Астана",
      image: "/api/placeholder/400/250",
      award: "Золотая медаль"
    },
    {
      id: 2,
      title: "Чемпионы города по баскетболу",
      category: "Спортивные",
      date: "10.03.2024",
      participants: ["Команда 'Лицеисты'"],
      level: "Городской",
      place: "г. Алматы",
      image: "/api/placeholder/400/250",
      award: "Кубок чемпиона"
    },
    {
      id: 3,
      title: "Первое место на конкурсе научных проектов",
      category: "Академические",
      date: "05.03.2024",
      participants: ["Елена Иванова", "Дмитрий Смирнов"],
      level: "Международный",
      place: "г. Москва",
      image: "/api/placeholder/400/250",
      award: "Грант на развитие"
    },
    {
      id: 4,
      title: "Лучший театральный коллектив",
      category: "Творческие",
      date: "28.02.2024",
      participants: ["Театр 'Маска'"],
      level: "Республиканский",
      place: "г. Шымкент",
      image: "/api/placeholder/400/250",
      award: "Гран-при фестиваля"
    }
  ]

  const topStudents = [
    {
      id: 1,
      name: "Анна Меркулова",
      achievements: 23,
      points: 450,
      avatar: "/api/placeholder/100/100",
      specialty: "Математика, Физика"
    },
    {
      id: 2,
      name: "Михаил Волков",
      achievements: 19,
      points: 380,
      avatar: "/api/placeholder/100/100",
      specialty: "Химия, Биология"
    },
    {
      id: 3,
      name: "София Козлова",
      achievements: 17,
      points: 350,
      avatar: "/api/placeholder/100/100",
      specialty: "Литература, Искусство"
    },
    {
      id: 4,
      name: "Даниил Петров",
      achievements: 15,
      points: 320,
      avatar: "/api/placeholder/100/100",
      specialty: "Информатика, Робототехника"
    },
    {
      id: 5,
      name: "Елизавета Новикова",
      achievements: 14,
      points: 300,
      avatar: "/api/placeholder/100/100",
      specialty: "История, Обществознание"
    }
  ]

  return (
    <div className="w-full max-w-7xl mx-auto px-4 py-8">
      {/* Header */}
      <div className="text-center mb-12">
        <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-primary via-primary/80 to-primary/60 bg-clip-text text-transparent">
          Достижения Лицея
        </h1>
        <p className="text-default-600 text-lg max-w-3xl mx-auto">
          Наши ученики и педагоги постоянно достигают новых высот в учебе, спорте, 
          творчестве и научной деятельности. Здесь мы гордимся каждым успехом!
        </p>
      </div>

      {/* Statistics Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-12">
        {achievementCategories.map((category) => {
          const Icon = category.icon
          return (
            <Card key={category.id} className="border-1 border-default-200 dark:border-default-100">
              <CardBody className="p-6">
                <div className="flex items-center justify-between mb-4">
                  <div className={`p-3 rounded-xl bg-${category.color}-100 dark:bg-${category.color}-900/20`}>
                    <Icon className={`h-8 w-8 text-${category.color}-600 dark:text-${category.color}-400`} />
                  </div>
                  <div className="text-right">
                    <div className="text-2xl font-bold">{category.stats.total}</div>
                    <div className="text-xs text-default-500">Всего достижений</div>
                  </div>
                </div>
                
                <h3 className="font-semibold mb-1">{category.title}</h3>
                <p className="text-default-500 text-sm mb-3">{category.description}</p>
                
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>За этот год:</span>
                    <span className="font-medium">{category.stats.thisYear}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span>Рост:</span>
                    <span className="font-medium text-success">+{category.stats.growth}%</span>
                  </div>
                  <Progress 
                    value={category.stats.growth} 
                    size="sm" 
                    color="success" 
                    className="mt-2"
                  />
                </div>
              </CardBody>
            </Card>
          )
        })}
      </div>

      {/* Recent Achievements */}
      <div className="mb-12">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-3xl font-bold">Последние достижения</h2>
          <Button variant="flat" color="primary">
            Все достижения
          </Button>
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {recentAchievements.map((achievement) => (
            <Card key={achievement.id} className="border-1 border-default-200 dark:border-default-100">
              <CardHeader className="p-0">
                <Image
                  removeWrapper
                  alt={achievement.title}
                  className="w-full h-48 object-cover rounded-t-lg"
                  src={achievement.image}
                />
              </CardHeader>
              <CardBody className="px-6 py-4">
                <div className="flex items-center justify-between mb-2">
                  <Chip size="sm" variant="flat" color="primary">
                    {achievement.category}
                  </Chip>
                  <Badge color="warning" variant="solid">
                    {achievement.award}
                  </Badge>
                </div>
                
                <h3 className="text-xl font-bold mb-2">{achievement.title}</h3>
                
                <div className="space-y-2 text-sm text-default-600 mb-3">
                  <div className="flex items-center gap-2">
                    <Calendar className="h-4 w-4" />
                    <span>{achievement.date}</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <Target className="h-4 w-4" />
                    <span>{achievement.level}</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <MapPin className="h-4 w-4" />
                    <span>{achievement.place}</span>
                  </div>
                </div>
                
                <div className="flex flex-wrap gap-1 mb-3">
                  {achievement.participants.map((participant, index) => (
                    <Chip key={index} size="sm" variant="flat">
                      {participant}
                    </Chip>
                  ))}
                </div>
                
                <Button 
                  color="primary" 
                  variant="ghost" 
                  size="sm" 
                  className="w-full"
                  onPress={() => onNavigate?.(`achievement/${achievement.id}`)}
                >
                  Подробнее
                </Button>
              </CardBody>
            </Card>
          ))}
        </div>
      </div>

      {/* Top Students */}
      <div className="mb-12">
        <h2 className="text-3xl font-bold mb-6">Лучшие ученики</h2>
        <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
          {topStudents.map((student, index) => (
            <Card key={student.id} className="text-center border-1 border-default-200 dark:border-default-100">
              <CardBody className="p-4">
                <div className="relative mb-3">
                  <Avatar 
                    src={student.avatar} 
                    name={student.name}
                    className="w-16 h-16 mx-auto text-large"
                  />
                  {index < 3 && (
                    <div className={`absolute -top-1 -right-1 w-6 h-6 rounded-full flex items-center justify-center text-white text-xs font-bold
                      ${index === 0 ? 'bg-warning' : index === 1 ? 'bg-default-400' : 'bg-default-300'}`}>
                      {index + 1}
                    </div>
                  )}
                </div>
                
                <h3 className="font-semibold text-sm mb-1">{student.name}</h3>
                <p className="text-default-500 text-xs mb-2">{student.specialty}</p>
                
                <div className="space-y-1 text-xs">
                  <div className="flex justify-between">
                    <span>Достижения:</span>
                    <span className="font-medium">{student.achievements}</span>
                  </div>
                  <div className="flex justify-between">
                    <span>Баллы:</span>
                    <span className="font-medium text-primary">{student.points}</span>
                  </div>
                </div>
              </CardBody>
            </Card>
          ))}
        </div>
      </div>

      {/* Call to Action */}
      <div className="text-center">
        <Card className="bg-gradient-to-r from-success/10 to-primary/10 border-1 border-success/20">
          <CardBody className="py-8">
            <div className="flex items-center justify-center gap-3 mb-4">
              <TrendingUp className="h-8 w-8 text-success" />
              <h3 className="text-2xl font-bold">Добавьте новое достижение</h3>
            </div>
            <p className="text-default-600 mb-6 max-w-2xl mx-auto">
              Расскажите о своих успехах или достижениях ваших учеников. 
              Вместе мы создаем историю успеха нашего лицея!
            </p>
            <div className="flex gap-3 justify-center">
              <Button color="primary" size="lg" className="font-medium">
                Добавить достижение
              </Button>
              <Button variant="flat" color="default" size="lg">
                Номинировать ученика
              </Button>
            </div>
          </CardBody>
        </Card>
      </div>
    </div>
  )
}
