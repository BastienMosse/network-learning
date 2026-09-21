# Network Learning Desktop

Application de bureau pour apprendre les réseaux informatiques, construite avec Tauri, React et Rust.

Elle combine un **simulateur réseau interactif** (playground) et un **moteur de cours** basé sur des fichiers YAML.

## Simulateur réseau

Le playground permet de construire et simuler des topologies réseau en temps réel :

- **Appareils** : PC, Hub, Switch, Bridge, Routeur
- **Interfaces** : ajout, configuration IP/masque/MTU, adresses MAC auto-générées
- **Câblage** : liaison point-à-point entre interfaces
- **Simulation** : pile réseau complète en Rust (Ethernet, ARP, IPv4, ICMP)
- **Console** : exécution de commandes sur chaque appareil

### Commandes disponibles

| Appareil | Commandes |
| --- | --- |
| PC | `ping <ip>`, `arping <ip>`, `arp`, `ifconfig`, `help`, `clear` |
| Routeur | `route`, `arp`, `ifconfig`, `help` |
| Hub / Switch / Bridge | `ifconfig`, `help` |

### Mode étape par étape

Activer le mode *step-by-step* pour observer chaque opération réseau :
- Résolution ARP (lookup, request, reply)
- Envoi et réception ICMP
- Apprentissage MAC sur switch/bridge
- Routage et forwarding

Les étapes s'affichent en gris dans la console pendant l'exécution des commandes.

## Moteur de cours

Les cours sont écrits en YAML dans `public/courses/` et découverts automatiquement. Chaque cours peut contenir :

- Texte, code, informations et avertissements
- Diagrammes Mermaid
- Images
- Démonstrations interactives animées (nœuds, paquets, tableaux)

```text
public/courses/
└── reseaux/
    └── adressage-ip/
        ├── course.yaml
        └── images/
```

### Créer un cours

```yaml
id: adressage-ip
title: Comprendre IPv4
description: Découvrir les adresses IPv4.
level: Débutant
duration: 20 min
themeId: reseaux
color: coral
sections:
  - id: intro
    title: Introduction
    kicker: 01 / Bases
    blocks:
      - type: text
        content: Une adresse IP identifie une interface sur un réseau.
```

Les blocs disponibles : `text`, `info`, `warning`, `code`, `mermaid`, `image`, `demo`.

## Prérequis

- Node.js 20+
- npm
- Rust (stable)
- Tauri CLI (`npm run tauri`)

## Installation et lancement

```bash
npm install
npm run tauri:dev
```

Le frontend Vite démarre sur `http://localhost:5173` et l'application Tauri s'ouvre automatiquement.

### Build de production

```bash
npm run tauri:build
```

### Frontend seul (sans simulateur)

```bash
npm run dev
```

Le moteur de cours fonctionne sans Tauri, mais le playground nécessite le backend Rust.

## Architecture

```text
src/                          Frontend React
├── App.tsx                   Point d'entrée
├── pages/
│   ├── Home/                 Page d'accueil
│   ├── Playground/           Simulateur réseau
│   ├── Course/               Lecteur de cours
│   ├── Theme/                Page thématique
│   └── Explorer/             Explorateur
├── netsim/
│   ├── components/           Canvas, Console, DevicePanel, Toolbar
│   └── types.ts              Types réseau (Device, Interface, Link)
└── courses/
    ├── services/             Chargement des cours
    ├── contexts/             Contextes React
    └── types/                Types des cours

src-tauri/                    Backend Rust (Tauri)
├── src/
│   ├── simulation.rs         Gestionnaire de simulation
│   ├── commands.rs           Commandes Tauri
│   ├── events.rs             Bus d'événements et mode step-by-step
│   ├── devices/              PC, Hub, Switch, Bridge, Router
│   ├── net/                  Protocoles (Ethernet, ARP, IPv4, ICMP)
│   └── utils/                Interfaces, tables ARP/MAC/routage, sockets
└── Cargo.toml

public/courses/               Contenu pédagogique (YAML)
scripts/                      Génération du manifeste de cours
```

## Vérification

```bash
npm run lint
npm run build
```
