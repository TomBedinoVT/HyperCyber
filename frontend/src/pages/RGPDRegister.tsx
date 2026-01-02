import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { entitiesApi } from '../api/entities'
import { rgpdApi, RegisterEntry } from '../api/rgpd'
import './RGPDRegister.css'

export default function RGPDRegister() {
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [selectedEntity, setSelectedEntity] = useState<string>('')
  const [formData, setFormData] = useState({
    processing_name: '',
    purpose: '',
    legal_basis: '',
    data_categories: '',
    data_subjects: '',
    recipients: '',
    retention_period: '',
    security_measures: '',
  })
  const queryClient = useQueryClient()

  const { data: entities } = useQuery({
    queryKey: ['entities'],
    queryFn: () => entitiesApi.list(),
  })

  const { data: entries, isLoading } = useQuery({
    queryKey: ['rgpd-register', selectedEntity],
    queryFn: () => rgpdApi.getRegister(selectedEntity || undefined),
  })

  const createMutation = useMutation({
    mutationFn: (data: any) => rgpdApi.addToRegister(selectedEntity, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['rgpd-register'] })
      setShowCreateForm(false)
      setFormData({
        processing_name: '',
        purpose: '',
        legal_basis: '',
        data_categories: '',
        data_subjects: '',
        recipients: '',
        retention_period: '',
        security_measures: '',
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
      ...formData,
      data_categories: formData.data_categories.split(',').map(s => s.trim()).filter(Boolean),
      data_subjects: formData.data_subjects.split(',').map(s => s.trim()).filter(Boolean),
      recipients: formData.recipients.split(',').map(s => s.trim()).filter(Boolean),
      retention_period: formData.retention_period || undefined,
      security_measures: formData.security_measures || undefined,
    })
  }

  if (isLoading) {
    return <div className="rgpd-register-page">Chargement...</div>
  }

  return (
    <div className="rgpd-register-page">
      <div className="page-header">
        <h1>Registre Léger RGPD</h1>
        <button onClick={() => setShowCreateForm(!showCreateForm)}>
          {showCreateForm ? 'Annuler' : 'Ajouter une entrée'}
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
          <h2>Ajouter une entrée au registre</h2>
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
            <label>Nom du traitement *</label>
            <input
              type="text"
              value={formData.processing_name}
              onChange={(e) => setFormData({ ...formData, processing_name: e.target.value })}
              required
            />
          </div>
          <div className="form-group">
            <label>Finalité *</label>
            <textarea
              value={formData.purpose}
              onChange={(e) => setFormData({ ...formData, purpose: e.target.value })}
              required
              rows={3}
            />
          </div>
          <div className="form-group">
            <label>Base légale *</label>
            <input
              type="text"
              value={formData.legal_basis}
              onChange={(e) => setFormData({ ...formData, legal_basis: e.target.value })}
              required
            />
          </div>
          <div className="form-group">
            <label>Catégories de données (séparées par des virgules) *</label>
            <input
              type="text"
              value={formData.data_categories}
              onChange={(e) => setFormData({ ...formData, data_categories: e.target.value })}
              required
              placeholder="ex: nom, email, adresse"
            />
          </div>
          <div className="form-group">
            <label>Personnes concernées (séparées par des virgules) *</label>
            <input
              type="text"
              value={formData.data_subjects}
              onChange={(e) => setFormData({ ...formData, data_subjects: e.target.value })}
              required
              placeholder="ex: clients, employés"
            />
          </div>
          <div className="form-group">
            <label>Destinataires (séparés par des virgules) *</label>
            <input
              type="text"
              value={formData.recipients}
              onChange={(e) => setFormData({ ...formData, recipients: e.target.value })}
              required
              placeholder="ex: service RH, service comptabilité"
            />
          </div>
          <div className="form-group">
            <label>Durée de conservation</label>
            <input
              type="text"
              value={formData.retention_period}
              onChange={(e) => setFormData({ ...formData, retention_period: e.target.value })}
              placeholder="ex: 5 ans"
            />
          </div>
          <div className="form-group">
            <label>Mesures de sécurité</label>
            <textarea
              value={formData.security_measures}
              onChange={(e) => setFormData({ ...formData, security_measures: e.target.value })}
              rows={3}
            />
          </div>
          <button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending ? 'Ajout...' : 'Ajouter'}
          </button>
        </form>
      )}

      <div className="entries-list">
        {entries && entries.length > 0 ? (
          <table>
            <thead>
              <tr>
                <th>Nom du traitement</th>
                <th>Finalité</th>
                <th>Base légale</th>
                <th>Catégories</th>
                <th>Date de création</th>
              </tr>
            </thead>
            <tbody>
              {entries.map((entry: RegisterEntry) => (
                <tr key={entry.id}>
                  <td>{entry.processing_name}</td>
                  <td>{entry.purpose}</td>
                  <td>{entry.legal_basis}</td>
                  <td>{entry.data_categories.join(', ')}</td>
                  <td>{new Date(entry.created_at).toLocaleDateString('fr-FR')}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <p>Aucune entrée dans le registre.</p>
        )}
      </div>
    </div>
  )
}

