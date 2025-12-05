# Documentation RSE - Frontend

Cette documentation détaille les choix techniques et les pratiques mises en place sur le frontend de l'application **Nuit de l'Info 2025** pour respecter les principes de Responsabilité Sociétale des Entreprises (RSE), et plus particulièrement l'Éco-conception (Green IT).

> [!NOTE]
> **Contexte des Défis**
> Certains éléments de l'interface (notamment les champs de saisie "ludiques" et le chatbot) ont été conçus spécifiquement pour répondre à des défis "Hostile Design" et "IA". Ils dérogent volontairement aux règles d'accessibilité et de sobriété présentées ci-dessous.

## 1. Architecture Éco-conçue avec Astro

Nous avons choisi le framework **Astro** pour sa philosophie "Zero JS by default".

### Pourquoi ce choix ?
*   **Réduction de l'empreinte carbone** : Contrairement aux Single Page Applications (SPA) classiques (React, Vue) qui envoient un gros bundle JavaScript au client, Astro génère du **HTML statique** au build.
*   **Moins de ressources client** : Le navigateur de l'utilisateur a beaucoup moins de calculs à effectuer (parsing, compilation, exécution JS), ce qui économise la batterie des terminaux mobiles et prolonge leur durée de vie.
*   **Compatibilité** : Le site fonctionne sur des appareils anciens ou avec une connexion lente, réduisant l'obsolescence programmée logicielle.

## 2. Optimisation des Assets (Poids & Bande Passante)

La réduction du poids des pages est un levier majeur du Green IT.

### Images
*   **Format WebP** : Toutes les images sont servies au format WebP, qui offre une compression supérieure de 30% en moyenne par rapport au JPEG/PNG à qualité égale.
*   **Lazy Loading** : L'attribut `loading="lazy"` est utilisé nativement sur les images (via le composant `<Image />` d'Astro ou les balises `<img>`). Les images ne sont chargées que lorsqu'elles entrent dans le viewport, économisant la bande passante inutile.

### Audio
*   **Format OGG** : Les fichiers audio (utilisés pour l'ambiance sonore) sont au format OGG Vorbis, optimisé pour le web et libre de droits.

## 3. Sobriété du Code

### JavaScript à la demande
Le JavaScript n'est envoyé au client que pour les composants qui nécessitent une interactivité stricte (Hydratation partielle).
*   *Exemple* : Le script du filtre gris (`Layout.astro`) ou du Chatbot n'est exécuté que si nécessaire.
*   **Lazy Loading des Librairies** : Pour le chatbot, la librairie `marked` (parsing Markdown) est chargée dynamiquement (`import()`) uniquement lorsque le bot doit afficher un message, évitant de charger 30kb de code inutile au chargement initial de la page.

### CSS Pur
*   Utilisation de **CSS natif** (Variables CSS) plutôt que de gros frameworks CSS (Bootstrap, Tailwind) qui alourdissent souvent le CSS final avec des classes inutilisées.
*   Cela garantit une feuille de style ultra-légère et maintenable.

## 4. Hébergement et Déploiement
*   Le site étant statique, il peut être hébergé sur n'importe quel serveur web basique (CDN), ne nécessitant pas de serveur d'application complexe (Node.js) tournant en permanence, réduisant ainsi la consommation énergétique côté serveur.

---

**En résumé**, notre approche frontend vise à minimiser l'impact environnemental numérique en :
1.  Envoyant le strict minimum de données (HTML statique, assets compressés).
2.  Demandant le minimum d'effort au terminal de l'utilisateur (peu de JS).
3.  Utilisant des technologies standards et pérennes.
