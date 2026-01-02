import apiClient from './client'

// Endpoint
export interface Endpoint {
  id: string
  name: string
  endpoint_type: string
  description?: string
  address?: string
  metadata?: any
  created_at: string
  updated_at: string
}

export interface CreateEndpointRequest {
  name: string
  endpoint_type: string
  description?: string
  address?: string
  metadata?: any
}

// License Key
export interface LicenseKey {
  id: string
  name: string
  license_type: string
  key_value?: string
  file_path?: string
  file_name?: string
  file_size?: number
  storage_type: string
  description?: string
  expires_at?: string
  created_at: string
  updated_at: string
}

export interface CreateLicenseKeyRequest {
  name: string
  license_type: string
  key_value?: string
  description?: string
  expires_at?: string
}

// Software Version
export interface SoftwareVersion {
  id: string
  name: string
  version: string
  description?: string
  release_date?: string
  end_of_life?: string
  metadata?: any
  created_at: string
  updated_at: string
}

export interface CreateSoftwareVersionRequest {
  name: string
  version: string
  description?: string
  release_date?: string
  end_of_life?: string
  metadata?: any
}

// Encryption Algorithm
export interface EncryptionAlgorithm {
  id: string
  name: string
  algorithm_type: string
  key_size?: number
  description?: string
  standard?: string
  metadata?: any
  created_at: string
  updated_at: string
}

export interface CreateEncryptionAlgorithmRequest {
  name: string
  algorithm_type: string
  key_size?: number
  description?: string
  standard?: string
  metadata?: any
}

// Catalogue Relation
export interface CatalogueRelation {
  id: string
  source_type: string
  source_id: string
  target_type: string
  target_id: string
  relation_type: string
  description?: string
  created_at: string
}

export interface CreateCatalogueRelationRequest {
  source_type: string
  source_id: string
  target_type: string
  target_id: string
  relation_type: string
  description?: string
}

export const catalogueApi = {
  // Endpoints
  listEndpoints: async (endpointType?: string): Promise<Endpoint[]> => {
    const params = endpointType ? { endpoint_type: endpointType } : {}
    const response = await apiClient.get<Endpoint[]>('/catalogue/endpoints', { params })
    return response.data
  },

  getEndpoint: async (id: string): Promise<Endpoint> => {
    const response = await apiClient.get<Endpoint>(`/catalogue/endpoints/${id}`)
    return response.data
  },

  createEndpoint: async (data: CreateEndpointRequest): Promise<Endpoint> => {
    const response = await apiClient.post<Endpoint>('/catalogue/endpoints', data)
    return response.data
  },

  updateEndpoint: async (id: string, data: Partial<CreateEndpointRequest>): Promise<Endpoint> => {
    const response = await apiClient.put<Endpoint>(`/catalogue/endpoints/${id}`, data)
    return response.data
  },

  // License Keys
  listLicenseKeys: async (): Promise<LicenseKey[]> => {
    const response = await apiClient.get<LicenseKey[]>('/catalogue/license-keys')
    return response.data
  },

  createLicenseKey: async (data: CreateLicenseKeyRequest): Promise<LicenseKey> => {
    const response = await apiClient.post<LicenseKey>('/catalogue/license-keys', data)
    return response.data
  },

  uploadLicenseKeyFile: async (id: string, file: File): Promise<void> => {
    const formData = new FormData()
    formData.append('file', file)
    await apiClient.post(`/catalogue/license-keys/${id}/upload`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
  },

  // Software Versions
  listSoftwareVersions: async (): Promise<SoftwareVersion[]> => {
    const response = await apiClient.get<SoftwareVersion[]>('/catalogue/software-versions')
    return response.data
  },

  createSoftwareVersion: async (data: CreateSoftwareVersionRequest): Promise<SoftwareVersion> => {
    const response = await apiClient.post<SoftwareVersion>('/catalogue/software-versions', data)
    return response.data
  },

  // Encryption Algorithms
  listEncryptionAlgorithms: async (): Promise<EncryptionAlgorithm[]> => {
    const response = await apiClient.get<EncryptionAlgorithm[]>('/catalogue/encryption-algorithms')
    return response.data
  },

  createEncryptionAlgorithm: async (data: CreateEncryptionAlgorithmRequest): Promise<EncryptionAlgorithm> => {
    const response = await apiClient.post<EncryptionAlgorithm>('/catalogue/encryption-algorithms', data)
    return response.data
  },

  // Relations
  createRelation: async (data: CreateCatalogueRelationRequest): Promise<CatalogueRelation> => {
    const response = await apiClient.post<CatalogueRelation>('/catalogue/relations', data)
    return response.data
  },
}

