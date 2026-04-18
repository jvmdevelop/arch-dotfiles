"use client"

import { useState, useEffect } from "react"
import { Plus, Calendar, User } from "lucide-react"
import { toast } from "sonner"
import { useAuth } from "@/lib/auth-context"
import { apiClient } from "@/lib/api"
import { AppShell } from "@/components/app-shell"
import { Button, Card, CardHeader, Modal, ModalHeader, ModalBody, ModalFooter, Input, Spinner } from "@heroui/react"

interface NewsAuthor {
  username: string
  email: string
}

interface NewsItem {
  id: number
  title: string
  content: string
  imageUrl?: string
  user?: NewsAuthor
  createdAt: string
}

export default function NewsPage() {
  const { hasRole } = useAuth()
  const [news, setNews] = useState<NewsItem[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [isSubmitting, setIsSubmitting] = useState(false)

  const [title, setTitle] = useState("")
  const [content, setContent] = useState("")
  const [image, setImage] = useState<File | null>(null)

  const canCreate = hasRole("TEACHER")

  const fetchNews = async () => {
    try {
      const { data } = await apiClient.get<NewsItem[]>("/api/news/get-all")
      setNews(data)
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Failed to fetch news")
    } finally {
      setIsLoading(false)
    }
  }

  useEffect(() => {
    fetchNews()
  }, [])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsSubmitting(true)
    try {
      const formData = new FormData()
      formData.append("title", title)
      formData.append("content", content)
      if (image) {
        formData.append("image", image)
      }
      await apiClient.post("/api/news/new", formData)
      toast.success("Новость успешно создана!")
      setIsModalOpen(false)
      setTitle("")
      setContent("")
      setImage(null)
      fetchNews()
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Не удалось создать новость")
    } finally {
      setIsSubmitting(false)
    }
  }

  const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleDateString("ru-RU", {
      year: "numeric",
      month: "long",
      day: "numeric",
    })
  }

  return (
    <AppShell>
      <div className="space-y-8">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-4xl font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
              Новости
            </h1>
            <p className="text-default-500 mt-2 text-lg">Будьте в курсе последних событий</p>
          </div>
          {canCreate && (
            <Button 
              onPress={() => setIsModalOpen(true)}
              className="bg-gradient-to-r from-primary to-secondary text-white font-semibold shadow-lg"
              size="lg"
              startContent={<Plus className="size-5" />}
            >
              Создать новость
            </Button>
          )}
        </div>

        <Modal 
          isOpen={isModalOpen} 
          onClose={() => setIsModalOpen(false)}
          size="2xl"
          scrollBehavior="inside"
        >
          <ModalHeader className="flex flex-col gap-1">
            <h2 className="text-2xl font-bold">Создать новость</h2>
            <p className="text-default-500 text-sm">Добавьте новую новость на платформу</p>
          </ModalHeader>
          <ModalBody>
            <form onSubmit={handleSubmit} className="space-y-6">
              <Input
                label="Заголовок"
                placeholder="Введите заголовок новости"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                variant="bordered"
                size="lg"
                isRequired
                startContent={
                  <div className="pointer-events-none flex items-center">
                    <span className="text-default-400 text-sm">📰</span>
                  </div>
                }
              />
              <Input
                label="Содержание"
                placeholder="Напишите содержание новости здесь..."
                value={content}
                onChange={(e) => setContent(e.target.value)}
                variant="bordered"
                size="lg"
                multiline
                minRows={6}
                isRequired
              />
              <Input
                label="Изображение"
                type="file"
                accept="image/*"
                onChange={(e) => setImage(e.target.files?.[0] || null)}
                variant="bordered"
                size="lg"
                startContent={
                  <div className="pointer-events-none flex items-center">
                    <span className="text-default-400 text-sm">🖼️</span>
                  </div>
                }
              />
            </form>
          </ModalBody>
          <ModalFooter>
            <Button 
              color="danger" 
              variant="light" 
              onPress={() => setIsModalOpen(false)}
            >
              Отмена
            </Button>
            <Button 
              onPress={() => {
                handleSubmit(new Event('submit') as any)
              }}
              className="bg-gradient-to-r from-primary to-secondary text-white font-semibold"
              disabled={isSubmitting}
            >
              {isSubmitting ? <Spinner size="sm" color="white" /> : "Создать"}
            </Button>
          </ModalFooter>
        </Modal>

        {isLoading ? (
          <div className="flex justify-center py-20">
            <Spinner size="lg" color="primary" />
          </div>
        ) : news.length === 0 ? (
          <Card className="border-0 shadow-sm">
            <div className="py-20 text-center">
              <div className="text-6xl mb-4">📰</div>
              <p className="text-default-500 text-lg">Новостей пока нет.</p>
            </div>
          </Card>
        ) : (
          <div className="grid gap-8 md:grid-cols-2 lg:grid-cols-3">
            {news.map((item) => (
              <Card key={item.id} className="border-0 shadow-lg hover:shadow-xl transition-shadow duration-300 overflow-hidden group">
                {item.imageUrl && (
                  <div className="aspect-video overflow-hidden">
                    <img
                      src={item.imageUrl}
                      alt={item.title}
                      className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                    />
                  </div>
                )}
                <CardHeader className="pb-2">
                  <h3 className="text-xl font-bold line-clamp-2 group-hover:text-primary transition-colors">
                    {item.title}
                  </h3>
                  <div className="flex items-center gap-4 text-xs text-default-500">
                    {item.user && (
                      <span className="flex items-center gap-1">
                        <User className="size-3" />
                        {item.user.username}
                      </span>
                    )}
                    <span className="flex items-center gap-1">
                      <Calendar className="size-3" />
                      {formatDate(item.createdAt)}
                    </span>
                  </div>
                </CardHeader>
                <div className="px-6 pb-6 pt-0">
                  <p className="text-default-600 line-clamp-3">{item.content}</p>
                </div>
              </Card>
            ))}
          </div>
        )}
      </div>
    </AppShell>
  )
}
