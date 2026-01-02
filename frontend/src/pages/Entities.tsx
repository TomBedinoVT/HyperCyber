import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { entitiesApi, Entity, CreateEntityRequest } from '../api/entities'
import './Entities.css'

export default function Entities() {
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [name, setName] = useState('')
  const [description, setDescription] = useState('')
  const queryClient = useQueryClient()

  const { data: entities, isLoading } = useQuery({
    queryKey: ['entities'],
    queryFn: () => entitiesApi.list(),
  })

  const createMutation = useMutation({
    mutationFn: (data: CreateEntityRequest) => entitiesApi.create(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['entities'] })
      setShowCreateForm(false)
      setName('')
      setDescription('')
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    createMutation.mutate({ name, description: description || undefined })
  }

  if (isLoading) {
    return <div className="entities-page">Chargement...</div>
  }

  return (
    <div className="entities-page">
      <div className="page-header">
        <h1>Gestion des Entités</h1>
        <button onClick={() => setShowCreateForm(!showCreateForm)}>
          {showCreateForm ? 'Annuler' : 'Créer une entité'}
        </button>
      </div>

      {showCreateForm && (
        <form className="create-form" onSubmit={handleSubmit}>
          <h2>Créer une nouvelle entité</h2>
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
            <label>Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={3}
            />
          </div>
          <button type="submit" disabled={createMutation.isPending}>
            {createMutation.isPending ? 'Création...' : 'Créer'}
          </button>
        </form>
      )}

      <div className="entities-list">
        {entities && entities.length > 0 ? (
          <table>
            <thead>
              <tr>
                <th>Nom</th>
                <th>Description</th>
                <th>Date de création</th>
              </tr>
            </thead>
            <tbody>
              {entities.map((entity: Entity) => (
                <tr key={entity.id}>
                  <td>{entity.name}</td>
                  <td>{entity.description || '-'}</td>
                  <td>{new Date(entity.created_at).toLocaleDateString('fr-FR')}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <p>Aucune entité trouvée. Créez votre première entité.</p>
        )}
      </div>
    </div>
  )
}

