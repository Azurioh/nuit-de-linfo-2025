# Nuit de l'Info 2025 - Projet "Les femmes dans le numérique"

Bienvenue sur le dépôt de notre participation à la Nuit de l'Info 2025. Ce projet a été réalisé pour répondre au défi lancé par l'ACDI sur le thème des femmes dans le numérique.

## 🔗 Accès Rapide aux Défis

### 🤖 Défi Chatbot & IA
Nous avons intégré un assistant virtuel intelligent (style "Clippy") capable de répondre aux questions sur le sujet.
*   [**Documentation Technique Chatbot**](./docs/backend/chatbot.md)

### 🌱 Défi RSE (Responsabilité Sociétale des Entreprises)
Nous avons mis un point d'honneur à respecter les normes d'éco-conception et d'accessibilité (tout en jouant avec les limites pour certains défis "Hostile Design").
*   [**Rapport RSE - Frontend**](./docs/rse/rse-frontend.md)
*   [**Rapport RSE - Backend**](./docs/rse/rse-backend.md)

---

## 📂 Architecture du Projet

Ce projet est un monorepo organisé comme suit :

### [Frontend](./apps/frontend)
Une application web statique performante construite avec **Astro**.
*   **Technologies** : Astro, TypeScript, Vanilla CSS.
*   **Points clés** : "Zero JS by default", Assets optimisés (WebP, OGG), Lazy Loading.
*   **Fonctionnalités** :
    *   Interface "Sober Vintage" (Windows 9x).
    *   Inputs "ludiques" (Téléphone rotatif, Roulette, Slider postal).
    *   Chatbot intégré.

### [Backend](./apps/backend)
Une API robuste et rapide développée en **Rust**.
*   **Technologies** : Rust, Axum (ou framework utilisé).
*   **Rôle** : Gestion des requêtes du chatbot, traitement des données.

## 🚀 Installation et Lancement

### Prérequis
*   Node.js (v18+)
*   pnpm
*   Rust (Cargo)

### Lancer le projet

1.  **Installer les dépendances**
    ```bash
    pnpm install
    ```

2.  **Lancer le Frontend**
    ```bash
    cd apps/frontend
    pnpm dev
    ```

3.  **Lancer le Backend**
    ```bash
    cd apps/backend
    cargo run
    ```

---
*Projet réalisé dans le cadre de la Nuit de l'Info 2025.*
