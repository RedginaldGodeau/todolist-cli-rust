-----

# Liste de tâches en Rust avec SQLite

Une simple application de liste de tâches en ligne de commande, développée avec Rust et SQLite. Cette application vous permet de gérer vos tâches directement depuis le terminal, offrant des fonctionnalités pour ajouter, visualiser, supprimer et modifier le statut de vos to-do.

-----

## Fonctionnalités

* **Ajouter de nouvelles tâches :** Créez facilement de nouvelles tâches avec un nom, une description et une priorité.
* **Voir toutes les tâches :** Affichez une liste complète de toutes vos tâches, incluant leur ID, nom, description, priorité et statut actuel.
* **Supprimer des tâches :** Retirez les tâches terminées ou indésirables de votre liste.
* **Changer le statut des tâches :** Mettez à jour le statut d'une tâche (par exemple, de "en attente" à "terminée").
* **Stockage persistant :** Toutes vos tâches sont sauvegardées dans une base de données SQLite, garantissant que vos données ne sont pas perdues lorsque vous fermez l'application.

-----

## Démarrage

Ces instructions vous aideront à obtenir une copie du projet fonctionnelle sur votre machine locale pour le développement et les tests.

### Prérequis

Vous devez avoir **Rust** et **Cargo** installés. Si ce n'est pas le cas, vous pouvez les installer en suivant les instructions sur le [site officiel de Rust](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

1.  **Cloner le dépôt :**

    ```bash
    git clone https://github.com/RedginaldGodeau/todolist-cli-rust
    cd todolist-cli-rust
    ```

2.  **Compiler l'application :**

    ```bash
    cargo build --release
    ```

    Cette commande compile le projet et crée un exécutable optimisé dans le répertoire `target/release/`.

### Exécuter l'application

Après la compilation, vous pouvez exécuter l'application directement depuis votre terminal :

```bash
./target/release/ToDo
```

-----

## Utilisation

Lorsque vous exécutez l'application, vous verrez une liste de vos tâches actuelles. Vous serez ensuite invité à choisir une action :

* **Add (Ajouter) :** Créez une nouvelle tâche. Il vous sera demandé le nom, la description et la priorité de la tâche.
* **Delete (Supprimer) :** Supprimez une tâche par son ID.
* **Change Status (Changer le statut) :** Mettez à jour le statut d'une tâche par son ID.
* **Quit (Quitter) :** Fermez l'application.

-----

## Structure du projet (basée sur votre `main.rs`)

* `src/main.rs` : Le point d'entrée principal de l'application, gérant la boucle principale, l'interaction utilisateur et la coordination entre les modules.
* `src/form.rs` : Gère probablement les formulaires d'entrée utilisateur et les invites.
* `src/database.rs` : Contient les fonctions pour se connecter à la base de données SQLite et gérer les opérations liées à la base de données.
* `src/task.rs` : Définit la structure `Task` et implémente les fonctions pour interagir avec les tâches dans la base de données (par exemple, créer, lire, mettre à jour, supprimer des tâches).

-----

## Remerciements

* [Langage de programmation Rust](https://www.rust-lang.org/)
* [SQLite](https://www.sqlite.org/index.html)

-----