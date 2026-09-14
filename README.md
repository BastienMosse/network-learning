# Paquet

Paquet est un mini CMS pédagogique basé sur des fichiers. Il permet de créer des cours interactifs sur les réseaux sans créer une nouvelle page React pour chaque cours.

```text
course.yaml -> générateur de catalogue -> moteur React -> cours interactif
```

## Prérequis

- Node.js 20 ou plus récent recommandé
- npm

```bash
node --version
npm --version
```

## Installation et lancement

Depuis la racine du projet :

```bash
npm install
npm run dev
```

Le serveur est disponible sur `http://localhost:5173`.

Le script `predev` s'exécute automatiquement avant Vite. Il recherche tous les fichiers `course.yaml` dans `public/courses/` et régénère `public/courses/manifest.json`.

Pour arrêter le serveur : `Ctrl+C`.

## Build de production

```bash
npm run build
```

Cette commande régénère le catalogue, vérifie les types TypeScript et construit l'application dans `dist/`.

Pour tester le build localement :

```bash
npm run preview
```

Pour vérifier le code :

```bash
npm run lint
npm run lint && npm run build
```

## Créer un cours

### 1. Créer le dossier

Un cours peut être rangé dans n'importe quel sous-dossier de `public/courses/`. Le dossier doit contenir un fichier nommé exactement `course.yaml`.

```text
public/courses/
└── reseaux/
    └── adressage-ip/
        ├── course.yaml
        ├── images/
        │   └── ipv4.png
        └── diagrams/
            └── sous-reseau.svg
```

### 2. Écrire le fichier YAML

Cours minimal :

```yaml
id: adressage-ip
title: Comprendre IPv4
description: Découvrir les adresses IPv4 et leur structure.
level: Débutant
duration: 20 min
themeId: reseaux
color: coral
objectives:
  - Lire une adresse IPv4
  - Identifier un réseau et un hôte
sections:
  - id: introduction
    title: Une adresse pour chaque interface
    kicker: 01 / Introduction
    blocks:
      - type: text
        title: À quoi sert une adresse IP ?
        content: Une adresse IP identifie une interface sur un réseau.
```

Le cours sera découvert automatiquement au prochain `npm run dev` ou `npm run build`. Il n'est pas nécessaire d'ajouter une route React.

Les blocs texte acceptent aussi les listes imbriquées Markdown :

```yaml
  - type: text
    content: |
      Les étapes de transmission sont :
      1. Écouter le support
         - Vérifier qu'il est libre
         - Attendre s'il est occupé
      2. Transmettre la trame
         - Surveiller une éventuelle collision
         - Réessayer si nécessaire
```
### Champs du cours

| Champ | Obligatoire | Rôle |
| --- | --- | --- |
| `id` | Oui | Identifiant unique, avec des minuscules et des tirets. |
| `title` | Oui | Titre affiché dans le catalogue et le cours. |
| `description` | Oui | Résumé affiché dans le catalogue. |
| `level` | Non | Niveau, par exemple `Débutant`. Défaut : `Tous niveaux`. |
| `duration` | Non | Durée affichée, par exemple `20 min`. |
| `themeId` | Oui | Identifiant du thème auquel rattacher le cours. |
| `color` | Non | Couleur de carte : `coral`, `blue`, `green`, `yellow` ou `teal`. Défaut : `coral`. |
| `objectives` | Non | Liste des objectifs pédagogiques. |
| `sections` | Oui | Liste ordonnée des sections. |

`slug` est facultatif. S'il est absent, la valeur de `id` est utilisée.

## Organiser les sections

Les sections sont affichées dans l'ordre d'apparition dans le YAML :

```yaml
sections:
  - id: bases
    title: Les bases
    kicker: 01 / Fondations
    blocks:
      - type: text
        content: Le premier contenu de la section.

  - id: pratique
    title: Mise en pratique
    kicker: 02 / Exercice
    blocks:
      - type: text
        content: Le second contenu de la section.
```

Chaque section possède `id`, `title`, `kicker` et `blocks`.

## Les blocs disponibles

### Texte

```yaml
- type: text
  title: Qu'est-ce qu'un routeur ?
  content: Un routeur choisit le prochain saut pour atteindre une destination.
```

`title` est facultatif.

### Information et avertissement

```yaml
- type: info
  title: Le bon réflexe
  content: Pour diagnostiquer un problème, commencez par vérifier le lien physique.

- type: warning
  title: Attention au vocabulaire
  content: Une adresse IP et un port n'identifient pas la même chose.
```

### Code

```yaml
- type: code
  language: aide-mémoire
  code: "IP      = où ?\nPort    = quel service ?\nRouteur = quel prochain saut ?"
```

`language` est facultatif. Les retours à la ligne dans `code` peuvent être écrits avec `\n`.

### Graphe Mermaid

Tu peux créer des schémas avec Mermaid directement dans le fichier YAML :

```yaml
- type: mermaid
  title: Le trajet d'un paquet
  width: 70%
  diagram: |
    flowchart LR
      PC[PC] --> SW[Switch]
      SW --> R[Routeur]
      R --> SERV[Serveur]
```

Le champ `diagram` accepte la syntaxe Mermaid, par exemple `flowchart LR`, `graph TD` ou `sequenceDiagram`.
Les champs `width` et `height` sont facultatifs et acceptent une valeur CSS comme `60%`, `520px` ou `32rem`. Par défaut, le diagramme utilise toute la largeur disponible et sa hauteur naturelle, calculée automatiquement par Mermaid.

```yaml
- type: mermaid
  title: Petit schéma réseau
  width: 60%
  height: 220px
  diagram: |
    flowchart LR
      PC --> Switch --> Routeur
```

Exemple de séquence :

```yaml
- type: mermaid
  title: Échange entre deux machines
  diagram: |
    sequenceDiagram
      participant PC
      participant Serveur
      PC->>Serveur: Requête HTTP
      Serveur-->>PC: Réponse
```

Le diagramme est généré automatiquement dans le navigateur. En cas d'erreur de syntaxe, le bloc affiche un message au lieu de faire disparaître le cours.

### Image

Les ressources sont servies depuis `public/`. Le chemin dans le YAML commence donc par `/` :

```yaml
- type: image
  src: /courses/reseaux/adressage-ip/images/ipv4.png
  alt: Exemple d'une adresse IPv4
  caption: Une adresse IPv4 est composée de quatre octets.
```

`alt` est obligatoire et `caption` est facultatif.

## Créer une démonstration interactive

Une démo contient des équipements, des étapes et éventuellement un paquet animé :

```yaml
- type: demo
  title: Un paquet traverse le réseau
  intro: Suivez le paquet saut après saut.
  nodes:
    - id: pc
      label: PC
      kind: device
    - id: routeur
      label: Routeur
      kind: device
    - id: serveur
      label: Serveur
      kind: service
  steps:
    - title: Le PC prépare la requête
      description: L'application crée les données à envoyer.
      active: [pc]
      panel: Donnée applicative

    - title: Le routeur transmet le paquet
      description: Le routeur consulte sa table de routage.
      active: [pc, routeur]
      packets:
        - from: pc
          to: routeur
      panel: Table de routage

    - title: Le serveur reçoit le paquet
      description: Le serveur remet les données à l'application.
      active: [routeur, serveur]
      packets:
        - from: routeur
          to: serveur
      panel: Requête reçue
```

### Nœuds

Chaque nœud possède :

- `id` : identifiant utilisé dans `active`, `packets.from` et `packets.to` ;
- `label` : nom visible ;
- `kind` : `device` ou `service` ;
- `position` : position optionnelle avec `x` et `y` en pourcentage.

Par défaut, les nœuds sont alignés horizontalement. Pour représenter un hub avec plusieurs appareils, utilise des positions et des connexions explicites :

```yaml
nodes:
  - id: pc-a
    label: PC A
    kind: device
    position: { x: 16, y: 22 }
  - id: hub
    label: Hub
    kind: device
    position: { x: 50, y: 50 }
  - id: pc-b
    label: PC B
    kind: device
    position: { x: 84, y: 22 }

connections:
  - from: pc-a
    to: hub
  - from: pc-b
    to: hub
```

Les coordonnées vont de `0` à `100`. Les connexions sont dessinées entre les nœuds indiqués. Les paquets animés utilisent les mêmes positions avec `packets.from` et `packets.to`.

### Étapes

Chaque étape possède :

- `title` : titre de l'étape ;
- `description` : explication affichée dans le panneau ;
- `active` : liste des nœuds mis en évidence ;
- `panel` : état ou information courte affichée ;
- `packets` : liste d'animations entre deux nœuds. Pour un seul paquet, la liste contient un seul élément ;
- `packetGroups` : groupes de paquets joués les uns après les autres, avec simultanéité dans chaque groupe ;
- `sequential` : si `true`, joue les paquets dans l'ordre avec un délai entre chaque trajet. Par défaut, ils partent ensemble ;
- `media` : image affichée dans le panneau de droite ;
- `table` : tableau affiché sous l'explication.

### Ajouter une image dans une étape

Place l'image dans le dossier du cours, puis référence-la depuis l'étape :

```yaml
- title: Le routeur consulte sa table
  description: Le routeur compare la destination avec ses routes connues.
  active: [routeur]
  panel: Décision de routage
  media:
    src: /courses/reseaux/ethernet/diagrams/routing-table.png
    alt: Table de routage d'un routeur
    caption: La route la plus précise est sélectionnée.
```

L'image apparaît dans le panneau à droite de l'animation. Les formats `SVG` et `PNG` sont adaptés aux schémas.

### Ajouter un tableau dans une étape

```yaml
- title: Le switch consulte sa table MAC
  description: Le switch associe une adresse MAC à un port physique.
  active: [switch]
  panel: Table MAC
  table:
    headers: [Adresse MAC, Port, État]
    rows:
      - ["AA:BB:CC:00:01", 1, Apprise]
      - ["AA:BB:CC:00:02", 4, Apprise]
```

Le tableau est généré automatiquement. Chaque ligne de `rows` doit contenir le même nombre de cellules que `headers`.

Une étape peut combiner `description`, `media` et `table` : le texte est affiché en premier, puis l'image et le tableau dans le même panneau.

Pour envoyer plusieurs paquets dans la même étape, utilise `packets` :

```yaml
- title: Le hub diffuse la trame
  description: Le hub répète le signal vers tous ses ports.
  active: [hub, pc-a, pc-b, pc-c, pc-d]
  panel: Diffusion vers tous les ports
  packets:
    - from: hub
      to: pc-a
    - from: hub
      to: pc-b
    - from: hub
      to: pc-c
    - from: hub
      to: pc-d
```

Pour jouer les paquets l'un après l'autre, active `sequential` uniquement sur l'étape concernée :

```yaml
sequential: true
packets:
  - from: pc-a
    to: hub-1
  - from: hub-1
    to: bridge
```

Sans `sequential: true`, les paquets de la même étape partent ensemble.

Pour un fonctionnement semi-séquentiel, utilise `packetGroups` :

```yaml
packetGroups:
  -
    - from: bridge
      to: hub
  -
    - from: hub
      to: pc-c
    - from: hub
      to: pc-d
```

Le résultat est `Bridge → Hub`, puis `Hub → PC C` et `Hub → PC D` simultanément.

L'interface fournit automatiquement précédent/suivant, accès direct aux étapes, lecture, pause et réinitialisation.

## Ressources d'un cours

```text
public/courses/reseaux/tcp-ip/
├── course.yaml
├── images/
│   └── encapsulation.png
├── diagrams/
│   └── architecture.svg
└── videos/
    └── tcp-handshake.mp4
```

Exemple de référence :

```yaml
src: /courses/reseaux/tcp-ip/images/encapsulation.png
```

Le bloc vidéo n'est pas encore rendu par l'interface actuelle. Pour ajouter un nouveau type de bloc, il faut modifier `src/types.ts` et `BlockRenderer` dans `src/App.tsx`.

## Ajouter un thème

Les thèmes sont décrits dans `public/courses/manifest.json`, dans le tableau `themes` :

```json
{
  "id": "securite",
  "title": "Sécurité réseau",
  "eyebrow": "Nouveau parcours",
  "description": "Comprendre les menaces et les protections réseau.",
  "accent": "blue",
  "courseIds": []
}
```

Utiliser ensuite le même identifiant dans les cours :

```yaml
themeId: securite
```

Le générateur actualise automatiquement `courses` et `courseFiles` dans le manifeste. Il ne remplace pas la définition visuelle des thèmes.

## Règles YAML importantes

- Respecter l'indentation de deux espaces.
- Utiliser des identifiants uniques pour les cours, sections et nœuds.
- Mettre entre guillemets les textes contenant un `:` :

```yaml
content: "Une adresse IP répond à la question : où envoyer le paquet ?"
```

- Éviter les tabulations.
- Vérifier que `active`, `packet.from` et `packet.to` correspondent à des `nodes.id` existants.
- Après une modification, relancer `npm run build` pour détecter les erreurs YAML.

## Architecture

```text
src/
├── App.tsx                         Interface et moteur de rendu
├── App.css                         Styles de l'application
├── index.css                       Styles globaux
└── types.ts                        Modèle TypeScript des cours et blocs

public/courses/
├── manifest.json                   Catalogue des thèmes et cours
└── <theme>/<cours>/course.yaml     Contenu pédagogique

scripts/
└── generate-course-manifest.mjs    Découverte automatique des course.yaml
```

## Ajouter un nouveau type de bloc

1. Ajouter un nouveau type dans `src/types.ts`.
2. Ajouter ce type à `ContentBlock`.
3. Ajouter son rendu dans `BlockRenderer` dans `src/App.tsx`.
4. Ajouter un exemple dans un `course.yaml`.
5. Lancer `npm run lint && npm run build`.

Les cours existants continueront à fonctionner avec le nouveau moteur de rendu.
