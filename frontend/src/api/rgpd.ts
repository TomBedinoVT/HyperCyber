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
  // Register
  getRegister: async (entityId?: string): Promise<RegisterEntry[]> => {
    const params = entityId ? { entity_id: entityId } : {}
    const response = await apiClient.get<RegisterEntry[]>('/rgpd/register', { params })
    return response.data
  },

  addToRegister: async (entityId: string, data: Partial<RegisterEntry>): Promise<RegisterEntry> => {
    const response = await apiClient.post<RegisterEntry>('/rgpd/register', data, {
      params: { entity_id: entityId },
    })
    return response.data
  },

  updateRegisterEntry: async (id: string, data: Partial<RegisterEntry>): Promise<RegisterEntry> => {
    const response = await apiClient.put<RegisterEntry>(`/rgpd/register/${id}`, data)
    return response.data
  },

  // Access Requests
  listAccessRequests: async (entityId?: string): Promise<AccessRequest[]> => {
    const params = entityId ? { entity_id: entityId } : {}
    const response = await apiClient.get<AccessRequest[]>('/rgpd/access-requests', { params })
    return response.data
  },

  createAccessRequest: async (entityId: string, data: Partial<AccessRequest>): Promise<AccessRequest> => {
    const response = await apiClient.post<AccessRequest>('/rgpd/access-requests', data, {
      params: { entity_id: entityId },
    })
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
    const params = entityId ? { entity_id: entityId } : {}
    const response = await apiClient.get<Breach[]>('/rgpd/breaches', { params })
    return response.data
  },

  createBreach: async (entityId: string, data: Partial<Breach>): Promise<Breach> => {
    const response = await apiClient.post<Breach>('/rgpd/breaches', data, {
      params: { entity_id: entityId },
    })
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

