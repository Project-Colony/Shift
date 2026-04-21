# Shift Private

Shift Private est une visionneuse d'images / galerie locale pensée pour l'écosystème Colony.

## Vision

Pas un simple ouvre-image Linux générique.

L'objectif est de construire une application :
- élégante,
- fluide,
- sobre en RAM,
- agréable à utiliser au clavier comme à la souris,
- et capable d'évoluer plus tard vers une vraie galerie personnelle.

## Direction produit

### V1
- ouvrir une image unique
- ouvrir un dossier d'images
- navigation précédent / suivant
- zoom basique
- fit-to-screen
- aperçu propre avec interface minimaliste
- consommation mémoire contrôlée

### Plus tard
- miniatures
- favoris
- collections
- tags
- wallpapers
- GIF / vidéo courte

## Philosophie technique

- Rust pour la robustesse et la perf
- Iced pour une UI desktop moderne
- une image courante en mémoire, avec cache minimal
- architecture simple, extensible, sans sur-ingénierie au début

## Inspirations

- imv
- les galeries d'images sobres et rapides
- l'esthétique Colony, plus propre, plus premium, plus personnelle

## Statut

Prototype initial en cours.
