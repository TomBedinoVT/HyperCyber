import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { entitiesApi } from '../api/entities'
import { rgpdApi, AccessRequest } from '../api/rgpd'
import './RGPDRequests.css'

export default function RGPDRequests() {
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [selectedEntity, setSelectedEntity] = useState<string>('')
  const [selectedRequest, setSelectedRequest] = useState<string | null>(null)
  const [responseText, setResponseText] = useState('')
  const [formData, setFormData] = useState({
    requester_name: '',
    requester_email: '',
    request_type: 'access',
    description: '',
  })
  const queryClient = useQueryClient()

  const { data: entities } = useQuery({
    queryKey: ['entities'],
    queryFn: () => entitiesApi.list(),
  })

  const { data: requests, isLoading } = useQuery({
    queryKey: ['rgpd-requests', selectedEntity],
    queryFn: () => rgpdApi.listAccessRequests(selectedEntity || undefined),
  })

  const createMutation = useMutation({
    mutationFn: (data: any) => rgpdApi.createAccessRequest(selectedEntity, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['rgpd-requests'] })
      setShowCreateForm(false)
      setFormData({
        requester_name: '',
        requester_email: '',
        request_type: 'access',
        description: '',
      })
    },
  })

  const respondMutation = useMutation({
    mutationFn: ({ id, status, response }: { id: string; status: string; response?: string }) =>
      rgpdApi.respondToRequest(id, { status, response }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['rgpd-requests'] })
      setSelectedRequest(null)
      setResponseText('')
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (!selectedEntity) {
      alert('Veuillez sélectionner une entité')
      return
    }
    createMutation.mutate({
      ...formData,
      description: formData.description || undefined,
    })
  }

  const handleRespond = (requestId: string, status: string) => {
    respondMutation.mutate({
      id: requestId,
      status,
      response: responseText || undefined,
    })
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending':
        return '#ff9800'
      case 'in_progress':
        return '#2196f3'
      case 'completed':
        return '#4caf50'
      case 'rejected':
        return '#f44336'
      default:
        return '#666'
    }
  }

  if (isLoading) {
    return <div className="rgpd-requests-page">Chargement...</div>
  }

  return (
    <div className="rgpd-requests-page">
      <div className="page-header">
        <h1>Demandes d'Accès RGPD</h1>
        <button onClick={() => setShowCreateForm(!showCreateForm)}>
          {showCreateForm ? 'Annuler' : 'Créer une demande'}
        </button>
      </div>

      <div className="entity-selector">
        <label>
          Filtrer par entité:
          <select
            value={selectedEntity}
            onChange={(e) => setSelectedEntity(e.target.value)}
          >
            <option value="">Toutes les entités</option>
            {entities?.map((entity) => (
              <option key={entity.id} value={entity.id}>
                {entity.name}
              </option>
            ))}
          </select>
        </label>
      </div>

      {showCreateForm && (
        <form className="create-form" onSubmit={handleSubmit}>
          <h2>Créer une demande d'accès</h2>
          <div className="form-group">
            <label>Entité *</label>
            <select
              value={selectedEntity}
              onChange={(e) => setSelectedEntity(e.target.value)}
              required
            >
              <option value="">Sélectionner une entité</option>
              {entities?.map((entity) => (
                <option key={entity.id} value={entity.id}>
                  {entity.name}
                </option>
              ))}
            </select>
          </div>
          <div className="form-group">
            <label>Nom du demandeur *</label>
            <input
              type="text"
              value={formData.requester_name}
              onChange={(e) => setFormData({ ...formData, requester_name: e.target.value })}
              required
            />
          </div>
          <div className="form-group">
            <label>Email du demandeur *</label>
            <input
              type="email"
              value={formData.requester_email}
              onChange={(e) => setFormData({ ...formData, requester_email: e.target.value })}
              required
            />
          </div>
          <div className="form-group">
            <label>Type de demande *</label>
            <select
              value={formData.request_type}
              onChange={(e) => setFormData({ ...formData, request_type: e.target.value })}
              required
            >
              <option value="access">Accès aux données</option>
              <option value="rectification">Rectification</option>
              <option value="erasure">Effacement</option>
              <option value="portability">Portabilité</option>
              <option value="objection">Opposition</option>
            </select>
          </div>
          <div className="form-group">
            <label>Description</label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              rows={3}
            />
          </div>
          <button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending ? 'Création...' : 'Créer'}
          </button>
        </form>
      )}

      <div className="requests-list">
        {requests && requests.length > 0 ? (
          <table>
            <thead>
              <tr>
                <th>Demandeur</th>
                <th>Email</th>
                <th>Type</th>
                <th>Statut</th>
                <th>Date</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {requests.map((request: AccessRequest) => (
                <tr key={request.id}>
                  <td>{request.requester_name}</td>
                  <td>{request.requester_email}</td>
                  <td>{request.request_type}</td>
                  <td>
                    <span
                      className="status-badge"
                      style={{ backgroundColor: getStatusColor(request.status) }}
                    >
                      {request.status}
                    </span>
                  </td>
                  <td>{new Date(request.created_at).toLocaleDateString('fr-FR')}</td>
                  <td>
                    {request.status === 'pending' && (
                      <button
                        onClick={() => setSelectedRequest(request.id)}
                        className="action-button"
                      >
                        Répondre
                      </button>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <p>Aucune demande trouvée.</p>
        )}
      </div>

      {selectedRequest && (
        <div className="modal">
          <div className="modal-content">
            <h2>Répondre à la demande</h2>
            <div className="form-group">
              <label>Réponse</label>
              <textarea
                value={responseText}
                onChange={(e) => setResponseText(e.target.value)}
                rows={5}
                placeholder="Votre réponse à la demande..."
              />
            </div>
            <div className="modal-actions">
              <button
                onClick={() => handleRespond(selectedRequest, 'completed')}
                className="success-button"
              >
                Accepter
              </button>
              <button
                onClick={() => handleRespond(selectedRequest, 'rejected')}
                className="danger-button"
              >
                Rejeter
              </button>
              <button onClick={() => setSelectedRequest(null)}>Annuler</button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

