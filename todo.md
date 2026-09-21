- Refaire le readme.md
- resize des terminaux + commande clear + fix affichage tableau
- vérification des adresses MAC

- tester le restart du playground
- tester les commandes en mode étape par étape et en mode classique
- implémenter arping <IP>

- Doit afficher en gris les différentes étapes du ping par exemple :
    - PC1> ping 192.168.0.2
    -   [PC1] PING 192.168.0.2 - check ARP table
    -   [PC1] ARP  - who has 192.168.0.2 ? Tell 192.168.0.1
    -   [PC2] ARP  - 192.168.0.2 is at 26:b3:39:02:63:64
    -   [PC1] PING 192.168.0.1 -> 192.168.0.2 - request sent
    -   [PC2] PING 192.168.0.1 -> 192.168.0.2 - request received
    -   [PC2] PING 192.168.0.1 -> 192.168.0.2 - response sent
    -   [PC2] PING 192.168.0.1 -> 192.168.0.2 - response received

- Rajouter netns (Linux ou Windows/WSL2)
- Permettre a l'utilisateur de coder son propre routeur / table ARP ... et d'implémenter son code dans la machine
- Si via netns (peut être en C, avec template de base)

- Ajout des questions a l'utilisateur connaissances + code => templates + vscode + sqlite