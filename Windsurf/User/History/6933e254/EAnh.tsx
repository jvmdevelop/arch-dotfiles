"use client"

import { useState, useEffect } from "react"
import { useRouter } from "next/navigation"
import { GraduationCap } from "lucide-react"
import { toast } from "sonner"
import { useAuth } from "@/lib/auth-context"
import { Button, Card, CardBody, CardHeader, Tabs, Tab, Input, Spinner } from "@heroui/react"

export default function LoginPage() {
  const { user, login, register, isLoading: authLoading } = useAuth()
  const router = useRouter()
  const [isLoading, setIsLoading] = useState(false)

  // Login form state
  const [loginUsername, setLoginUsername] = useState("")
  const [loginPassword, setLoginPassword] = useState("")

  // Register form state
  const [regUsername, setRegUsername] = useState("")
  const [regEmail, setRegEmail] = useState("")
  const [regPassword, setRegPassword] = useState("")

  useEffect(() => {
    if (user && !authLoading) {
      router.push("/news")
    }
  }, [user, authLoading, router])

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsLoading(true)
    try {
      await login(loginUsername, loginPassword)
      toast.success("Добро пожаловать!")
      router.push("/news")
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Ошибка входа")
    } finally {
      setIsLoading(false)
    }
  }

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsLoading(true)
    try {
      await register(regUsername, regPassword, regEmail)
      toast.success("Аккаунт успешно создан!")
      router.push("/news")
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Ошибка регистрации")
    } finally {
      setIsLoading(false)
    }
  }

  if (authLoading || user) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-secondary-50">
        <Spinner size="lg" color="primary" />
      </div>
    )
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-secondary-50 p-4">
      <Card className="w-full max-w-md shadow-2xl border-0">
        <CardHeader className="text-center pb-0">
          <div className="flex justify-center mb-4">
            <div className="p-4 bg-gradient-to-r from-primary to-secondary rounded-full shadow-lg">
              <GraduationCap className="size-10 text-white" />
            </div>
          </div>
          <h1 className="text-3xl font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
            Лицей
          </h1>
          <p className="text-default-500 mt-2">Образовательная платформа</p>
        </CardHeader>
        <CardBody className="pt-6">
          <Tabs aria-label="Auth tabs" className="w-full">
            <Tab key="login" title="Вход">
              <form onSubmit={handleLogin} className="space-y-6 mt-6">
                <Input
                  label="Имя пользователя"
                  placeholder="Введите имя пользователя"
                  value={loginUsername}
                  onChange={(e) => setLoginUsername(e.target.value)}
                  variant="bordered"
                  size="lg"
                  isRequired
                  startContent={
                    <div className="pointer-events-none flex items-center">
                      <span className="text-default-400 text-sm">👤</span>
                    </div>
                  }
                />
                <Input
                  label="Пароль"
                  type="password"
                  placeholder="Введите пароль"
                  value={loginPassword}
                  onChange={(e) => setLoginPassword(e.target.value)}
                  variant="bordered"
                  size="lg"
                  isRequired
                  startContent={
                    <div className="pointer-events-none flex items-center">
                      <span className="text-default-400 text-sm">🔒</span>
                    </div>
                  }
                />
                <Button 
                  type="submit" 
                  className="w-full bg-gradient-to-r from-primary to-secondary text-white font-semibold shadow-lg" 
                  size="lg"
                  disabled={isLoading}
                >
                  {isLoading ? <Spinner size="sm" color="white" /> : "Войти"}
                </Button>
              </form>
            </Tab>

            <Tab key="register" title="Регистрация">
              <form onSubmit={handleRegister} className="space-y-6 mt-6">
                <Input
                  label="Имя пользователя"
                  placeholder="Придумайте имя пользователя"
                  value={regUsername}
                  onChange={(e) => setRegUsername(e.target.value)}
                  variant="bordered"
                  size="lg"
                  isRequired
                  startContent={
                    <div className="pointer-events-none flex items-center">
                      <span className="text-default-400 text-sm">👤</span>
                    </div>
                  }
                />
                <Input
                  label="Email"
                  type="email"
                  placeholder="Введите ваш email"
                  value={regEmail}
                  onChange={(e) => setRegEmail(e.target.value)}
                  variant="bordered"
                  size="lg"
                  isRequired
                  startContent={
                    <div className="pointer-events-none flex items-center">
                      <span className="text-default-400 text-sm">📧</span>
                    </div>
                  }
                />
                <Input
                  label="Пароль"
                  type="password"
                  placeholder="Придумайте пароль"
                  value={regPassword}
                  onChange={(e) => setRegPassword(e.target.value)}
                  variant="bordered"
                  size="lg"
                  isRequired
                  startContent={
                    <div className="pointer-events-none flex items-center">
                      <span className="text-default-400 text-sm">🔒</span>
                    </div>
                  }
                />
                <Button 
                  type="submit" 
                  className="w-full bg-gradient-to-r from-primary to-secondary text-white font-semibold shadow-lg" 
                  size="lg"
                  disabled={isLoading}
                >
                  {isLoading ? <Spinner size="sm" color="white" /> : "Создать аккаунт"}
                </Button>
              </form>
            </Tab>
          </Tabs>
        </CardBody>
      </Card>
    </div>
  )
}
