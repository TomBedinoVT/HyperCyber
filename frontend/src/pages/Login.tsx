import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import './Login.css'

export default function Login() {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [isRegister, setIsRegister] = useState(false)
  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)
  const { login, register } = useAuth()
  const navigate = useNavigate()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')
    setLoading(true)

    try {
      if (isRegister) {
        await register(email, password, firstName || undefined, lastName || undefined)
      } else {
        await login(email, password)
      }
      navigate('/')
    } catch (err: any) {
      setError(err.response?.data?.error || 'Une erreur est survenue')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="login-container">
      <div className="login-card">
        <h1>HyperCyber</h1>
        <h2>{isRegister ? 'Créer un compte' : 'Connexion'}</h2>
        {error && <div className="error-message">{error}</div>}
        <form onSubmit={handleSubmit}>
          {isRegister && (
            <>
              <div className="form-group">
                <label>Prénom</label>
                <input
                  type="text"
                  value={firstName}
                  onChange={(e) => setFirstName(e.target.value)}
                />
              </div>
              <div className="form-group">
                <label>Nom</label>
                <input
                  type="text"
                  value={lastName}
                  onChange={(e) => setLastName(e.target.value)}
                />
              </div>
            </>
          )}
          <div className="form-group">
            <label>Email</label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>
          <div className="form-group">
            <label>Mot de passe</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
          </div>
          <button type="submit" disabled={loading}>
            {loading ? 'Chargement...' : isRegister ? 'Créer un compte' : 'Se connecter'}
          </button>
        </form>
        {!isRegister && (
          <div style={{ marginTop: '1rem', textAlign: 'center' }}>
            <button
              type="button"
              onClick={() => {
                window.location.href = `${import.meta.env.VITE_API_URL || 'http://localhost:8080/api'}/auth/oidc/authorize`
              }}
              style={{
                padding: '0.5rem 1rem',
                backgroundColor: '#4285f4',
                color: 'white',
                border: 'none',
                borderRadius: '4px',
                cursor: 'pointer',
              }}
            >
              Se connecter avec OIDC
            </button>
          </div>
        )}
        <p>
          {isRegister ? (
            <>
              Déjà un compte ?{' '}
              <button className="link-button" onClick={() => setIsRegister(false)}>
                Se connecter
              </button>
            </>
          ) : (
            <>
              Pas de compte ?{' '}
              <button className="link-button" onClick={() => setIsRegister(true)}>
                Créer un compte
              </button>
            </>
          )}
        </p>
      </div>
    </div>
  )
}

