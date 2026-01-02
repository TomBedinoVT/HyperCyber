import apiClient from './client'

export interface RegisterEntry {
  id: string
  entity_id: string
  processing_name: string
  purpose: string
  legal_basis: string
  data_categories: string[]
  data_subjects: string[]
  recipients: string[]
  retention_period?: string
  security_measures?: string
  created_at: string
  updated_at: string
}

export interface AccessRequest {
  id: string
  entity_id: string
  requester_name: string
  requester_email: string
  request_type: string
  description?: string
  status: string
  response?: string
  created_at: string
  updated_at: string
  completed_at?: string
}

export interface Breach {
  id: string
  entity_id: string
  breach_date: string
  discovery_date: string
  description: string
  data_categories_affected: string[]
  number_of_subjects?: number
  severity: string
  status: string
  containment_measures?: string
  notification_date?: string
  authority_notified: boolean
  subjects_notified: boolean
  created_at: string
  updated_at: string
}

export const rgpdApi = {
  // Register - New routes with entity_id in path
  getRegister: async (entityId?: string): Promise<RegisterEntry[]> => {
    if (entityId) {
      const response = await apiClient.get<RegisterEntry[]>(`/entities/${entityId}/rgpd/register`)
      return response.data
    }
    // Fallback to old route for backward compatibility
    const response = await apiClient.get<RegisterEntry[]>('/rgpd/register')
    return response.data
  },

  addToRegister: async (entityId: string, data: Partial<RegisterEntry>): Promise<RegisterEntry> => {
    const response = await apiClient.post<RegisterEntry>(`/entities/${entityId}/rgpd/register`, data)
    return response.data
  },

  updateRegisterEntry: async (id: string, data: Partial<RegisterEntry>): Promise<RegisterEntry> => {
    // Use old route for updates (entity_id not needed in path for updates)
    const response = await apiClient.put<RegisterEntry>(`/rgpd/register/${id}`, data)
    return response.data
  },

  // Access Requests
  listAccessRequests: async (entityId?: string): Promise<AccessRequest[]> => {
    if (entityId) {
      const response = await apiClient.get<AccessRequest[]>(`/entities/${entityId}/rgpd/access-requests`)
      return response.data
    }
    const response = await apiClient.get<AccessRequest[]>('/rgpd/access-requests')
    return response.data
  },

  createAccessRequest: async (entityId: string, data: Partial<AccessRequest>): Promise<AccessRequest> => {
    const response = await apiClient.post<AccessRequest>(`/entities/${entityId}/rgpd/access-requests`, data)
    return response.data
  },

  getAccessRequest: async (id: string): Promise<AccessRequest> => {
    const response = await apiClient.get<AccessRequest>(`/rgpd/access-requests/${id}`)
    return response.data
  },

  respondToRequest: async (id: string, data: { status: string; response?: string }): Promise<AccessRequest> => {
    const response = await apiClient.post<AccessRequest>(`/rgpd/access-requests/${id}/respond`, data)
    return response.data
  },

  // Breaches
  listBreaches: async (entityId?: string): Promise<Breach[]> => {
    if (entityId) {
      const response = await apiClient.get<Breach[]>(`/entities/${entityId}/rgpd/breaches`)
      return response.data
    }
    const response = await apiClient.get<Breach[]>('/rgpd/breaches')
    return response.data
  },

  createBreach: async (entityId: string, data: Partial<Breach>): Promise<Breach> => {
    const response = await apiClient.post<Breach>(`/entities/${entityId}/rgpd/breaches`, data)
    return response.data
  },

  getBreach: async (id: string): Promise<Breach> => {
    const response = await apiClient.get<Breach>(`/rgpd/breaches/${id}`)
    return response.data
  },

  updateBreach: async (id: string, data: Partial<Breach>): Promise<Breach> => {
    const response = await apiClient.put<Breach>(`/rgpd/breaches/${id}`, data)
    return response.data
  },
}

