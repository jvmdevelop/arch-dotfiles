"use client"

import Link from "next/link"
import { usePathname } from "next/navigation"
import { useAuth } from "@/lib/auth-context"
import { Button } from "@/components/ui/button"
import { 
  Home, 
  Newspaper, 
  Building, 
  GraduationCap, 
  LogOut, 
  User,
  Menu,
  X
} from "lucide-react"
import { useState } from "react"
import { useTheme } from "next-themes"
import { Moon, Sun } from "lucide-react"

const navigation = [
  { name: "Новости", href: "/news", icon: Newspaper },
  { name: "Музей", href: "/museum", icon: Building },
  { name: "Классы", href: "/classrooms", icon: GraduationCap },
]

export function Navigation() {
  const pathname = usePathname()
  const { user, logout } = useAuth()
  const { theme, setTheme } = useTheme()
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false)

  const handleLogout = async () => {
    await logout()
  }

  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark")
  }

  return (
    <header className="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container flex h-16 items-center justify-between px-4 lg:px-8">
        <div className="flex items-center space-x-4">
          <Link href="/news" className="flex items-center space-x-3 group">
            <div className="relative">
              <img 
                src="/img.png" 
                alt="Лицей" 
                className="h-8 w-8 rounded-lg object-cover transition-transform group-hover:scale-105"
              />
              <div className="absolute -inset-1 bg-primary/20 rounded-lg blur-sm opacity-0 group-hover:opacity-100 transition-opacity" />
            </div>
            <span className="hidden font-bold text-xl sm:inline-block bg-gradient-to-r from-primary to-primary/70 bg-clip-text text-transparent">
              Лицей
            </span>
          </Link>
        </div>

        <nav className="hidden md:flex items-center space-x-6">
          {navigation.map((item) => {
            const Icon = item.icon
            return (
              <Link
                key={item.name}
                href={item.href}
                className={`flex items-center space-x-2 text-sm font-medium transition-colors hover:text-primary ${
                  pathname === item.href
                    ? "text-primary"
                    : "text-muted-foreground"
                }`}
              >
                <Icon className="h-4 w-4" />
                <span>{item.name}</span>
              </Link>
            )
          })}
        </nav>

        {/* Right side actions */}
        <div className="flex items-center space-x-2">
          {/* Theme toggle */}
          <Button
            variant="ghost"
            size="icon"
            onClick={toggleTheme}
            className="h-9 w-9"
          >
            <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
            <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
            <span className="sr-only">Переключить тему</span>
          </Button>

          {/* User menu */}
          {user && (
            <div className="hidden sm:flex items-center space-x-2">
              <div className="flex items-center space-x-2 text-sm text-muted-foreground">
                <User className="h-4 w-4" />
                <span>{(user as any)?.name || user.email}</span>
              </div>
              <Button
                variant="ghost"
                size="icon"
                onClick={handleLogout}
                className="h-9 w-9"
              >
                <LogOut className="h-4 w-4" />
                <span className="sr-only">Выйти</span>
              </Button>
            </div>
          )}

          {/* Mobile menu button */}
          <Button
            variant="ghost"
            size="icon"
            className="md:hidden h-9 w-9"
            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
          >
            {isMobileMenuOpen ? (
              <X className="h-4 w-4" />
            ) : (
              <Menu className="h-4 w-4" />
            )}
            <span className="sr-only">Меню</span>
          </Button>
        </div>
      </div>

      {/* Mobile Navigation */}
      {isMobileMenuOpen && (
        <div className="md:hidden border-t bg-background">
          <div className="container px-4 py-4 space-y-3">
            {navigation.map((item) => {
              const Icon = item.icon
              return (
                <Link
                  key={item.name}
                  href={item.href}
                  className={`flex items-center space-x-3 text-sm font-medium transition-colors hover:text-primary py-2 ${
                    pathname === item.href
                      ? "text-primary"
                      : "text-muted-foreground"
                  }`}
                  onClick={() => setIsMobileMenuOpen(false)}
                >
                  <Icon className="h-4 w-4" />
                  <span>{item.name}</span>
                </Link>
              )
            })}
            
            {user && (
              <div className="pt-3 border-t space-y-3">
                <div className="flex items-center space-x-3 text-sm text-muted-foreground py-2">
                  <User className="h-4 w-4" />
                  <span>{(user as any)?.name || user.email}</span>
                </div>
                <Button
                  variant="ghost"
                  className="w-full justify-start text-sm"
                  onClick={handleLogout}
                >
                  <LogOut className="h-4 w-4 mr-3" />
                  Выйти
                </Button>
              </div>
            )}
          </div>
        </div>
      )}
    </header>
  )
}
