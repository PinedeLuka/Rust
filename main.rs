use std::io;

    #[derive(Debug, PartialEq, Eq)]
    struct Livre{
    titre : String,
    auteur : String,
    annee : i32,
    disponible : bool,
}

fn add_book(vector : &mut Vec<Livre>){
    
    let mut titre = String::new();
    println!("Titre :");
    io::stdin().read_line(&mut titre).expect("Failed to read input");
    titre= titre.trim().to_string();
    
    
    let mut auteur = String::new();
    println!("Auteur :");
    io::stdin().read_line(&mut auteur).expect("Failed to read input");
    auteur=auteur.trim().to_string();
    
    
    let mut date = String::new();
    println!("Date:");
    io::stdin().read_line(&mut date).expect("Failed to read line");
    let mut num: i32 = date.trim().parse().expect("Invalid input");

    if num<-3300 || num>2025{
        while num<0 || num>2025{
            println!("Date invalide, recommencez:");
            io::stdin().read_line(&mut date).expect("Failed to read line");
            num = date.trim().parse().expect("Invalid input");
        }
    }
    
    let livre = Livre{
        titre : titre,
        auteur : auteur,
        annee : num,
        disponible : true
    };
    if vector.contains(&livre){
        println!("le livre existe déjà");
        return;
    }else{
        vector.push(livre);
    }
}

fn borrow_book ( vector : &mut Vec<Livre>){
    let mut book = String::new(); 
    io::stdin().read_line(&mut book).expect("Failed to read line");
    for i in vector{
        if i.titre == book{
            if i.disponible == true{
                i.disponible = false;
                println!("livre emprunté.");
            }else{
                println!("le livre demandé est déjà emprunté")
            }
        }
    }
}

fn give_back ( vector : &mut Vec<Livre>){
    let mut book = String::new(); 
    io::stdin().read_line(&mut book).expect("Failed to read line");
    for i in vector{
        if i.titre == book{
            if i.disponible == false{
                i.disponible = true;
                println!("livre rendu.");
            }else{
                println!("le livre demandé est déjà rendu")
            }
        }
    }
}

fn print_all_books(vector : &Vec<Livre>){
    if vector.len() == 0{
        println!("il n'y a aucun livre dans la bibliothèque");
    }else{
        for i in vector{
            println!("- {}, par {}, en {}", i.titre, i.auteur, i.annee);
        }
    }
}

fn print_disponible_books(vector : &Vec<Livre>){
    let mut vec2 = Vec::new();
    vec2.extend(vector);
    vec2.retain(|x| x.disponible == true);
    if vec2.len()==0{
        println!("il n'y a aucun livre disponible dans la bibliothèque");
    }else{
        for i in vector{
            println!("- {}, par {}, en {}", i.titre, i.auteur, i.annee);
        }
    }
}

#[rustfmt::skip]
fn main (){
    let mut i = true;
    
    let mut liste: Vec<Livre> = Vec::new();
    let mut choix: &str;
  
    while i == true{
        println!("Bibliothèque \n 1. Ajouter un livre \n 2. Emprunter un livre \n 3. Retourner un livre \n 4. Afficher tout les livres \n 5. Afficher les livres disponibles \n 6. Quitter");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        choix = input.trim();
        println!("Votre choix : {}",choix);
        while !["0", "1", "2", "3", "4", "5", "6"].contains(&choix){
            println!("saisie invalide, recommencez.");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            choix = input.trim();
        }
        match choix {
            "1" => add_book(&mut liste),
            "2" => borrow_book(&mut liste),
            "3" => give_back(&mut liste),
            "4" => print_all_books(&liste),
            "5" => print_disponible_books(&liste),
            "6" => i=false,
            _ => println!("choix invalide"),
        }
    }
}
