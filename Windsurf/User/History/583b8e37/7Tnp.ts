import type { Donate, DonateTypesMap } from "./types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080/api/v1/donate";

export async function fetchDonateTypes(): Promise<DonateTypesMap> {
  const res = await fetch(`${API_BASE}/get-all-types`);
  if (!res.ok) throw new Error("Failed to fetch donate types");
  return res.json();
}

export async function payForDonation(donate: Partial<Donate>): Promise<Donate> {
  const res = await fetch(`${API_BASE}/pay`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(donate),
  });
  if (!res.ok) throw new Error("Payment request failed");
  return res.json();
}

export async function fetchRecentDonations(): Promise<Donate[]> {
  const res = await fetch(`${API_BASE}/recent`);
  if (!res.ok) throw new Error("Failed to fetch recent donations");
  return res.json();
}
