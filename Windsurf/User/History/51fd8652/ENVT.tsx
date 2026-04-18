"use client"

import { useState } from "react"
import type { Metadata } from "next"
import { DonateForm } from "@/components/donate/donate-form"
import { RecentDonations } from "@/components/donate/recent-donations"
import { DonateInstructions } from "@/components/donate/donate-instructions"

export default function DonatePage() {
  const [selectedType, setSelectedType] = useState<string | null>(null)
  const [nickname, setNickname] = useState<string>("")

  return (
    <div className="mx-auto max-w-6xl px-4 py-12">
      <div className="mb-10 text-center">
        <h1 className="text-balance text-4xl font-bold text-foreground">
          {"Донаты"}
        </h1>
        <p className="mt-3 text-muted-foreground">
          {"Донать нам. Донать..."}
        </p>
      </div>

      <div className="grid gap-10 lg:grid-cols-2">
        <DonateForm 
          onSelectedTypeChange={setSelectedType}
          onNicknameChange={setNickname}
        />
        <RecentDonations />
      </div>

      <div id="donate-instructions" className="mt-16">
        <DonateInstructions 
          selectedType={selectedType || undefined}
          nickname={nickname || undefined}
        />
      </div>
    </div>
  )
}
