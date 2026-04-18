"use client"

// HeroUIProvider not available, using simple wrapper
import { ThemeProvider as NextThemesProvider } from "next-themes"
import { ThemeProviderProps } from "next-themes/dist/types"
import { useRouter } from "next/navigation"
import { useEffect, useState } from "react"

interface ProvidersProps {
  children: React.ReactNode
  themeProps?: ThemeProviderProps
}

declare module "@heroui/react" {
  interface useRouter {
    push: (href: string) => void
    replace: (href: string) => void
    prefetch: (href: string) => void
    back: () => void
    forward: () => void
    refresh: () => void
  }
}

export function Providers({ children, themeProps }: ProvidersProps) {
  const router = useRouter()

  const [mounted, setMounted] = useState(false)

  useEffect(() => {
    setMounted(true)
  }, [])

  if (!mounted) {
    return null
  }

  return (
    <HeroUIProvider navigate={router.push} useHref={(path) => path}>
      <NextThemesProvider {...themeProps}>
        {children}
      </NextThemesProvider>
    </HeroUIProvider>
  )
}
