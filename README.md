# Nuit de l'Info 2025 - Projet "Les femmes dans le numérique"

Bienvenue sur le dépôt de notre participation à la Nuit de l'Info 2025. Ce projet a été réalisé pour répondre au défi lancé par l'ACDI sur le thème des femmes dans le numérique.

## 🔗 Accès Rapide aux Défis

### 🤖 Défi Chatbot & IA
Nous avons intégré un assistant virtuel intelligent (style "Clippy") capable de répondre aux questions sur le sujet.
*   [**Documentation Technique Chatbot**](./docs/backend/chatbot.md)

👉 **Accès** : Rendez-vous sur la page **[`/chatbot`](https://nuit-de-linfo-2025-website.s3-website.fr-par.scw.cloud/chatbot)** de l'application.

### 🌱 Défi RSE (Responsabilité Sociétale des Entreprises)
Nous avons mis un point d'honneur à respecter les normes d'éco-conception et d'accessibilité (tout en jouant avec les limites pour certains défis "Hostile Design").
*   [**Rapport RSE - Frontend**](./docs/rse/rse-frontend.md)
*   [**Rapport RSE - Backend**](./docs/rse/rse-backend.md)

### 🎢 Défi `ft_rube_goldberg`
Pour répondre à ce défi, nous avons imaginé un **CAPTCHA Rube Goldberg**.
C'est une machine infernale numérique : pour valider une simple inscription, l'utilisateur doit réussir une succession d'épreuves absurdes et inutilement complexes (Casse-brique, Puzzle Taquin, Memory, tests de réflexes...).

👉 **Accès** : Rendez-vous sur la page **[`/fun`](https://nuit-de-linfo-2025-website.s3-website.fr-par.scw.cloud/fun)** de l'application.

### 👩‍💻 Défi ACDI : "Les femmes dans le numérique"
Ce défi vise à mettre en lumière la contribution essentielle des femmes dans l'histoire de l'informatique et à promouvoir la mixité.
Nous avons réalisé une page dédiée intégrant :
*   🎙️ **Un Podcast** original de 5 minutes (réalisé par l'équipe).
*   📚 Une présentation pédagogique sur des figures emblématiques (Ada Lovelace...).
*   🎨 Une mise en page soignée respectant la charte graphique.

👉 **Accès** : Rendez-vous sur la page **[`/acdi`](https://nuit-de-linfo-2025-website.s3-website.fr-par.scw.cloud/acdi)** de l'application.

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
