export interface Client {
  id: number
  name: string
  phone: string
  email: string
  gstin: string
  address: string
  balance: number
}

export interface CreateClientRequest {
  name: string
  phone: string
  email: string
  gstin: string
  address: string
}
