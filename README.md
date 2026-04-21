# Shift Private

Shift Private est une visionneuse d’images locale en Rust, pensée comme une alternative moderne à `imv` pour Linux, avec une identité plus soignée, plus Colony, et une base propre pour évoluer.

L’idée n’est pas de faire un ouvre-image générique de plus. Le but est de construire un viewer :

- très rapide au démarrage,
- fluide à l’usage,
- agréable au clavier comme à la souris,
- capable de zoomer fort,
- sobre en mémoire,
- et assez élégant pour donner envie de l’utiliser tous les jours.

## État actuel

Le prototype actuel permet déjà :

- d’ouvrir une image via un sélecteur de fichiers,
- d’afficher l’image choisie,
- d’indexer automatiquement les autres images du même dossier,
- de naviguer entre les images précédentes et suivantes,
- d’avoir une première base desktop en Rust + Iced qui compile proprement.

Ce n’est pas encore la version finale. C’est une fondation propre.

## Vision produit

Shift Private vise une expérience de viewer locale, rapide et premium.

### Priorités du projet

- ouverture instantanée,
- navigation fluide,
- zoom précis et profond,
- fit-to-window propre,
- bon confort clavier,
- UI minimaliste et lisible,
- base technique extensible sans sur-ingénierie.

### Ce que le projet pourra devenir

À terme, Shift Private pourra évoluer vers quelque chose de plus riche :

- miniatures de dossier,
- panneau latéral ou bandeau filmstrip,
- favoris,
- tags,
- collections,
- raccourcis clavier avancés,
- wallpapers,
- support GIF et formats supplémentaires,
- galerie personnelle locale.

## Positionnement

Shift Private se place entre plusieurs mondes :

- la vitesse brute d’un viewer Linux léger,
- le confort d’une application desktop moderne,
- et une direction visuelle plus propre, plus intime, plus Colony.

Ce n’est pas un DAM complexe.
Ce n’est pas un clone lourd d’une galerie photo grand public.
C’est un viewer local rapide, beau et sérieux.

## Stack technique

- **Rust** pour la robustesse, la performance et la maintenabilité
- **Iced** pour l’interface desktop
- **rfd** pour les dialogues natifs
- **image** pour le traitement/décodage d’images au besoin

## Philosophie technique

Le projet suit une ligne simple :

- commencer petit,
- garder une architecture claire,
- éviter la complexité prématurée,
- construire d’abord une excellente expérience de visionnage,
- puis seulement ajouter les couches galerie/organisation.

Autrement dit : un bon cœur avant les gadgets.

## Roadmap courte

### MVP viewer

- [x] Base Rust + Iced
- [x] Ouverture d’image
- [x] Navigation locale dans le dossier
- [ ] Ouverture de dossier réelle
- [ ] Fit-to-window
- [ ] Zoom avant / arrière
- [ ] Pan / déplacement dans l’image
- [ ] Raccourcis clavier
- [ ] Barre d’état plus propre

### Suite naturelle

- [ ] Filmstrip / miniatures
- [ ] Préchargement intelligent
- [ ] Gestion de gros dossiers
- [ ] Favoris
- [ ] Tags / collections locales
- [ ] Thème visuel Colony plus affirmé

## Lancer le projet

```bash
cargo run
```

Pour une build optimisée :

```bash
cargo run --release
```

## Pourquoi “Shift Private” ?

Parce que le projet cherche une sensation précise :

- quelque chose de personnel,
- local,
- discret,
- rapide,
- et un peu premium.

Un outil qui ne donne pas l’impression d’être juste “technique”, mais vraiment agréable à garder près de soi.
