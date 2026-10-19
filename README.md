# Rust CLI App

![Rust](https://img.shields.io/badge/Rust-2024_edition-000000?logo=rust&logoColor=white)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Status](https://img.shields.io/badge/status-en_d%C3%A9veloppement-orange)

> Une application en ligne de commande (CLI) développée en Rust, servant de base à un outil configurable, extensible et capable, à terme, d'interagir avec une base de données.

**Dépôt :** [sylvainTIASSOU/rust_cli_app](https://github.com/sylvainTIASSOU/rust_cli_app)  
**Branche principale :** `master`  
**Langue de documentation :** français

> [!IMPORTANT]
> Ce README décrit le code actuellement présent dans le dépôt. Le projet est un **prototype (v0.1.0)** : certaines fonctionnalités sont déclarées ou amorcées, mais non finalisées. Les exemples ne supposent pas qu'une connexion SQL ou qu'une sauvegarde de configuration fonctionne déjà.

## Sommaire

- [Présentation](#présentation)
- [Fonctionnalités et état d'avancement](#fonctionnalités-et-état-davancement)
- [Stack technique](#stack-technique)
- [Prérequis](#prérequis)
- [Installation](#installation)
- [Démarrage rapide](#démarrage-rapide)
- [Référence des commandes](#référence-des-commandes)
- [Configuration de l'application](#configuration-de-lapplication)
- [Architecture du projet](#architecture-du-projet)
- [Fonctionnement interne](#fonctionnement-interne)
- [Développement et qualité](#développement-et-qualité)
- [Compilation et distribution](#compilation-et-distribution)
- [Sécurité et limitations](#sécurité-et-limitations)
- [Dépannage](#dépannage)
- [Feuille de route](#feuille-de-route)
- [Contribuer](#contribuer)
- [Auteur](#auteur)
- [Licence](#licence)

## Présentation

**Rust CLI App** est un projet Rust destiné à illustrer la construction d'une interface en ligne de commande structurée :

- Analyse des arguments et sous-commandes avec **Clap**.
- Chargement d'une configuration applicative avec **Confy**.
- Sérialisation et désérialisation de données avec **Serde**.
- Propagation des erreurs avec **Anyhow**.
- Structure modulaire séparant les commandes de la configuration et des utilitaires.
- Préparation d'une chaîne de connexion à une base de données, sans connexion effective dans les commandes examinées.

Le projet est adapté à l'apprentissage de Rust et à l'évolution progressive vers un outil CLI plus complet.

## Fonctionnalités et état d'avancement

| Élément | État dans le code | Détails |
| --- | --- | --- |
| Interface CLI | ✅ Présent | Trois sous-commandes sont reconnues. |
| Aide et version | ✅ Présent | Générées par les macros de Clap. |
| Chargement de configuration | ✅ Présent | `confy::load("rust-cli", None)` est appelé au démarrage. |
| Chemin de configuration | ✅ Présent | Le chemin est demandé à Confy puis affiché sous forme de résultat de débogage. |
| `init` | 🟡 Partiel | Vérifie la chaîne de connexion, construit une configuration temporaire, mais ne la sauvegarde pas. |
| `update` | 🚧 Ébauche | Affiche actuellement `from the update command`. |
| `configure` | 🚧 Ébauche | Affiche actuellement `from the config command`. |
| Connexion à la base de données | 🚧 Non démontrée | `tiberius` est déclaré comme dépendance ; aucune connexion opérationnelle n'a été identifiée dans les commandes. |
| Tests automatisés | ❓ À vérifier | Aucune suite de tests n'a été constatée dans les fichiers examinés. |

## Stack technique

Le manifeste [`Cargo.toml`](Cargo.toml) déclare une application en **édition Rust 2024**, version **0.1.0**.

| Crate | Version déclarée | Utilité |
| --- | --- | --- |
| `clap` | `4.6.7` | Définition et analyse des options et sous-commandes (`derive`). |
| `anyhow` | `1.0.104` | Gestion et propagation des erreurs. |
| `confy` | `2.0.0` | Gestion des fichiers de configuration. |
| `serde` | `1.0.229` | Sérialisation et désérialisation (`derive`). |
| `serde_json` | `1.0.151` | Traitement JSON potentiel. |
| `log` | `0.4.34` | Façade de journalisation. |
| `env_logger` | `0.11.11` | Journalisation via les variables d'environnement. |
| `smol` | `2.0.2` | Runtime asynchrone. |
| `async-std` | `1.11` | Utilitaires asynchrones. |
| `tiberius` | `0.13.0` | Pilote TDS pour SQL Server ; intégration à réaliser ou à vérifier. |

> La présence d'une dépendance dans `Cargo.toml` ne signifie pas nécessairement qu'elle est déjà utilisée dans un parcours fonctionnel de l'application.

## Prérequis

- **Git** pour récupérer le code source.
- **Rust et Cargo** avec prise en charge de l'édition Rust 2024 (Rust **1.85 ou supérieur**).
- Une connexion Internet lors du premier téléchargement des dépendances.
- Un terminal sous Linux, macOS ou Windows.

Vérifier les outils installés :

```bash
git --version
rustc --version
cargo --version
```

Pour installer Rust si nécessaire, consulter [rustup.rs](https://rustup.rs/).

Sur Linux ou macOS, installation habituelle :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Puis redémarrer le terminal ou charger l'environnement Cargo selon les instructions de `rustup`.

## Installation

### 1. Cloner le dépôt

```bash
git clone https://github.com/sylvainTIASSOU/rust_cli_app.git
cd rust_cli_app
```

### 2. Installer les dépendances et compiler

```bash
cargo build --locked
```

`--locked` permet d'utiliser les versions déjà figées dans `Cargo.lock` sans modifier ce fichier.

### 3. Vérifier l'interface

```bash
cargo run -- --help
cargo run -- --version
```

> Le premier `--` sépare les arguments destinés à Cargo des arguments transmis à l'application.

## Démarrage rapide

Afficher les commandes disponibles :

```bash
cargo run -- --help
```

Afficher l'aide de la commande d'initialisation :

```bash
cargo run -- init --help
```

Lancer `init` avec une chaîne de connexion fictive (sans établir de connexion SQL) :

```bash
cargo run -- init --db-conn-str 'server=tcp:localhost,1433;database=demo;user=demo;password=CHANGE_ME'
```

> [!WARNING]
> Cet exemple **ne valide pas** la connexion, ne crée pas de base de données et ne persiste pas la configuration. Ne renseignez pas de vrai mot de passe sur une machine partagée : les arguments de ligne de commande et l'affichage de débogage peuvent exposer des secrets.

Tester les sous-commandes en cours de construction :

```bash
cargo run -- update
cargo run -- configure
```

## Référence des commandes

Le programme définit les sous-commandes suivantes dans [`src/main.rs`](src/main.rs) :

```text
rust_cli_app
├── init
├── update
└── configure
```

### `init` — Initialiser une configuration (partiel)

Syntaxe :

```bash
cargo run -- init --db-conn-str '<CHAINE_DE_CONNEXION>' [--name <NOM>] [--path <CHEMIN>]
```

Options définies dans [`src/cmd/init.rs`](src/cmd/init.rs) :

| Option | Alias | Obligatoire | Valeur par défaut | Comportement actuel |
| --- | --- | --- | --- | --- |
| `--db-conn-str` | `-s` | Oui, selon Clap | — | Chaîne de connexion récupérée puis injectée dans un nouvel objet de configuration. |
| `--name` | `-n` | Non | `rust-cli` | Argument analysé, mais pas utilisé par le traitement actuel. |
| `--path` | `-p` | Non | `makedeveasy/rust-cli` | Argument analysé, mais pas utilisé par le traitement actuel. |

Exemple :

```bash
cargo run -- init -s 'server=tcp:localhost,1433;database=demo;user=demo;password=CHANGE_ME' -n my-cli -p ./config
```

**Important :** `init` crée un nouvel objet `AppConfig` et appelle son builder. Le résultat n'est actuellement ni affecté à la configuration chargée au démarrage (`cfg`), ni enregistré sur disque. Les options `--name` et `--path` ne changent pas son comportement pour l'instant.

### `update` — Mettre à jour (ébauche)

```bash
cargo run -- update
```

Comportement actuel :

```text
from the update command
```

Aucune opération de mise à jour n'est implémentée dans cette commande à ce stade.

### `configure` — Configurer (ébauche)

```bash
cargo run -- configure
```

Comportement actuel :

```text
from the config command
```

Le programme ne propose pas encore de sous-options pour modifier, consulter ou supprimer des valeurs de configuration.

### Options d'aide

```bash
cargo run -- --help
cargo run -- init --help
cargo run -- update --help
cargo run -- configure --help
cargo run -- --version
```

## Configuration de l'application

La structure principale, [`AppConfig`](src/utils/config.rs), contient :

| Champ Rust | Type | Valeur par défaut |
| --- | --- | --- |
| `app_name` | `String` | Nom défini dans `Cargo.toml` (`rust_cli_app`). |
| `default_path` | `String` | `makedeveasy/rust-cli`. |
| `db_conn_str` | `String` | Chaîne vide. |

Au démarrage, l'application exécute :

```rust
let mut cfg: utils::config::AppConfig = confy::load("rust-cli", None)?;
```

Confy sélectionne le chemin de configuration en fonction du système d'exploitation. Pour connaître le chemin calculé dans votre environnement, lancez une commande : le code affiche actuellement un `Result` correspondant à ce chemin.

Un objet de configuration comparable, en représentation JSON **illustrative uniquement**, serait :

```json
{
  "app_name": "rust_cli_app",
  "default_path": "makedeveasy/rust-cli",
  "db_conn_str": ""
}
```

> Il ne s'agit **pas** d'un exemple de fichier réellement enregistré par l'application, ni d'une garantie sur le format de sérialisation employé par Confy. Le projet n'appelle pas encore `confy::store` dans les commandes examinées.

### Builder de configuration

Le trait `Builder` propose :

- `new()` pour créer un objet `AppConfig` ;
- `set_config_path(&str)` pour modifier `default_path` ;
- `set_db_conn_str(&str)` pour modifier la chaîne de connexion ;
- `build()` pour retourner une copie de la configuration construite.

La chaîne est préparée en mémoire, mais sa persistance reste à développer.

## Architecture du projet

Arborescence des éléments inspectés :

```text
rust_cli_app/
├── Cargo.toml               # Manifeste du projet et dépendances
├── Cargo.lock               # Résolution verrouillée des dépendances
├── .gitignore               # Exclusions Git
├── README                   # Description initiale (une ligne)
├── docs/                    # Dossier de documentation existant
└── src/
    ├── main.rs             # Entrée, définition CLI, dispatch des commandes
    ├── cmd/
    │   ├── mod.rs          # Déclaration des modules de commandes
    │   ├── init.rs         # Arguments et logique d'initialisation
    │   ├── update.rs       # Ébauche de la commande update
    │   └── conf.rs         # Ébauche de la commande configure
    └── utils/
        ├── mod.rs          # Déclaration des utilitaires
        ├── config.rs       # AppConfig et trait Builder
        └── db.rs           # Module de base de données (contenu non confirmé)
```

> L'arborescence ci-dessus recense les fichiers vérifiés ou directement référencés par les modules ; elle n'affirme pas être la liste exhaustive de tout le dépôt.

### Responsabilités

**`src/main.rs`** : charge la configuration, analyse les arguments CLI et délègue au module correspondant.

**`src/cmd/`** : rassemble les sous-commandes ; chacune expose ses arguments et un point d'entrée `main`.

**`src/utils/config.rs`** : définit les données applicatives et le builder permettant de préparer une configuration.

**`src/utils/db.rs`** : module de données déclaré ; les opérations réellement implémentées restent à préciser.

## Fonctionnement interne

Séquence générale :

```text
Exécution de rust_cli_app
        │
        ▼
Chargement de la configuration par Confy
        │
        ▼
Affichage de débogage de la configuration et du chemin
        │
        ▼
Analyse des arguments par Clap
        │
        ├── init ───────► Construit une configuration temporaire
        │
        ├── update ─────► Affiche un message placeholder
        │
        └── configure ──► Affiche un message placeholder
```

Les retours des commandes utilisent `anyhow::Result<()>` et l'opérateur `?` pour faire remonter certaines erreurs à `main`.

## Développement et qualité

### Vérification du code

```bash
cargo check --locked
```

Cette commande analyse le code sans produire un binaire final.

### Formatage

```bash
cargo fmt --all -- --check
```

Pour appliquer automatiquement le formatage :

```bash
cargo fmt --all
```

### Analyse statique

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

> Certaines commandes de vérification peuvent signaler des avertissements sur les éléments encore inutilisés du prototype. `-D warnings` traite ces avertissements comme des erreurs.

### Tests

```bash
cargo test --locked
```

La commande est utile même si les tests n'ont pas encore été ajoutés. Les futurs tests devraient notamment couvrir l'analyse des arguments, le builder, la persistance de configuration et la gestion des erreurs.

### Documentation Rust

```bash
cargo doc --no-deps --open
```

## Compilation et distribution

### Développement

```bash
cargo run -- --help
```

### Version optimisée

```bash
cargo build --release --locked
```

Le binaire produit est généralement situé à :

- Linux/macOS : `target/release/rust_cli_app`
- Windows : `target\release\rust_cli_app.exe`

Exemple sous Linux/macOS :

```bash
./target/release/rust_cli_app --help
```

Le projet ne contient pas, parmi les éléments vérifiés, de procédure de publication automatique ou de paquet de distribution documenté.

## Sécurité et limitations

> [!CAUTION]
> **Risque d'exposition des chaînes de connexion :** au démarrage, l'application imprime `AppConfig` avec `{:?}`. Comme la structure contient `db_conn_str`, une chaîne chargée contenant un mot de passe pourrait être affichée dans les logs ou dans le terminal.

Avant toute utilisation avec des identifiants réels, il est recommandé de :

1. Supprimer ou masquer l'affichage de débogage contenant `db_conn_str`.
2. Éviter la transmission de secrets en clair dans les arguments CLI, visibles dans certains historiques et outils système.
3. Utiliser une source de secrets sécurisée ou des variables d'environnement avec précautions appropriées.
4. Définir clairement les règles de lecture, écriture et permissions des fichiers de configuration.
5. Implémenter et tester les connexions et la gestion des erreurs avant d'annoncer une prise en charge opérationnelle des bases de données.

**Limitations actuellement identifiées :**

- La commande `init` ne sauvegarde pas la configuration construite.
- `update` et `configure` ne réalisent pas encore d'opération métier.
- La connexion SQL n'a pas été vérifiée comme opérationnelle.
- `--name` et `--path` n'influencent pas encore l'initialisation.
- La configuration est affichée avant l'analyse des sous-commandes.

## Dépannage

### `cargo: command not found`

Rust/Cargo n'est pas installé ou le répertoire Cargo ne figure pas dans le `PATH`. Vérifiez l'installation avec `rustup` puis redémarrez le terminal.

### `failed to parse manifest` ou problème d'édition

Vérifiez que la version de Rust accepte l'édition 2024 :

```bash
rustup update stable
rustc --version
```

### Erreurs au téléchargement des crates

Vérifiez la connexion Internet et le registre Cargo, puis relancez :

```bash
cargo build --locked
```

### `unexpected argument` ou commande inconnue

Consultez l'aide :

```bash
cargo run -- --help
cargo run -- init --help
```

Avec `cargo run`, ne pas oublier le séparateur `--`.

### La configuration ne change pas après `init`

Ce comportement est conforme à l'état actuel du code : l'objet construit par `init` n'est pas persisté. Il faut implémenter la mise à jour et la sauvegarde explicite de la configuration.

### `update` ou `configure` n'effectue aucune action

Ces sous-commandes sont présentes, mais leur logique métier n'est pas encore implémentée.

## Feuille de route

Les éléments suivants sont **des propositions d'évolution**, et non des fonctionnalités déjà disponibles :

- [ ] Finaliser `init` et enregistrer la configuration avec Confy.
- [ ] Prendre en compte `--name` et `--path`.
- [ ] Ajouter `configure show`, `configure set` et `configure reset`.
- [ ] Définir précisément les opérations attendues de `update`.
- [ ] Implémenter la connexion à SQL Server avec `tiberius` si c'est bien la cible retenue.
- [ ] Gérer proprement les erreurs réseau et les timeouts.
- [ ] Remplacer les impressions de débogage par une journalisation sécurisée.
- [ ] Ajouter des tests unitaires et des tests d'intégration.
- [ ] Mettre en place une CI GitHub Actions (formatage, Clippy, tests, compilation).
- [ ] Documenter un exemple de configuration sans secrets.
- [ ] Produire des binaires pour Linux, macOS et Windows.

## Contribuer

Les contributions et retours sont bienvenus.

1. Créer un fork du dépôt.
2. Créer une branche dédiée :

   ```bash
   git checkout -b feat/nom-de-la-fonctionnalite
   ```

3. Effectuer les changements et, si possible, ajouter des tests.
4. Exécuter les vérifications :

   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ```

5. Ouvrir une *pull request* en décrivant le problème et la solution.

Pour signaler un problème, utiliser les [issues GitHub](https://github.com/sylvainTIASSOU/rust_cli_app/issues).

## Auteur

**Sylvain Tiassou** — [@sylvainTIASSOU](https://github.com/sylvainTIASSOU)

Dépôt source : [github.com/sylvainTIASSOU/rust_cli_app](https://github.com/sylvainTIASSOU/rust_cli_app)

## Licence

Aucun fichier de licence n'a été identifié dans les fichiers racine affichés lors de l'inspection. En l'absence de licence explicite, il ne faut pas présumer que le code est libre de réutilisation ou de redistribution.

Si vous souhaitez ouvrir le projet à la contribution et à la réutilisation, choisissez une licence adaptée (par exemple MIT ou Apache-2.0), ajoutez un fichier `LICENSE` au dépôt, puis mettez à jour cette section.
