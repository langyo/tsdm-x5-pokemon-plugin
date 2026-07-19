const BASE = '/plugin.php?id=pokemon:api'

async function request<T>(endpoint: string, params: Record<string, any> = {}): Promise<T> {
  const url = `${BASE}&endpoint=${endpoint}&${new URLSearchParams(params)}`
  const res = await fetch(url, { credentials: 'include' })
  if (!res.ok) throw new Error(`HTTP ${res.status}`)
  const json = await res.json()
  if (!json.success) throw new Error(json.error || json.reason || 'Unknown error')
  return json as T
}

async function post<T>(endpoint: string, body: Record<string, any>): Promise<T> {
  const form = new FormData()
  for (const [k, v] of Object.entries(body)) {
    form.append(k, typeof v === 'object' ? JSON.stringify(v) : String(v))
  }
  const res = await fetch(`${BASE}&endpoint=${endpoint}`, {
    method: 'POST',
    credentials: 'include',
    body: form,
  })
  if (!res.ok) throw new Error(`HTTP ${res.status}`)
  const json = await res.json()
  if (!json.success) throw new Error(json.error || json.reason || 'Unknown error')
  return json as T
}

async function postAdmin(action: string, params: Record<string, any> = {}): Promise<any> {
  const body = new URLSearchParams()
  body.append('action', action)
  for (const [k, v] of Object.entries(params)) {
    body.append(k, typeof v === 'object' ? JSON.stringify(v) : String(v))
  }
  const res = await fetch(`${BASE}&endpoint=admin`, {
    method: 'POST',
    credentials: 'include',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: body.toString(),
  })
  if (!res.ok) throw new Error(`HTTP ${res.status}`)
  return res.json()
}

export const api = { get: request, post, admin: postAdmin }
