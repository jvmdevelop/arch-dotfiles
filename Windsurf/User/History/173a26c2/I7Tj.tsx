"use client"

import { useState, useEffect } from "react"
import { Plus, Calendar, User } from "lucide-react"
import { toast } from "sonner"
import { useAuth } from "@/lib/auth-context"
import { apiClient } from "@/lib/api"
import { AppShell } from "@/components/app-shell"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { Spinner } from "@/components/ui/spinner"

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
  const [isDialogOpen, setIsDialogOpen] = useState(false)
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
      toast.success("News created successfully!")
      setIsDialogOpen(false)
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
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-foreground">News</h1>
            <p className="text-muted-foreground mt-1">Stay updated with the latest news</p>
          </div>
          {canCreate && (
            <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
              <DialogTrigger asChild>
                <Button>
                  <Plus className="size-4" />
                  Create News
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Create News</DialogTitle>
                  <DialogDescription>Add a new news article to the platform.</DialogDescription>
                </DialogHeader>
                <form onSubmit={handleSubmit} className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="title">Title</Label>
                    <Input
                      id="title"
                      value={title}
                      onChange={(e) => setTitle(e.target.value)}
                      placeholder="Enter news title"
                      required
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="content">Content</Label>
                    <Textarea
                      id="content"
                      value={content}
                      onChange={(e) => setContent(e.target.value)}
                      placeholder="Write the news content..."
                      rows={5}
                      required
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="image">Image (optional)</Label>
                    <Input
                      id="image"
                      type="file"
                      accept="image/*"
                      onChange={(e) => setImage(e.target.files?.[0] || null)}
                    />
                  </div>
                  <DialogFooter>
                    <Button type="submit" disabled={isSubmitting}>
                      {isSubmitting ? <Spinner className="size-4" /> : "Create"}
                    </Button>
                  </DialogFooter>
                </form>
              </DialogContent>
            </Dialog>
          )}
        </div>

        {isLoading ? (
          <div className="flex justify-center py-12">
            <Spinner className="size-8 text-primary" />
          </div>
        ) : news.length === 0 ? (
          <Card className="border-primary/20">
            <CardContent className="py-12 text-center">
              <p className="text-muted-foreground">No news articles yet.</p>
            </CardContent>
          </Card>
        ) : (
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {news.map((item) => (
              <Card key={item.id} className="border-primary/20 shadow-md overflow-hidden">
                {item.imageUrl && (
                  <div className="aspect-video overflow-hidden">
                    <img
                      src={item.imageUrl}
                      alt={item.title}
                      className="w-full h-full object-cover"
                    />
                  </div>
                )}
                <CardHeader>
                  <CardTitle className="line-clamp-2">{item.title}</CardTitle>
                  <CardDescription className="flex items-center gap-4 text-xs">
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
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <p className="text-sm text-muted-foreground line-clamp-3">{item.content}</p>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </div>
    </AppShell>
  )
}
