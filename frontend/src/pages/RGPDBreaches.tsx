import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { entitiesApi } from '../api/entities'
import { rgpdApi, Breach } from '../api/rgpd'
import './RGPDBreaches.css'

export default function RGPDBreaches() {
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [selectedEntity, setSelectedEntity] = useState<string>('')
  const [formData, setFormData] = useState({
    breach_date: '',
    discovery_date: '',
    description: '',
    data_categories_affected: '',
    number_of_subjects: '',
    severity: 'medium',
    containment_measures: '',
  })
  const queryClient = useQueryClient()

  const { data: entities } = useQuery({
    queryKey: ['entities'],
    queryFn: () => entitiesApi.list(),
  })

  const { data: breaches, isLoading } = useQuery({
    queryKey: ['rgpd-breaches', selectedEntity],
    queryFn: () => rgpdApi.listBreaches(selectedEntity || undefined),
  })

  const createMutation = useMutation({
    mutationFn: (data: any) => rgpdApi.createBreach(selectedEntity, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['rgpd-breaches'] })
      setShowCreateForm(false)
      setFormData({
        breach_date: '',
        discovery_date: '',
        description: '',
        data_categories_affected: '',
        number_of_subjects: '',
        severity: 'medium',
        containment_measures: '',
      })
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (!selectedEntity) {
      alert('Veuillez sélectionner une entité')
      return
    }
    createMutation.mutate({
      breach_date: new Date(formData.breach_date).toISOString(),
      discovery_date: new Date(formData.discovery_date).toISOString(),
      description: formData.description,
      data_categories_affected: formData.data_categories_affected
        .split(',')
        .map((s) => s.trim())
        .filter(Boolean),
      number_of_subjects: formData.number_of_subjects
        ? parseInt(formData.number_of_subjects)
        : undefined,
      severity: formData.severity,
      containment_measures: formData.containment_measures || undefined,
    })
  }

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'low':
        return '#4caf50'
      case 'medium':
        return '#ff9800'
      case 'high':
        return '#f44336'
      case 'critical':
        return '#9c27b0'
      default:
        return '#666'
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'detected':
        return '#f44336'
      case 'contained':
        return '#ff9800'
      case 'investigating':
        return '#2196f3'
      case 'resolved':
        return '#4caf50'
      case 'reported':
        return '#9c27b0'
      default:
        return '#666'
    }
  }

  if (isLoading) {
    return <div className="rgpd-breaches-page">Chargement...</div>
  }

  return (
    <div className="rgpd-breaches-page">
      <div className="page-header">
        <h1>Gestion des Écarts RGPD</h1>
        <button onClick={() => setShowCreateForm(!showCreateForm)}>
          {showCreateForm ? 'Annuler' : 'Déclarer un écart'}
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
          <h2>Déclarer un écart de sécurité</h2>
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
            <label>Date de l'écart *</label>
            <input
              type="datetime-local"
              value={formData.breach_date}
              onChange={(e) => setFormData({ ...formData, breach_date: e.target.value })}
              required
            />
          </div>
          <div className="form-group">
            <label>Date de découverte *</label>
            <input
              type="datetime-local"
              value={formData.discovery_date}
              onChange={(e) => setFormData({ ...formData, discovery_date: e.target.value })}
              required
            />
          </div>
          <div className="form-group">
            <label>Description *</label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              required
              rows={4}
            />
          </div>
          <div className="form-group">
            <label>Catégories de données affectées (séparées par des virgules) *</label>
            <input
              type="text"
              value={formData.data_categories_affected}
              onChange={(e) =>
                setFormData({ ...formData, data_categories_affected: e.target.value })
              }
              required
            />
          </div>
          <div className="form-group">
            <label>Nombre de personnes concernées</label>
            <input
              type="number"
              value={formData.number_of_subjects}
              onChange={(e) => setFormData({ ...formData, number_of_subjects: e.target.value })}
              min="0"
            />
          </div>
          <div className="form-group">
            <label>Gravité *</label>
            <select
              value={formData.severity}
              onChange={(e) => setFormData({ ...formData, severity: e.target.value })}
              required
            >
              <option value="low">Faible</option>
              <option value="medium">Moyenne</option>
              <option value="high">Élevée</option>
              <option value="critical">Critique</option>
            </select>
          </div>
          <div className="form-group">
            <label>Mesures de confinement</label>
            <textarea
              value={formData.containment_measures}
              onChange={(e) =>
                setFormData({ ...formData, containment_measures: e.target.value })
              }
              rows={3}
            />
          </div>
          <button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending ? 'Déclaration...' : 'Déclarer'}
          </button>
        </form>
      )}

      <div className="breaches-list">
        {breaches && breaches.length > 0 ? (
          <table>
            <thead>
              <tr>
                <th>Date de découverte</th>
                <th>Description</th>
                <th>Gravité</th>
                <th>Statut</th>
                <th>Personnes concernées</th>
                <th>Autorité notifiée</th>
              </tr>
            </thead>
            <tbody>
              {breaches.map((breach: Breach) => (
                <tr key={breach.id}>
                  <td>{new Date(breach.discovery_date).toLocaleDateString('fr-FR')}</td>
                  <td>{breach.description.substring(0, 100)}...</td>
                  <td>
                    <span
                      className="severity-badge"
                      style={{ backgroundColor: getSeverityColor(breach.severity) }}
                    >
                      {breach.severity}
                    </span>
                  </td>
                  <td>
                    <span
                      className="status-badge"
                      style={{ backgroundColor: getStatusColor(breach.status) }}
                    >
                      {breach.status}
                    </span>
                  </td>
                  <td>{breach.number_of_subjects || '-'}</td>
                  <td>{breach.authority_notified ? 'Oui' : 'Non'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <p>Aucun écart déclaré.</p>
        )}
      </div>
    </div>
  )
}

