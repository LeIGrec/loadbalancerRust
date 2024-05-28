# Load Balancer en Rust

Ce projet est une implémentation simple d'un load balancer en Rust, utilisant la bibliothèque `tokio` pour la gestion des sockets et des tâches asynchrones. Il répartit les requêtes entrantes de manière alternée (round-robin) entre deux serveurs backend.

## Prérequis

- Rust (version 1.63 ou supérieure)
- Cargo

## Installation

1. Clonez ce dépôt sur votre machine locale :

git clone https://github.com/LeIGrec/load-balancer-rust.git

2. Accédez au répertoire du projet :

cd load-balancer-rust

## Utilisation

1. Compilez le projet avec Cargo :

cargo build

2. Démarrez le load balancer :

cargo run --bin load-balancer

Vous devriez voir un message indiquant que le load balancer a démarré et écoute sur `127.0.0.1:8080`.

3. Dans deux autres terminaux, démarrez les serveurs backend :

Terminal 1 :

cargo run --bin server1

Terminal 2 :

cargo run --bin server2

4. Envoyez des requêtes HTTP au load balancer sur `127.0.0.1:8080`. Vous pouvez utiliser un outil comme `curl` ou un navigateur web. Les réponses devraient alterner entre "Hello from Server 1!" et "Hello from Server 2!".

## Structure du projet

- `src/main.rs` : Code source du load balancer
- `src/bin/server1.rs` : premier serveur backend
- `src/bin/server2.rs` : second serveur backend
- `Cargo.toml` : Fichier de conf Cargo pour la gestion des dépendances

## Licence

Ce projet est sous licence [MIT](LICENSE).
