import type { Metadata, Viewport } from "next"
import { Inter, JetBrains_Mono, Pixelify_Sans } from "next/font/google"
import { Analytics } from "@vercel/analytics/next"
import { ThemeProvider } from "@/components/theme/theme-provider"
import { Header } from "@/components/layout/header"
import { Footer } from "@/components/layout/footer"
import { SplashScreen } from "@/components/loading-spinner"
import "./globals.css"

const pixelify = Pixelify_Sans({
    subsets: ["latin"],
    variable: "--font-pixelify",
    weight: ["400", "500", "600", "700"]
})

const inter = Inter({ subsets: ["latin", "cyrillic"], variable: "--font-inter" })
const jetbrains = JetBrains_Mono({
    subsets: ["latin", "cyrillic"],
    variable: "--font-jetbrains",
})

export const metadata: Metadata = {
    title: {
        default: "Walto RPG",
        template: "%s | Walto RPG",
    },
    description:
        "Walto RPG - мир приключений и магии. Поддержи проект и стань частью легенды.",
}

export const viewport: Viewport = {
    themeColor: "#381E49",
}

export default function RootLayout({
                                       children,
                                   }: Readonly<{
    children: React.ReactNode
}>) {
    return (
        <html lang="ru" suppressHydrationWarning>
        <body
            className={`${pixelify.variable} ${inter.variable} ${jetbrains.variable} font-sans antialiased`}
        >
        <ThemeProvider>
            <SplashScreen />
            <div className="flex min-h-screen flex-col">
                <Header />
                <main className="flex-1">{children}</main>
                <Footer />
            </div>
        </ThemeProvider>
        <Analytics />
        </body>
        </html>
    )
}