Voici le détail par thème, avec la fiche Équipements réseau ajoutée dans les Bases et son approfondissement après ARP.

## Bases

- **OSI** — fiche : les 7 couches, rôle de chacune, PDU (bit/trame/paquet/segment), encapsulation/désencapsulation. Démo : faire "descendre" une donnée à travers les couches visuellement.
- **TCP/IP** — fiche : les 4 couches, correspondance avec OSI (tableau comparatif).
- **LAN, MAN, WAN** — fiche courte : échelle géographique, exemples concrets, qui gère quoi.
- **Standards ISO et IEEE** — fiche très courte : à quoi ça sert, IEEE 802.x pour Ethernet/Wi-Fi, ISO pour OSI.
- **Équipements réseau (survol)** — fiche courte, quasi glossaire imagé : hub (L1), switch/bridge (L2), routeur (L3), AP (L1/L2 sans-fil), modem (L1). Juste le placement dans les couches et le vocabulaire, pas encore le "comment ça marche" — ça sera repris en profondeur après ARP.

## Ethernet

- **CSMA/CD** — ton cours existant.
- **Trame MAC** — fiche + démo : découpage champ par champ, trame réelle à décortiquer.
- **Adresse MAC** — fiche : format 48 bits, OUI, unicast/broadcast/multicast. Exercice : reconnaître le type d'une adresse donnée.
- **Ethernet devices** — fiche visuelle reprenant les équipements vus dans les Bases, mais recentrée sur le rôle Ethernet (hub vs switch surtout).
- **Domaine de collision** — fiche + exercice : topologie donnée, compter les domaines.
- **Domaine de broadcast** — même format, sur une topologie avec switches/routeurs.
- **Spanning Tree Protocol** — fiche : Root Bridge, BPDU, root/designated/blocked port, élection. Démo : animation d'élection sur une topologie en triangle.
- **Training** — exercice combiné : topologie complexe, identifier domaines de collision/broadcast, construire la table MAC, situer chaque équipement dans le modèle OSI (L1→L4).

## VLAN

- **Découverte du principe** — fiche : le problème (tout le monde dans le même domaine de broadcast), la solution (partition logique). Schéma avant/après.
- **Trame VLAN** — fiche + démo : tag 802.1Q, VLAN ID, native VLAN, access port vs trunk.
- **Training** — exercice : topologie avec plusieurs VLAN et trunks, dire si deux machines communiquent directement et pourquoi.

## ARP

- **Trame ARP** — fiche + démo : utilité, structure requête/réponse, champs (opération, MAC/IP source et cible).
- **Tables ARP** — démo interactive : construction pas à pas d'une table ARP suite à des échanges.
- **Routage** — fiche : rôle d'ARP quand la destination est hors réseau local (résolution du next-hop, pas de la destination finale).

## IPv4

- **IP over Ethernet (adresse IPv4 + Trame IP + Analyse)** — fiche + demo : notation décimale, structure réseau/hôte, champs de l'en-tête (TTL, protocole, checksum...), exercice d'analyse d'une trame capturée.
- **Masque/CIDR + Agrégation + Classes** — plusieurs sous-fiches : classes historiques, masque/CIDR, calcul réseau/broadcast/hôtes, agrégation de routes. Bloc dense, à traiter en petites fiches + série d'exercices de calcul.
- **Fragmentation** — fiche : MTU, pourquoi/comment un paquet se fragmente, flags/offset.
- **ICMP + TCP/UDP** — deux fiches séparées : ICMP (ping, types/codes), TCP/UDP (ports, handshake, connecté vs non connecté).
- **NAT/PAT** — fiche + démo : pourquoi, NAT statique/dynamique/PAT, table de traduction.
- **DHCP/DNS** — deux fiches : DHCP (DORA, bail, options), DNS (résolution nom→IP, hiérarchie, types d'enregistrements).
- **Exercices** — série transversale (calcul de sous-réseaux, lecture de trame, DHCP/DNS).

## IPv6

- **Adresse IPv6** — fiche : 128 bits, notation hexadécimale, compression `::`, préfixe `/64`.
- **Trame + Analyse** — fiche + démo : en-tête IPv6 simplifié vs IPv4, exercice d'analyse.
- **DHCPv6 + ICMPv6** — fiche : différences avec IPv4 (SLAAC vs DHCPv6 stateful, Neighbor Discovery qui remplace ARP).
- **Headers** — fiche : notion d'extension headers chaînés (vs en-tête fixe IPv4).
- **Exercices** — écriture/compression d'adresses, identification de types d'adresses (link-local, ULA, global unicast, multicast).



# TODO:

- Refaire les couleurs + les timers (durée du cours)
- Faire les quizz
- Faire les parties archi et tout