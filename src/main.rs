use std::fmt;
use std::io;
use std::io::Write;
use std::fs::File;

// Enum pour représenter les types de Pokemon
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum PokemonType {
    Feu,
    Eau,
    Plante,
    Electrik,
}

impl fmt::Display for PokemonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = match self {
            PokemonType::Feu => "Feu",
            PokemonType::Eau => "Eau",
            PokemonType::Plante => "Plante",
            PokemonType::Electrik => "Electrik",
        };
        write!(f, "{}", type_str)
    }
}

// Enum pour le genre
#[derive(Debug, PartialEq, Clone)]
enum Genre {
    Male,
    Femelle,
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let genre_str = match self {
            Genre::Male => "Male",
            Genre::Femelle => "Femelle",
        };
        write!(f, "{}", genre_str)
    }
}

// Structure représentant un Pokemon
#[derive(Debug, Clone)]
struct Pokemon {
    nom: String,
    niveau: u32,
    p_type: PokemonType,
    xp: u32,
    genre: Genre,
}

impl Pokemon {
    fn new(nom: &str, niveau: u32, p_type: PokemonType, xp: u32, genre: Genre) -> Pokemon {
        Pokemon {
            nom: nom.to_string(),
            niveau,
            p_type,
            xp,
            genre,
        }
    }

    // Méthode pour gagner de l'XP (et monter de niveau dès 100 XP)
    fn gagner_xp(&mut self, points: u32) {
        self.xp += points;
        while self.xp >= 100 {
            self.xp -= 100;
            self.niveau += 1;
            println!("{} passe au niveau {}", self.nom, self.niveau);
        }
    }

    // Affiche les informations du Pokemon
    fn afficher(&self) {
        println!("Nom   : {}", self.nom);
        println!("Niveau: {}", self.niveau);
        println!("Type  : {}", self.p_type);
        println!("XP    : {}", self.xp);
        println!("Genre : {}", self.genre);
    }

    // Vérifie si deux Pokemon peuvent se reproduire selon les règles :
    // même type, niveau minimum (ici 5), genres opposés
    fn peuvent_se_reproduire(&self, autre: &Pokemon) -> bool {
        let niveau_min = 5;
        self.p_type == autre.p_type &&
        self.niveau >= niveau_min &&
        autre.niveau >= niveau_min &&
        self.genre != autre.genre
    }

    // Tente la reproduction. En cas de succès, retourne un nouveau Pokemon avec:
    // niveau 1, XP 0, même type et nom "Mystere"
    fn reproduction(&self, autre: &Pokemon) -> Option<Pokemon> {
        if self.peuvent_se_reproduire(autre) {
            Some(Pokemon::new("Mystere", 1, self.p_type.clone(), 0, Genre::Male))
        } else {
            None
        }
    }
}

// Structure gérant l'élevage de Pokemon
struct Elevage {
    pokemons: Vec<Pokemon>,
}

impl Elevage {
    fn new() -> Elevage {
        Elevage {
            pokemons: Vec::new(),
        }
    }

    fn ajouter_pokemon(&mut self, pokemon: Pokemon) {
        self.pokemons.push(pokemon);
    }

    fn afficher(&self) {
        if self.pokemons.is_empty() {
            println!("L'élevage est vide.");
        } else {
            for (i, p) in self.pokemons.iter().enumerate() {
                println!("=== Pokemon {} ===", i + 1);
                p.afficher();
                println!("----------------------");
            }
        }
    }

    fn entrainer(&mut self, xp: u32) {
        for p in self.pokemons.iter_mut() {
            p.gagner_xp(xp);
        }
    }

    fn reproduction(&mut self, index1: usize, index2: usize) {
        if index1 < self.pokemons.len() && index2 < self.pokemons.len() {
            let p1 = &self.pokemons[index1];
            let p2 = &self.pokemons[index2];
            if let Some(nouveau) = p1.reproduction(p2) {
                println!("Reproduction réussie ! Nouveau Pokemon généré :");
                nouveau.afficher();
                self.ajouter_pokemon(nouveau);
            } else {
                println!("Reproduction impossible entre {} et {}", p1.nom, p2.nom);
            }
        } else {
            println!("Index invalide.");
        }
    }

    fn trier_par_niveau(&mut self) {
        self.pokemons.sort_by(|a, b| a.niveau.cmp(&b.niveau));
    }

    fn trier_par_type(&mut self) {
        self.pokemons.sort_by(|a, b| a.p_type.cmp(&b.p_type));
    }

    // Sauvegarder les données de l'élevage dans un fichier
    fn sauvegarder(&self, filename: &str) {
        let mut file = File::create(filename).expect("Impossible de créer le fichier");
        for p in &self.pokemons {
            let data = format!(
                "Nom   : {}\nNiveau: {}\nType  : {}\nXP    : {}\nGenre : {}\n\n",
                p.nom, p.niveau, p.p_type, p.xp, p.genre
            );
            file.write_all(data.as_bytes()).expect("Erreur d'écriture");
        }
        println!("Données sauvegardées dans le fichier '{}'", filename);
    }
}

// Fonction d'aide pour lire une entrée utilisateur après avoir affiché un prompt
fn lire_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Erreur de flush");
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

// Fonction pour créer un Pokemon via les entrées de l'utilisateur
fn creer_pokemon() -> Pokemon {
    let nom = lire_input("Entrez le nom du Pokemon : ");

    let niveau_input = lire_input("Entrez le niveau initial : ");
    let niveau: u32 = niveau_input.parse().unwrap_or(1);

    println!("Choisissez le type du Pokemon :");
    println!("1 - Feu");
    println!("2 - Eau");
    println!("3 - Plante");
    println!("4 - Electrik");
    let type_input = lire_input("Votre choix (1-4) : ");
    let p_type = match type_input.as_str() {
        "1" => PokemonType::Feu,
        "2" => PokemonType::Eau,
        "3" => PokemonType::Plante,
        "4" => PokemonType::Electrik,
        _ => {
            println!("Choix invalide, type par défaut : Feu");
            PokemonType::Feu
        }
    };

    let xp_input = lire_input("Entrez l'XP initiale : ");
    let xp: u32 = xp_input.parse().unwrap_or(0);

    println!("Choisissez le genre :");
    println!("1 - Male");
    println!("2 - Femelle");
    let genre_input = lire_input("Votre choix (1-2) : ");
    let genre = match genre_input.as_str() {
        "1" => Genre::Male,
        "2" => Genre::Femelle,
        _ => {
            println!("Choix invalide, genre par défaut : Male");
            Genre::Male
        }
    };

    Pokemon::new(&nom, niveau, p_type, xp, genre)
}

fn main() {
    let mut elevage = Elevage::new();
    let filename = "elevage.txt"; // fichier de sauvegarde

    loop {
        println!("\n--- Menu ---");
        println!("1. Ajouter un Pokemon");
        println!("2. Afficher tous les Pokemon");
        println!("3. Entrainer tous les Pokemon");
        println!("4. Tenter une reproduction entre deux Pokemon");
        println!("5. Trier les Pokemon par niveau");
        println!("6. Trier les Pokemon par type");
        println!("7. Quitter");

        let choix = lire_input("Entrez votre choix : ");

        match choix.as_str() {
            "1" => {
                let pokemon = creer_pokemon();
                elevage.ajouter_pokemon(pokemon);
                println!("Pokemon ajouté avec succès !");
                // Mise à jour automatique de la sauvegarde dans le fichier
                elevage.sauvegarder(filename);
            }
            "2" => {
                println!("Affichage de tous les Pokemon :");
                elevage.afficher();
            }
            "3" => {
                let xp_input = lire_input("Entrez le nombre d'XP à ajouter à chaque Pokemon : ");
                let xp: u32 = xp_input.parse().unwrap_or(0);
                elevage.entrainer(xp);
                println!("Tous les Pokemon ont été entrainés !");
                elevage.sauvegarder(filename);
            }
            "4" => {
                let index1_input = lire_input("Entrez l'index du premier Pokemon (commence à 1) : ");
                let index2_input = lire_input("Entrez l'index du second Pokemon (commence à 1) : ");
                let index1: usize = index1_input.parse().unwrap_or(0);
                let index2: usize = index2_input.parse().unwrap_or(0);
                if index1 == 0 || index2 == 0 {
                    println!("Indices invalides.");
                } else {
                    elevage.reproduction(index1 - 1, index2 - 1);
                    elevage.sauvegarder(filename);
                }
            }
            "5" => {
                elevage.trier_par_niveau();
                println!("Pokemon triés par niveau.");
                elevage.sauvegarder(filename);
            }
            "6" => {
                elevage.trier_par_type();
                println!("Pokemon triés par type.");
                elevage.sauvegarder(filename);
            }
        
            "7" => {
                println!("Au revoir !");
                break;
            }
            _ => {
                println!("Choix invalide, veuillez réessayer.");
            }
        }
    }
}
