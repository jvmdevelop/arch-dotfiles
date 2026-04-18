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
  const [isMenuOpen, setIsMenuOpen] = useState(false)

  const handleLogout = async () => {
    await logout()
  }

  return (
    <Navbar 
      isBordered 
      isBlurred 
      className="backdrop-blur-md bg-background/70"
      maxWidth="full"
    >
      <NavbarBrand>
        <Link href="/news" className="flex items-center group">
          <div className="relative h-12 w-auto flex items-center">
            <img 
              src="/img.png" 
              alt="Лицей" 
              className="max-h-full object-contain transition-transform group-hover:scale-105"
            />
          </div>
        </Link>
      </NavbarBrand>

      <NavbarContent className="hidden md:flex gap-4" justify="center">
        {navigation.map((item) => {
          const Icon = item.icon
          const isActive = pathname === item.href
          return (
            <NavbarItem key={item.name} isActive={isActive}>
              <Link
                href={item.href}
                className="flex items-center gap-2 text-foreground hover:text-primary transition-colors"
              >
                <Icon className="h-4 w-4" />
                <span>{item.name}</span>
              </Link>
            </NavbarItem>
          )
        })}
      </NavbarContent>

      <NavbarContent justify="end">
        {/* Theme toggle */}
        <Switch
          isSelected={theme === "dark"}
          onValueChange={(isSelected) => setTheme(isSelected ? "dark" : "light")}
          size="sm"
          color="primary"
          startContent={<Sun className="h-4 w-4" />}
          endContent={<Moon className="h-4 w-4" />}
        />

        {/* User menu */}
        {user && (
          <Dropdown placement="bottom-end">
            <DropdownTrigger>
              <Button
                isIconOnly
                radius="full"
                variant="light"
                className="hidden sm:flex"
              >
                <Avatar 
                  size="sm" 
                  src="" 
                  name={(user as any)?.name || user.email}
                  className="w-6 h-6 text-tiny"
                />
              </Button>
            </DropdownTrigger>
            <DropdownMenu aria-label="User menu" variant="flat">
              <DropdownItem key="profile" className="h-14 gap-2">
                <p className="font-semibold">Вошли как</p>
                <p className="font-normal">{(user as any)?.name || user.email}</p>
              </DropdownItem>
              <DropdownItem key="logout" color="danger" onClick={handleLogout}>
                <div className="flex items-center gap-2">
                  <LogOut className="h-4 w-4" />
                  Выйти
                </div>
              </DropdownItem>
            </DropdownMenu>
          </Dropdown>
        )}

        <NavbarMenuToggle
          aria-label={isMenuOpen ? "Закрыть меню" : "Открыть меню"}
          className="md:hidden"
        />
      </NavbarContent>

      <NavbarMenu>
        {navigation.map((item) => {
          const Icon = item.icon
          return (
            <NavbarMenuItem key={item.name}>
              <Link
                href={item.href}
                className="flex items-center gap-3 text-foreground hover:text-primary transition-colors py-2"
                onClick={() => setIsMenuOpen(false)}
              >
                <Icon className="h-4 w-4" />
                <span>{item.name}</span>
              </Link>
            </NavbarMenuItem>
          )
        })}
        
        {user && (
          <>
            <Divider className="my-2" />
            <NavbarMenuItem>
              <div className="flex items-center gap-3 py-2">
                <Avatar 
                  size="sm" 
                  src="" 
                  name={(user as any)?.name || user.email}
                  className="w-6 h-6 text-tiny"
                />
                <span className="font-medium">{(user as any)?.name || user.email}</span>
              </div>
            </NavbarMenuItem>
            <NavbarMenuItem>
              <Button
                variant="flat"
                color="danger"
                className="w-full justify-start"
                onClick={() => {
                  handleLogout()
                  setIsMenuOpen(false)
                }}
              >
                <LogOut className="h-4 w-4 mr-2" />
                Выйти
              </Button>
            </NavbarMenuItem>
          </>
        )}
      </NavbarMenu>
    </Navbar>
  )
}
