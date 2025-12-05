# Architecture du Chatbot (Backend)

Ce document décrit le fonctionnement interne du chatbot implémenté dans le backend Rust. Il détaille le flux de traitement d'un message, de la réception à la réponse, en passant par les mécanismes de cache et d'interaction avec l'IA.

## Vue d'ensemble

Le chatbot est conçu pour être **rapide**, **économe en ressources** (RSE) et **amusant**. Il repose sur une architecture hybride mêlant cache local, cache distribué et génération par IA (Mistral).

### Flux de Traitement

1.  **Réception de la requête** : Le serveur reçoit une requête POST sur `/chat` avec le prompt de l'utilisateur et un ID de conversation optionnel.
2.  **Validation** : La taille du prompt est vérifiée (< 4096 caractères) pour éviter les abus.
3.  **Vérification du Cache (Stratégie en 3 étapes)** :
    *   **Niveau 1 : Cache Mémoire (LRU)** : Vérification immédiate en RAM pour une correspondance exacte. C'est le plus rapide.
    *   **Niveau 2 : Cache Redis** : Si absent de la RAM, vérification dans Redis (persistance temporaire partagée).
    *   **Niveau 3 : Fuzzy Matching** : Si aucune correspondance exacte, le système cherche une correspondance "proche" (Levenshtein distance > 0.6) dans le cache mémoire. Cela permet de répondre à des questions similaires sans recalculer.
4.  **Logique "Thème Aléatoire" (Spécificité du projet)** :
    *   Si aucune réponse n'est en cache, le backend génère localement un **thème absurde** (ex: "Le pigeon dépressif").
    *   **Subtilité** : Le prompt utilisateur est sauvegardé dans l'historique pour l'affichage, mais c'est ce *thème aléatoire* qui est envoyé à l'IA pour générer la réponse. Cela garantit des réponses surprenantes et décalées, tout en économisant des tokens de contexte complexes.
5.  **Génération IA (Mistral)** :
    *   Appel à l'API Mistral (`mistral-tiny` ou configuré) en mode **streaming**.
    *   Le flux est renvoyé directement au client pour une sensation de réactivité immédiate.
6.  **Sauvegarde** :
    *   Une fois la génération terminée, la réponse complète est stockée dans le cache LRU et dans Redis (TTL 1 heure) pour les futures requêtes identiques.

## Composants Clés

### 1. Gestionnaire d'État (`AppState`)
L'état de l'application est partagé et thread-safe (`Arc<Mutex<AppStateInternal>>`). Il contient :
*   `sessions`: HashMap stockant l'historique des conversations en mémoire.
*   `cache`: Cache LRU (Least Recently Used) pour les réponses fréquentes.
*   `redis`: Client Redis pour le cache distribué.

### 2. Nettoyage Automatique
Une tâche de fond (background task) s'exécute toutes les minutes pour supprimer les sessions inactives depuis plus d'une heure, libérant ainsi la mémoire du serveur.

### 3. Client Mistral
Un client HTTP asynchrone (`reqwest`) configuré pour communiquer avec l'API de Mistral AI. Il gère l'authentification via Bearer Token et le parsing du flux de données (Server-Sent Events).

## Optimisations RSE & Performance

*   **Fuzzy Matching** : Évite de solliciter l'IA pour des questions très proches (ex: "Bonjour" vs "Bonjour !").
*   **Génération Locale de Thème** : Réduit la complexité du prompt envoyé à l'IA.
*   **Streaming** : Améliore l'expérience utilisateur (Time To First Byte) sans attendre la génération complète.
*   **Rust** : L'utilisation de Rust garantit une empreinte mémoire minimale et une exécution très rapide du code "glue" (logique métier).
