"use client"

import Link from "next/link"
import {usePathname} from "next/navigation"
import {ThemeToggle} from "@/components/theme/theme-toggle"
import {cn} from "@/lib/utils"
import {Menu, X} from "lucide-react"
import {useState} from "react"
import Image from "next/image";

const NAV_ITEMS = [
    {href: "/", label: "Главная"},
    {href: "/donate", label: "Донат"},
    {href: "/wiki", label: "Вики"},
]

export function Header() {
    const pathname = usePathname()
    const [mobileOpen, setMobileOpen] = useState(false)

    return (
        <header className="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-md">
            <div className="mx-auto flex h-20 max-w-6xl items-center justify-between px-4">
                <Link href="/" className="flex items-center gap-2">
                    <div className="relative h-8 w-8 overflow-hidden rounded">
                        <Image
                            src="/images/walto-avatar.jpg"
                            alt="WaltoRPG"
                            fill
                            className="object-cover"
                            priority
                        />
                    </div>
                    <span className="text-lg font-bold tracking-tight text-foreground">
        Walto RPG
    </span>
                </Link>

                <nav className="hidden items-center gap-1 md:flex">
                    {NAV_ITEMS.map((item) => (
                        <Link
                            key={item.href}
                            href={item.href}
                            className={cn(
                                "rounded-md px-4 py-2 text-sm font-medium transition-colors",
                                pathname === item.href
                                    ? "bg-primary text-primary-foreground"
                                    : "text-muted-foreground hover:bg-secondary hover:text-secondary-foreground"
                            )}
                        >
                            {item.label}
                        </Link>
                    ))}
                    <div className="ml-2">
                        <ThemeToggle/>
                    </div>
                </nav>

                <button
                    className="inline-flex items-center justify-center rounded-md p-2 text-foreground md:hidden"
                    onClick={() => setMobileOpen(!mobileOpen)}
                    aria-label="Toggle menu"
                >
                    {mobileOpen ? <X className="h-5 w-5"/> : <Menu className="h-5 w-5"/>}
                </button>
            </div>

            {mobileOpen && (
                <div className="border-t border-border bg-background px-4 pb-4 md:hidden">
                    <nav className="flex flex-col gap-1 pt-2">
                        {NAV_ITEMS.map((item) => (
                            <Link
                                key={item.href}
                                href={item.href}
                                onClick={() => setMobileOpen(false)}
                                className={cn(
                                    "rounded-md px-4 py-2.5 text-sm font-medium transition-colors",
                                    pathname === item.href
                                        ? "bg-primary text-primary-foreground"
                                        : "text-muted-foreground hover:bg-secondary hover:text-secondary-foreground"
                                )}
                            >
                                {item.label}
                            </Link>
                        ))}
                        <div className="pt-2">
                            <ThemeToggle/>
                        </div>
                    </nav>
                </div>
            )}
        </header>
    )
}
