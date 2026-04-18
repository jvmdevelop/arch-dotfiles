"use client"

import { 
  Card,
  CardHeader,
  CardBody,
  CardFooter,
  Image,
  Button,
  Chip,
  Avatar,
  Spacer
} from "@nextui-org/react"
import { 
  Calendar, 
  User, 
  ArrowRight,
  Heart,
  MessageCircle,
  Share2,
  Eye
} from "lucide-react"
import { formatDistanceToNow } from "date-fns"
import { ru } from "date-fns/locale"

interface HeroCardProps {
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
  onReadMore?: (id: number) => void
  onLike?: (id: number) => void
  onComment?: (id: number) => void
  onShare?: (id: number) => void
}

export function HeroCard({ 
  id, 
  title, 
  content, 
  imageUrl, 
  authorName, 
  authorAvatar,
  createdAt,
  views = 0,
  likes = 0,
  comments = 0,
  onReadMore,
  onLike,
  onComment,
  onShare
}: HeroCardProps) {
  return (
    <Card 
      className="w-full hover:shadow-xl transition-all duration-300 border-1 border-default-200 dark:border-default-100"
      isHoverable
      isPressable
      onPress={() => onReadMore?.(id)}
      radius="lg"
    >
      <CardHeader className="p-0">
        {imageUrl && (
          <div className="relative">
            <Image
              removeWrapper
              alt={title}
              className="w-full h-48 object-cover rounded-t-lg"
              src={imageUrl}
            />
            <div className="absolute top-2 right-2">
              <Chip 
                variant="solid" 
                color="primary" 
                size="sm"
                startContent={<Eye className="h-3 w-3" />}
              >
                {views}
              </Chip>
            </div>
          </div>
        )}
      </CardHeader>
      
      <CardBody className="px-6 py-4">
        <div className="flex items-center justify-between mb-3">
          <Chip 
            variant="flat" 
            color="primary" 
            size="sm"
            startContent={<Calendar className="h-3 w-3" />}
          >
            {formatDistanceToNow(new Date(createdAt), { 
              addSuffix: true, 
              locale: ru 
            })}
          </Chip>
          
          <div className="flex gap-1">
            <Button
              isIconOnly
              size="sm"
              variant="light"
              color="danger"
              onPress={() => onLike?.(id)}
            >
              <Heart className="h-4 w-4" />
            </Button>
            <Button
              isIconOnly
              size="sm"
              variant="light"
              onPress={() => onComment?.(id)}
            >
              <MessageCircle className="h-4 w-4" />
            </Button>
            <Button
              isIconOnly
              size="sm"
              variant="light"
              onPress={() => onShare?.(id)}
            >
              <Share2 className="h-4 w-4" />
            </Button>
          </div>
        </div>
        
        <h3 className="text-xl font-bold mb-2 line-clamp-2 text-foreground">
          {title}
        </h3>
        
        <p className="text-default-600 text-sm line-clamp-3 mb-4">
          {content}
        </p>
        
        {authorName && (
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Avatar 
                size="sm" 
                src={authorAvatar}
                name={authorName}
                className="w-6 h-6 text-tiny"
                color="primary"
              />
              <span className="text-default-500 text-sm font-medium">
                {authorName}
              </span>
            </div>
            
            <div className="flex gap-3 text-default-500 text-sm">
              <span className="flex items-center gap-1">
                <Heart className="h-3 w-3" />
                {likes}
              </span>
              <span className="flex items-center gap-1">
                <MessageCircle className="h-3 w-3" />
                {comments}
              </span>
            </div>
          </div>
        )}
      </CardBody>
      
      <CardFooter className="px-6 py-4">
        <Button
          color="primary"
          variant="ghost"
          className="w-full font-medium"
          endContent={<ArrowRight className="h-4 w-4" />}
          onPress={() => onReadMore?.(id)}
        >
          Читать далее
        </Button>
      </CardFooter>
    </Card>
  )
}
