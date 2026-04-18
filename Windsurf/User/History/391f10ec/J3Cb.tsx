"use client"

import { useState } from "react"
import useSWR from "swr"
import { Heart, ExternalLink, Loader2 } from "lucide-react"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { DonateStatus } from "@/lib/types"
import type { DonateTypesMap } from "@/lib/types"
import { fetchDonateTypes, payForDonation } from "@/lib/api"

interface DonateFormProps {
  onSelectedTypeChange?: (type: string | null) => void
  onNicknameChange?: (nickname: string) => void
}

export function DonateForm({ onSelectedTypeChange, onNicknameChange }: DonateFormProps) {
  const {
    data: donateTypes,
    isLoading: typesLoading,
    error: typesError,
  } = useSWR<DonateTypesMap>("donate-types", fetchDonateTypes)

  const [nickname, setNickname] = useState("")
  const [selectedType, setSelectedType] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)
  const [paymentUrl, setPaymentUrl] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const selectedPrice = selectedType && donateTypes ? donateTypes[selectedType] : null

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    setError(null)
    setPaymentUrl(null)

    if (!nickname.trim()) {
      setError("Введите никнейм")
      return
    }
    if (!selectedType) {
      setError("Выберите тип доната")
      return
    }

    setLoading(true)
    try {
      const result = await payForDonation({
        nickname: nickname.trim(),
        type: selectedType,
        status: DonateStatus.UNPAID,
      })
      if (result.paymentUrl) {
        setPaymentUrl(result.paymentUrl)
      }
    } catch {
      setError("Не удалось создать платёж. Попробуйте позже.")
    } finally {
      setLoading(false)
    }
  }

  if (typesLoading) {
    return (
      <Card className="border-border bg-card">
        <CardContent className="flex items-center justify-center py-16">
          <Loader2 className="h-8 w-8 animate-spin text-primary" />
        </CardContent>
      </Card>
    )
  }

  if (typesError || !donateTypes) {
    return (
      <Card className="border-border bg-card">
        <CardContent className="py-16 text-center">
          <p className="text-destructive">
            Не удалось загрузить типы донатов. Попробуйте обновить страницу.
          </p>
        </CardContent>
      </Card>
    )
  }

  const typeEntries = Object.entries(donateTypes)

  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col gap-3">
        <h2 className="text-lg font-semibold text-foreground">
          Выберите тип поддержки
        </h2>
        <div className="grid gap-3 sm:grid-cols-2">
          {typeEntries.map(([title, price]) => {
            const isActive = selectedType === title
            return (
              <button
                key={title}
                type="button"
                onClick={() => setSelectedType(title)}
                className={`group relative cursor-pointer rounded-xl border-2 p-5 text-left transition-all ${
                  isActive
                    ? "border-primary bg-primary/10 shadow-lg shadow-primary/10"
                    : "border-border bg-card hover:border-primary/40 hover:bg-primary/5"
                }`}
              >
                <div className="mb-3 flex h-9 w-9 items-center justify-center rounded-lg bg-primary/10">
                  <Heart
                    className={`h-4 w-4 transition-colors ${
                      isActive ? "text-primary" : "text-muted-foreground group-hover:text-primary"
                    }`}
                  />
                </div>
                <p
                  className={`text-sm font-medium leading-snug ${
                    isActive ? "text-foreground" : "text-foreground"
                  }`}
                >
                  {title}
                </p>
                <p className="mt-2 text-2xl font-bold text-foreground">
                  {price}{" "}
                  <span className="text-sm font-normal text-muted-foreground">
                    RUB
                  </span>
                </p>
                {isActive && (
                  <span className="absolute right-3 top-3 flex h-5 w-5 items-center justify-center rounded-full bg-primary text-[10px] text-primary-foreground">
                    {"✓"}
                  </span>
                )}
              </button>
            )
          })}
        </div>
      </div>

      <Card className="border-border bg-card">
        <CardHeader>
          <CardTitle className="text-foreground">Оформление</CardTitle>
          <CardDescription className="text-muted-foreground">
            Введите никнейм и перейдите к оплате
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="flex flex-col gap-5">
            <div className="flex flex-col gap-2">
              <Label htmlFor="nickname" className="text-foreground">
                Никнейм
              </Label>
              <Input
                id="nickname"
                placeholder="Введите ваш никнейм"
                value={nickname}
                onChange={(e) => setNickname(e.target.value)}
                className="border-border bg-secondary text-foreground placeholder:text-muted-foreground"
                disabled={loading}
              />
            </div>

            {selectedPrice !== null && (
              <div className="flex items-baseline justify-between rounded-lg bg-secondary px-4 py-3">
                <span className="text-sm text-muted-foreground">Сумма</span>
                <span className="text-2xl font-bold text-foreground">
                  {selectedPrice}{" "}
                  <span className="text-base font-normal text-muted-foreground">
                    RUB
                  </span>
                </span>
              </div>
            )}

            {error && (
              <p className="rounded-md bg-destructive/10 px-3 py-2 text-sm text-destructive">
                {error}
              </p>
            )}

            {paymentUrl && (
              <a
                href={paymentUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 rounded-md bg-primary/10 px-3 py-2 text-sm text-primary hover:bg-primary/20"
              >
                <ExternalLink className="h-4 w-4" />
                Перейти к оплате
              </a>
            )}

            <Button
              type="button"
              onClick={() => {
                const instructionsSection = document.getElementById('donate-instructions')
                if (instructionsSection) {
                  instructionsSection.scrollIntoView({ behavior: 'smooth' })
                }
              }}
              disabled={!selectedType}
              className="gap-2 bg-primary text-primary-foreground hover:bg-primary/90"
            >
              <Heart className="h-4 w-4" />
              {selectedType
                ? `Поддержать за ${selectedPrice} RUB`
                : "Выберите тип доната"}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}
