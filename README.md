# HyperCyber - SMSI (Système de Management de la Sécurité de l'Information)

HyperCyber est un SaaS de gestion de la sécurité de l'information (SMSI) conforme à la norme ISO 27001, avec des fonctionnalités RGPD intégrées.

## Architecture

- **Backend**: Rust (Actix Web)
- **Frontend**: React (TypeScript) avec Vite
- **Base de données**: PostgreSQL
- **Authentification**: Login/password + OIDC

## Fonctionnalités

### Authentification
- Connexion par email/mot de passe
- Authentification OIDC (OpenID Connect)
- Gestion des tokens JWT avec refresh tokens
- Support multi-entités (un utilisateur peut accéder à plusieurs entités)

### Gestion des Entités
- Création et gestion d'entités (multi-tenant)
- Attribution de rôles aux utilisateurs par entité
- Contrôle d'accès basé sur les entités

### RGPD
- **Registre léger**: Enregistrement des traitements de données personnelles
- **Demandes d'accès**: Gestion des demandes d'accès, rectification, effacement, portabilité et opposition
- **Gestion des écarts**: Déclaration et suivi des violations de données (data breaches)

## Installation

### Prérequis

- Rust (dernière version stable)
- Node.js 18+ et npm
- PostgreSQL 15+
- Docker et Docker Compose (optionnel)

### Configuration

1. Clonez le repository:
```bash
git clone <repository-url>
cd HyperCyber
```

2. Copiez le fichier `.env.example` vers `.env` et configurez les variables:
```bash
cp .env.example .env
```

3. Démarrez PostgreSQL avec Docker Compose:
```bash
docker-compose up -d
```

### Backend

1. Installez les dépendances Rust:
```bash
cd backend
cargo build
```

2. Configurez les variables d'environnement dans `.env` (voir `.env.example`)

3. Lancez le serveur:
```bash
cargo run
```

Le serveur sera accessible sur `http://localhost:8080`

### Frontend

1. Installez les dépendances:
```bash
cd frontend
npm install
```

2. Créez un fichier `.env` avec:
```
VITE_API_URL=http://localhost:8080/api
```

3. Lancez le serveur de développement:
```bash
npm run dev
```

L'application sera accessible sur `http://localhost:5173`

## Configuration OIDC

Pour activer l'authentification OIDC:

1. Configurez les variables d'environnement dans `.env`:
```
OIDC_CLIENT_ID=votre-client-id
OIDC_CLIENT_SECRET=votre-client-secret
OIDC_ISSUER=https://votre-provider-oidc.com
OIDC_REDIRECT_URI=http://localhost:8080/api/auth/oidc/callback
```

2. Le backend découvrira automatiquement les endpoints OIDC via le document de découverte (`.well-known/openid-configuration`)

## Structure du projet

```
HyperCyber/
├── backend/              # Backend Rust
│   ├── src/
│   │   ├── auth/        # Authentification (login, OIDC, JWT)
│   │   ├── entities/    # Gestion des entités
│   │   ├── rgpd/        # Fonctionnalités RGPD
│   │   └── main.rs
│   ├── migrations/      # Migrations SQL
│   └── Cargo.toml
├── frontend/            # Frontend React
│   ├── src/
│   │   ├── api/         # Clients API
│   │   ├── components/  # Composants réutilisables
│   │   ├── hooks/       # Hooks React
│   │   ├── pages/       # Pages de l'application
│   │   └── App.tsx
│   └── package.json
├── docker-compose.yml   # Configuration PostgreSQL
└── README.md
```

## API Endpoints

### Authentification
- `POST /api/auth/login` - Connexion
- `POST /api/auth/register` - Inscription
- `GET /api/auth/oidc/authorize` - Initier la connexion OIDC
- `GET /api/auth/oidc/callback` - Callback OIDC
- `POST /api/auth/refresh` - Rafraîchir le token
- `GET /api/auth/me` - Informations utilisateur actuel

### Entités
- `GET /api/entities` - Liste des entités
- `POST /api/entities` - Créer une entité
- `GET /api/entities/{id}` - Détails d'une entité
- `PUT /api/entities/{id}` - Modifier une entité
- `GET /api/entities/{id}/users` - Utilisateurs d'une entité

### RGPD - Registre
- `GET /api/rgpd/register?entity_id={id}` - Liste des entrées
- `POST /api/rgpd/register?entity_id={id}` - Ajouter une entrée
- `PUT /api/rgpd/register/{id}` - Modifier une entrée

### RGPD - Demandes d'accès
- `GET /api/rgpd/access-requests?entity_id={id}` - Liste des demandes
- `POST /api/rgpd/access-requests?entity_id={id}` - Créer une demande
- `GET /api/rgpd/access-requests/{id}` - Détails d'une demande
- `POST /api/rgpd/access-requests/{id}/respond` - Répondre à une demande

### RGPD - Écarts
- `GET /api/rgpd/breaches?entity_id={id}` - Liste des écarts
- `POST /api/rgpd/breaches?entity_id={id}` - Déclarer un écart
- `GET /api/rgpd/breaches/{id}` - Détails d'un écart
- `PUT /api/rgpd/breaches/{id}` - Modifier un écart

## Développement

### Migrations de base de données

Les migrations sont exécutées automatiquement au démarrage du serveur. Elles se trouvent dans `backend/migrations/`.

### Tests

```bash
# Backend
cd backend
cargo test

# Frontend
cd frontend
npm test
```

## Licence

[À définir]

## Support

Pour toute question ou problème, veuillez ouvrir une issue sur le repository.