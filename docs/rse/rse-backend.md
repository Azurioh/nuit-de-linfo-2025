# Normes et Règles RSE - Backend

Ce document détaille les choix techniques et architecturaux réalisés pour le backend de notre application, dans une démarche de Responsabilité Sociétale des Entreprises (RSE), incluant des aspects écologiques et sociaux.

## 1. Hébergement Responsable : Scaleway

Nous avons fait le choix stratégique d'héberger notre solution chez **Scaleway**.

*   **Souveraineté et Social** : Scaleway est un acteur **français** et européen. Ce choix favorise l'écosystème technologique local et garantit que nos données (et celles de nos utilisateurs) restent sous juridiction européenne (RGPD), assurant une meilleure protection de la vie privée.
*   **Impact Écologique** : Les datacenters de Scaleway, notamment ceux situés en France (région `fr-par` que nous utilisons), bénéficient d'un mix énergétique bas carbone (majoritairement nucléaire et renouvelable). Scaleway est également reconnu pour ses efforts en matière d'efficacité énergétique (PUE faible) et de gestion de l'eau (pas de climatisation chimique pour certains datacenters comme DC5).

## 2. Architecture Serverless et Sobriété Numérique

Notre déploiement utilise l'offre **Serverless Containers** de Scaleway.

*   **Scaling à Zéro** : Contrairement à un serveur classique qui tourne 24h/24 même sans trafic, nos conteneurs sont configurés avec un `min-scale=0`. Cela signifie que lorsqu'aucun utilisateur n'utilise l'application, **aucune ressource de calcul n'est consommée**. C'est l'optimisation énergétique ultime.
*   **Limitation des Ressources** : Nous avons défini des limites strictes de CPU (500m) et de RAM (512Mo) pour éviter le gaspillage de ressources et la surconsommation inutile.

## 3. Choix Technologique : Rust

Le backend est entièrement développé en **Rust**.

*   **Efficacité Énergétique** : Rust est l'un des langages les plus efficaces au monde. Selon une étude de 2017 (et confirmée depuis), il est extrêmement peu énergivore comparé à des langages interprétés comme Python ou JavaScript (Node.js). Un programme Rust peut consommer jusqu'à **50x moins d'énergie** pour la même tâche.
*   **Performance et Durabilité** : La haute performance du code permet de faire tourner l'application sur du matériel plus modeste et prolonge la durée de vie des infrastructures.

## 4. Optimisation des Binaires et Conteneurs

Nous avons apporté un soin particulier à la construction de nos artefacts de déploiement (Docker).

*   **Images Distroless** : Nous utilisons des images de base `gcr.io/distroless/cc-debian12` pour la production. Ces images ne contiennent que le strict nécessaire pour exécuter l'application (pas de shell, pas de gestionnaire de paquets), réduisant drastiquement la taille de l'image (moins de stockage, transfert réseau plus rapide) et la surface d'attaque (sécurité).
*   **Optimisation de la Compilation** : Notre configuration `Cargo.toml` applique des optimisations agressives (`opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`) pour minimiser la taille du binaire final. Un binaire plus petit signifie moins de données à transférer et à charger en mémoire.
*   **Cache de Build** : Notre pipeline CI/CD utilise `cargo-chef` pour mettre en cache les dépendances. Cela évite de recompiler tout le projet à chaque mise à jour, économisant ainsi des minutes précieuses de temps de calcul (CPU) dans le cloud à chaque déploiement.

## 5. Sécurité et Données (Social)

*   **Dépendances Minimales** : L'utilisation de Rust et d'images distroless limite le nombre de dépendances logicielles, réduisant les risques de failles de sécurité (Supply Chain Attacks).
*   **Protection des API** : L'intégration de `actix-governor` permet de limiter le taux de requêtes (Rate Limiting), protégeant l'infrastructure contre les abus et les attaques par déni de service (DDoS), garantissant la disponibilité du service pour tous les utilisateurs.
