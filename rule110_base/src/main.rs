//cherche un elt dans un tableau

fn search <const N: usize>(a: [(u8,u8,u8);N], elt:(u8,u8,u8)) -> bool {
for i in 0..N {
    if a[i]==elt {
        return true;
    };
}
false

}
const N:usize = 10;

fn main (){
    let mut jeu_pair= [0;N+1]; //état du jeu après le tour n_iter si n_iter pair
    let mut jeu_impair= [0;N+1]; //état du jeu après le tour n_iter si n_iter impair
    
    //initialisation de la ligne 0
    jeu_pair[ N-1]=1;
    for _ in 0..(N-1){print!(" ")};
    println!("X");
    
    //boucle
    for n_iter in 1.. N { 
    if n_iter%2==0 {
        //bord gauche
        if (jeu_impair[0]==1)|(jeu_impair[1]==1){jeu_pair[0]=1;
            print!("X")}
        else {jeu_pair[0]=0;
            print!(" ")};
        
        //milieu
        for ind in 0..(N-1) { 
            match (jeu_impair[ind-1],jeu_impair[ind],jeu_impair[ind+1]) {
                x if search([(1,1,1),(1,0,0),(0,0,0)],x) => {jeu_pair[ind]=0;
                    print!(" ")},
                _ => {jeu_pair[ind]=1;
                    print!("X")},
            };
            };
        //bord droit
        if jeu_impair[N-1]==0{jeu_pair[N-1]=0;
        println!(" ")}
        else {jeu_pair[N-1]=1;
        println!("X")}

    }
    else{ //bord gauche
        if (jeu_pair[0]==1)|(jeu_pair[1]==1){jeu_impair[0]=1;
            print!("X")}
        else {jeu_impair[0]=0;
            print!(" ")};
        
        //milieu
        for ind in 0..(N-1) { 
            match (jeu_pair[ind-1],jeu_pair[ind],jeu_pair[ind+1]) {
                x if search([(1,1,1),(1,0,0),(0,0,0)],x) => {jeu_impair[ind]=0;
                    print!(" ")},
                _ => {jeu_impair[ind]=1;
                    print!("X")},
            };
            };
        //bord droit
        if jeu_pair[N-1]==0 {jeu_impair[N-1]=0;
        println!(" ")}
        else {jeu_impair[N-1]=1;
        println!("X")}
    };
};
}