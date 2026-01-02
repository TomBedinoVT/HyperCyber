import apiClient from './client'

export interface Entity {
  id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
}

export interface CreateEntityRequest {
  name: string
  description?: string
}

export const entitiesApi = {
  list: async (): Promise<Entity[]> => {
    const response = await apiClient.get<Entity[]>('/entities')
    return response.data
  },

  get: async (id: string): Promise<Entity> => {
    const response = await apiClient.get<Entity>(`/entities/${id}`)
    return response.data
  },

  create: async (data: CreateEntityRequest): Promise<Entity> => {
    const response = await apiClient.post<Entity>('/entities', data)
    return response.data
  },

  update: async (id: string, data: Partial<CreateEntityRequest>): Promise<Entity> => {
    const response = await apiClient.put<Entity>(`/entities/${id}`, data)
    return response.data
  },

  getUsers: async (id: string) => {
    const response = await apiClient.get(`/entities/${id}/users`)
    return response.data
  },
}

