import { Outlet, Link, useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import './Layout.css'

export default function Layout() {
  const { user, logout } = useAuth()
  const navigate = useNavigate()

  const handleLogout = () => {
    logout()
    navigate('/login')
  }

  return (
    <div className="layout">
      <nav className="navbar">
        <div className="nav-brand">
          <h1>HyperCyber</h1>
        </div>
        <div className="nav-links">
          <Link to="/">Dashboard</Link>
          <Link to="/entities">Entités</Link>
          <Link to="/catalogue">Catalogue</Link>
          <Link to="/rgpd/register">Registre RGPD</Link>
          <Link to="/rgpd/requests">Demandes d'accès</Link>
          <Link to="/rgpd/breaches">Écarts</Link>
        </div>
        <div className="nav-user">
          <span>{user?.email}</span>
          <button onClick={handleLogout}>Déconnexion</button>
        </div>
      </nav>
      <main className="main-content">
        <Outlet />
      </main>
    </div>
  )
}

