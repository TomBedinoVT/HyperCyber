import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { catalogueApi, Endpoint, LicenseKey, SoftwareVersion, EncryptionAlgorithm, CatalogueRelation } from '../api/catalogue'
import './Catalogue.css'

type CatalogueTab = 'endpoints' | 'license-keys' | 'software-versions' | 'encryption-algorithms'

export default function Catalogue() {
  const [activeTab, setActiveTab] = useState<CatalogueTab>('endpoints')
  const queryClient = useQueryClient()

  return (
    <div className="catalogue-page">
      <h1>Catalogue</h1>
      <div className="catalogue-tabs">
        <button
          className={activeTab === 'endpoints' ? 'active' : ''}
          onClick={() => setActiveTab('endpoints')}
        >
          Endpoints
        </button>
        <button
          className={activeTab === 'license-keys' ? 'active' : ''}
          onClick={() => setActiveTab('license-keys')}
        >
          Clés de licences
        </button>
        <button
          className={activeTab === 'software-versions' ? 'active' : ''}
          onClick={() => setActiveTab('software-versions')}
        >
          Versions de logiciels
        </button>
        <button
          className={activeTab === 'encryption-algorithms' ? 'active' : ''}
          onClick={() => setActiveTab('encryption-algorithms')}
        >
          Algorithmes de cryptage
        </button>
      </div>

      <div className="catalogue-content">
        {activeTab === 'endpoints' && <EndpointsTab queryClient={queryClient} />}
        {activeTab === 'license-keys' && <LicenseKeysTab queryClient={queryClient} />}
        {activeTab === 'software-versions' && <SoftwareVersionsTab queryClient={queryClient} />}
        {activeTab === 'encryption-algorithms' && <EncryptionAlgorithmsTab queryClient={queryClient} />}
      </div>
    </div>
  )
}

function EndpointsTab({ queryClient }: { queryClient: any }) {
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [name, setName] = useState('')
  const [endpointType, setEndpointType] = useState('machine')
  const [description, setDescription] = useState('')
  const [address, setAddress] = useState('')

  const { data: endpoints, isLoading } = useQuery({
    queryKey: ['catalogue-endpoints'],
    queryFn: () => catalogueApi.listEndpoints(),
  })

  const createMutation = useMutation({
    mutationFn: (data: any) => catalogueApi.createEndpoint(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['catalogue-endpoints'] })
      setShowCreateForm(false)
      setName('')
      setEndpointType('machine')
      setDescription('')
      setAddress('')
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    createMutation.mutate({
      name,
      endpoint_type: endpointType,
      description: description || undefined,
      address: address || undefined,
    })
  }

  if (isLoading) return <div>Chargement...</div>

  return (
    <div className="catalogue-tab-content">
      <div className="page-header">
        <h2>Endpoints</h2>
        <button onClick={() => setShowCreateForm(!showCreateForm)}>
          {showCreateForm ? 'Annuler' : 'Créer un endpoint'}
        </button>
      </div>

      {showCreateForm && (
        <form className="create-form" onSubmit={handleSubmit}>
          <h3>Créer un endpoint</h3>
          <div className="form-group">
            <label>Nom *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Type *</label>
            <select
              value={endpointType}
              onChange={(e) => setEndpointType(e.target.value)}
              required
            >
              <option value="machine">Machine</option>
              <option value="program">Programme</option>
              <option value="url">URL</option>
              <option value="api">API</option>
            </select>
          </div>
          <div className="form-group">
            <label>Adresse</label>
            <input
              type="text"
              value={address}
              onChange={(e) => setAddress(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label>Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </div>
          <button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending ? 'Création...' : 'Créer'}
          </button>
        </form>
      )}

      <div className="items-list">
        {endpoints?.map((endpoint) => (
          <EndpointCard key={endpoint.id} endpoint={endpoint} queryClient={queryClient} />
        ))}
        {endpoints?.length === 0 && <p>Aucun endpoint trouvé</p>}
      </div>
    </div>
  )
}

function LicenseKeysTab({ queryClient }: { queryClient: any }) {
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [name, setName] = useState('')
  const [licenseType, setLicenseType] = useState('string')
  const [keyValue, setKeyValue] = useState('')
  const [description, setDescription] = useState('')
  const [uploadingFile, setUploadingFile] = useState<string | null>(null)

  const { data: licenseKeys, isLoading } = useQuery({
    queryKey: ['catalogue-license-keys'],
    queryFn: () => catalogueApi.listLicenseKeys(),
  })

  const createMutation = useMutation({
    mutationFn: (data: any) => catalogueApi.createLicenseKey(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['catalogue-license-keys'] })
      setShowCreateForm(false)
      setName('')
      setLicenseType('string')
      setKeyValue('')
      setDescription('')
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    createMutation.mutate({
      name,
      license_type: licenseType,
      key_value: licenseType === 'string' ? keyValue : undefined,
      description: description || undefined,
    })
  }

  if (isLoading) return <div>Chargement...</div>

  return (
    <div className="catalogue-tab-content">
      <div className="page-header">
        <h2>Clés de licences</h2>
        <button onClick={() => setShowCreateForm(!showCreateForm)}>
          {showCreateForm ? 'Annuler' : 'Créer une clé'}
        </button>
      </div>

      {showCreateForm && (
        <form className="create-form" onSubmit={handleSubmit}>
          <h3>Créer une clé de licence</h3>
          <div className="form-group">
            <label>Nom *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Type *</label>
            <select
              value={licenseType}
              onChange={(e) => setLicenseType(e.target.value)}
              required
            >
              <option value="string">String</option>
              <option value="file">Fichier</option>
            </select>
          </div>
          {licenseType === 'string' && (
            <div className="form-group">
              <label>Valeur de la clé *</label>
              <textarea
                value={keyValue}
                onChange={(e) => setKeyValue(e.target.value)}
                required
              />
            </div>
          )}
          <div className="form-group">
            <label>Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </div>
          <button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending ? 'Création...' : 'Créer'}
          </button>
        </form>
      )}

      <div className="items-list">
        {licenseKeys?.map((key) => (
          <div key={key.id} className="item-card">
            <h3>{key.name}</h3>
            <p><strong>Type:</strong> {key.license_type}</p>
            {key.license_type === 'string' && key.key_value && (
              <p><strong>Clé:</strong> {key.key_value.substring(0, 50)}...</p>
            )}
            {key.license_type === 'file' && (
              <div>
                {key.file_name ? (
                  <p><strong>Fichier:</strong> {key.file_name} ({key.file_size ? `${(key.file_size / 1024).toFixed(2)} KB` : 'N/A'})</p>
                ) : (
                  <div>
                    <p>Aucun fichier uploadé</p>
                    <input
                      type="file"
                      id={`file-upload-${key.id}`}
                      style={{ display: 'none' }}
                      onChange={async (e) => {
                        const file = e.target.files?.[0]
                        if (file) {
                          setUploadingFile(key.id)
                          try {
                            await catalogueApi.uploadLicenseKeyFile(key.id, file)
                            queryClient.invalidateQueries({ queryKey: ['catalogue-license-keys'] })
                          } catch (error) {
                            alert('Erreur lors de l\'upload: ' + (error as Error).message)
                          } finally {
                            setUploadingFile(null)
                            e.target.value = ''
                          }
                        }
                      }}
                    />
                    <label htmlFor={`file-upload-${key.id}`} className="upload-button">
                      {uploadingFile === key.id ? 'Upload en cours...' : 'Uploader un fichier'}
                    </label>
                  </div>
                )}
              </div>
            )}
            {key.description && <p>{key.description}</p>}
          </div>
        ))}
        {licenseKeys?.length === 0 && <p>Aucune clé de licence trouvée</p>}
      </div>
    </div>
  )
}

function SoftwareVersionsTab({ queryClient }: { queryClient: any }) {
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [name, setName] = useState('')
  const [version, setVersion] = useState('')
  const [description, setDescription] = useState('')

  const { data: versions, isLoading } = useQuery({
    queryKey: ['catalogue-software-versions'],
    queryFn: () => catalogueApi.listSoftwareVersions(),
  })

  const createMutation = useMutation({
    mutationFn: (data: any) => catalogueApi.createSoftwareVersion(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['catalogue-software-versions'] })
      setShowCreateForm(false)
      setName('')
      setVersion('')
      setDescription('')
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    createMutation.mutate({
      name,
      version,
      description: description || undefined,
    })
  }

  if (isLoading) return <div>Chargement...</div>

  return (
    <div className="catalogue-tab-content">
      <div className="page-header">
        <h2>Versions de logiciels</h2>
        <button onClick={() => setShowCreateForm(!showCreateForm)}>
          {showCreateForm ? 'Annuler' : 'Créer une version'}
        </button>
      </div>

      {showCreateForm && (
        <form className="create-form" onSubmit={handleSubmit}>
          <h3>Créer une version de logiciel</h3>
          <div className="form-group">
            <label>Nom *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Version *</label>
            <input
              type="text"
              value={version}
              onChange={(e) => setVersion(e.target.value)}
              placeholder="ex: 1.0.0"
              required
            />
          </div>
          <div className="form-group">
            <label>Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </div>
          <button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending ? 'Création...' : 'Créer'}
          </button>
        </form>
      )}

      <div className="items-list">
        {versions?.map((v) => (
          <div key={v.id} className="item-card">
            <h3>{v.name}</h3>
            <p><strong>Version:</strong> {v.version}</p>
            {v.description && <p>{v.description}</p>}
          </div>
        ))}
        {versions?.length === 0 && <p>Aucune version trouvée</p>}
      </div>
    </div>
  )
}

function EncryptionAlgorithmsTab({ queryClient }: { queryClient: any }) {
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [name, setName] = useState('')
  const [algorithmType, setAlgorithmType] = useState('symmetric')
  const [keySize, setKeySize] = useState('')
  const [standard, setStandard] = useState('')
  const [description, setDescription] = useState('')

  const { data: algorithms, isLoading } = useQuery({
    queryKey: ['catalogue-encryption-algorithms'],
    queryFn: () => catalogueApi.listEncryptionAlgorithms(),
  })

  const createMutation = useMutation({
    mutationFn: (data: any) => catalogueApi.createEncryptionAlgorithm(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['catalogue-encryption-algorithms'] })
      setShowCreateForm(false)
      setName('')
      setAlgorithmType('symmetric')
      setKeySize('')
      setStandard('')
      setDescription('')
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    createMutation.mutate({
      name,
      algorithm_type: algorithmType,
      key_size: keySize ? parseInt(keySize) : undefined,
      standard: standard || undefined,
      description: description || undefined,
    })
  }

  if (isLoading) return <div>Chargement...</div>

  return (
    <div className="catalogue-tab-content">
      <div className="page-header">
        <h2>Algorithmes de cryptage</h2>
        <button onClick={() => setShowCreateForm(!showCreateForm)}>
          {showCreateForm ? 'Annuler' : 'Créer un algorithme'}
        </button>
      </div>

      {showCreateForm && (
        <form className="create-form" onSubmit={handleSubmit}>
          <h3>Créer un algorithme de cryptage</h3>
          <div className="form-group">
            <label>Nom *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Type *</label>
            <select
              value={algorithmType}
              onChange={(e) => setAlgorithmType(e.target.value)}
              required
            >
              <option value="symmetric">Symétrique</option>
              <option value="asymmetric">Asymétrique</option>
              <option value="hashing">Hachage</option>
            </select>
          </div>
          <div className="form-group">
            <label>Taille de clé (bits)</label>
            <input
              type="number"
              value={keySize}
              onChange={(e) => setKeySize(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label>Standard (ex: AES-256, RSA-2048)</label>
            <input
              type="text"
              value={standard}
              onChange={(e) => setStandard(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label>Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </div>
          <button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending ? 'Création...' : 'Créer'}
          </button>
        </form>
      )}

      <div className="items-list">
        {algorithms?.map((alg) => (
          <div key={alg.id} className="item-card">
            <h3>{alg.name}</h3>
            <p><strong>Type:</strong> {alg.algorithm_type}</p>
            {alg.standard && <p><strong>Standard:</strong> {alg.standard}</p>}
            {alg.key_size && <p><strong>Taille de clé:</strong> {alg.key_size} bits</p>}
            {alg.description && <p>{alg.description}</p>}
          </div>
        ))}
        {algorithms?.length === 0 && <p>Aucun algorithme trouvé</p>}
      </div>
    </div>
  )
}

// Composant pour afficher un endpoint avec ses relations
function EndpointCard({ endpoint, queryClient }: { endpoint: Endpoint; queryClient: any }) {
  const [showRelations, setShowRelations] = useState(false)
  const [showCreateRelation, setShowCreateRelation] = useState(false)
  const [targetType, setTargetType] = useState('license_key')
  const [targetId, setTargetId] = useState('')
  const [relationType, setRelationType] = useState('uses')
  const [description, setDescription] = useState('')

  const { data: relations } = useQuery({
    queryKey: ['catalogue-relations', endpoint.id],
    queryFn: () => catalogueApi.listRelations({ source_id: endpoint.id, source_type: 'endpoint' }),
    enabled: showRelations,
  })

  const { data: licenseKeys } = useQuery({
    queryKey: ['catalogue-license-keys'],
    queryFn: () => catalogueApi.listLicenseKeys(),
    enabled: showCreateRelation && targetType === 'license_key',
  })

  const { data: softwareVersions } = useQuery({
    queryKey: ['catalogue-software-versions'],
    queryFn: () => catalogueApi.listSoftwareVersions(),
    enabled: showCreateRelation && targetType === 'software_version',
  })

  const { data: algorithms } = useQuery({
    queryKey: ['catalogue-encryption-algorithms'],
    queryFn: () => catalogueApi.listEncryptionAlgorithms(),
    enabled: showCreateRelation && targetType === 'encryption_algorithm',
  })

  const createRelationMutation = useMutation({
    mutationFn: (data: any) => catalogueApi.createRelation(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['catalogue-relations', endpoint.id] })
      setShowCreateRelation(false)
      setTargetId('')
      setDescription('')
    },
  })

  const deleteRelationMutation = useMutation({
    mutationFn: (id: string) => catalogueApi.deleteRelation(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['catalogue-relations', endpoint.id] })
    },
  })

  const handleCreateRelation = (e: React.FormEvent) => {
    e.preventDefault()
    if (!targetId) return
    createRelationMutation.mutate({
      source_type: 'endpoint',
      source_id: endpoint.id,
      target_type: targetType,
      target_id: targetId,
      relation_type: relationType,
      description: description || undefined,
    })
  }

  const getTargetOptions = () => {
    switch (targetType) {
      case 'license_key':
        return licenseKeys?.map(k => ({ id: k.id, name: k.name })) || []
      case 'software_version':
        return softwareVersions?.map(v => ({ id: v.id, name: `${v.name} ${v.version}` })) || []
      case 'encryption_algorithm':
        return algorithms?.map(a => ({ id: a.id, name: a.name })) || []
      default:
        return []
    }
  }

  return (
    <div className="item-card">
      <h3>{endpoint.name}</h3>
      <p><strong>Type:</strong> {endpoint.endpoint_type}</p>
      {endpoint.address && <p><strong>Adresse:</strong> {endpoint.address}</p>}
      {endpoint.description && <p>{endpoint.description}</p>}
      
      <div style={{ marginTop: '1rem' }}>
        <button
          onClick={() => setShowRelations(!showRelations)}
          style={{ padding: '0.5rem 1rem', background: '#6c757d', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
        >
          {showRelations ? 'Masquer' : 'Voir'} les relations
        </button>
        <button
          onClick={() => setShowCreateRelation(!showCreateRelation)}
          style={{ padding: '0.5rem 1rem', background: '#28a745', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer', marginLeft: '0.5rem' }}
        >
          {showCreateRelation ? 'Annuler' : 'Créer une relation'}
        </button>
      </div>

      {showCreateRelation && (
        <form onSubmit={handleCreateRelation} style={{ marginTop: '1rem', padding: '1rem', background: '#f9f9f9', borderRadius: '4px' }}>
          <div className="form-group">
            <label>Type de cible *</label>
            <select value={targetType} onChange={(e) => { setTargetType(e.target.value); setTargetId('') }} required>
              <option value="license_key">Clé de licence</option>
              <option value="software_version">Version de logiciel</option>
              <option value="encryption_algorithm">Algorithme de cryptage</option>
            </select>
          </div>
          <div className="form-group">
            <label>Cible *</label>
            <select value={targetId} onChange={(e) => setTargetId(e.target.value)} required>
              <option value="">Sélectionner...</option>
              {getTargetOptions().map(opt => (
                <option key={opt.id} value={opt.id}>{opt.name}</option>
              ))}
            </select>
          </div>
          <div className="form-group">
            <label>Type de relation *</label>
            <select value={relationType} onChange={(e) => setRelationType(e.target.value)} required>
              <option value="uses">Utilise</option>
              <option value="depends_on">Dépend de</option>
              <option value="implements">Implémente</option>
              <option value="contains">Contient</option>
            </select>
          </div>
          <div className="form-group">
            <label>Description</label>
            <textarea value={description} onChange={(e) => setDescription(e.target.value)} />
          </div>
          <button type="submit" disabled={createRelationMutation.isPending}>
            {createRelationMutation.isPending ? 'Création...' : 'Créer'}
          </button>
        </form>
      )}

      {showRelations && (
        <div className="relations-section">
          <h4>Relations</h4>
          {relations && relations.length > 0 ? (
            <div className="relations-list">
              {relations.map((rel) => (
                <div key={rel.id} className="relation-item">
                  <div className="relation-info">
                    <p>
                      <strong>{endpoint.name}</strong> {rel.relation_type} <strong>{rel.target_type}</strong> ({rel.target_id.substring(0, 8)}...)
                    </p>
                    {rel.description && <p style={{ fontSize: '0.9rem', color: '#666' }}>{rel.description}</p>}
                  </div>
                  <button
                    onClick={() => {
                      if (confirm('Supprimer cette relation ?')) {
                        deleteRelationMutation.mutate(rel.id)
                      }
                    }}
                    style={{ padding: '0.25rem 0.5rem', background: '#dc3545', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
                  >
                    Supprimer
                  </button>
                </div>
              ))}
            </div>
          ) : (
            <p>Aucune relation</p>
          )}
        </div>
      )}
    </div>
  )
}

