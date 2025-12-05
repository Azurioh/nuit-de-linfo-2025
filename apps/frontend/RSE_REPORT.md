# Rapport d'Analyse RSE - Frontend

Ce rapport détaille les points de non-conformité aux normes RSE (Responsabilité Sociétale des Entreprises), incluant l'accessibilité (a11y), le Green IT (éco-conception), la protection des données (RGPD) et la qualité du code.

## 1. Accessibilité (A11y)

### 🔴 Critique
*   **Composants d'entrée "Ludiques" (Hostile Design)** :
    *   `InputFieldRotaryPhone.astro` : Totalement inutilisable au clavier ou via lecteur d'écran. Repose uniquement sur des événements souris/touch complexes.
    *   `InputFieldRoulette.astro` : Bien que les boutons soient accessibles, la sélection aléatoire rend la saisie d'une date spécifique impossible, bloquant l'utilisateur.
    *   `InputFieldPostalSlider.astro` : Techniquement accessible (`<input type="range">`), mais l'expérience utilisateur est dégradée (sélectionner un code postal précis sur 99999 valeurs est extrêmement laborieux).
*   **Langue de la page** :
    *   `Layout.astro` déclare `<html lang="en">` alors que le contenu est majoritairement en français. Cela perturbe la synthèse vocale des lecteurs d'écran.

### 🟠 Important
*   **Titres de page** :
    *   `Layout.astro` utilise un titre par défaut `<title>Astro Basics</title>`. Chaque page devrait avoir un titre unique et descriptif.
*   **Contraste des couleurs** :
    *   Bien que le contraste global soit bon, l'utilisation de couleurs hardcodées dans `Chatbot.astro` (ex: `#C0C0C0`) sans vérification systématique peut poser problème.

## 2. Green IT (Éco-conception)

### 🟠 Important
*   **Fonctionnalité "Grey Filter" (`Layout.astro`)** :
    *   À chaque frappe clavier (`input` event), une vérification est faite et potentiellement une animation lourde est déclenchée (filtre gris + lecture audio + chargement image).
    *   Cela génère un trafic réseau inutile (téléchargement des images/sons) et une consommation CPU/Batterie excessive pour une fonctionnalité "gadget".
*   **Dépendances Client-Side** :
    *   `Chatbot.astro` importe la librairie `marked` (parser Markdown) côté client. C'est une librairie relativement lourde (~30kb) qui pourrait être évitée si le parsing était fait côté serveur ou via une librairie plus légère, ou chargée uniquement au besoin (lazy loading du script).

### 🟢 Points Positifs
*   Utilisation du format d'image **WebP** et audio **OGG** (formats compressés).
*   Utilisation de `loading="lazy"` sur les images.
*   Architecture **Astro** qui génère du HTML statique par défaut, réduisant le JavaScript client.

## 3. Protection des Données (RGPD) & Vie Privée

### 🟠 Important
*   **API Chatbot** :
    *   Les messages des utilisateurs sont envoyés à une API externe (`PUBLIC_CHATBOT_API_URL`). Il manque une mention d'information ou de consentement explicite indiquant que les données sont traitées par une IA.
*   **Logs en Production** :
    *   Présence de `console.log('RSE: ...')` dans `Layout.astro` et `Chatbot.astro`. Les logs de débogage doivent être retirés en production pour éviter de fuiter des informations potentielles et pour la propreté du code.

## 4. Qualité & Maintenabilité

### 🟠 Important
*   **Internationalisation (i18n)** :
    *   Les textes sont écrits "en dur" dans le code (ex: "Bonjour ! Comment puis-je vous aider..."). Cela rend la maintenance et la traduction future difficiles.
*   **Accessibilité du code** :
    *   Certains composants mélangent logique complexe et présentation, rendant la maintenance plus difficile.

## Recommandations Prioritaires

1.  **Corriger la langue** dans `Layout.astro` (`lang="fr"`).
2.  **Rendre les inputs accessibles** : Ajouter des alternatives textuelles simples (input standard) pour les utilisateurs ne pouvant pas utiliser les widgets "ludiques".
3.  **Optimiser le "Grey Filter"** : Réduire la fréquence de déclenchement (debounce) ou permettre à l'utilisateur de le désactiver (ce qui est partiellement fait avec `prefers-reduced-motion`, mais un bouton explicite serait mieux).
4.  **Nettoyer les logs** : Supprimer les `console.log`.
5.  **Informer l'utilisateur** sur l'usage de ses données dans le chatbot.
