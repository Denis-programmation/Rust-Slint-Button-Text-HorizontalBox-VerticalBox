// basé sur une vidéo de Syed Hussim : https://www.youtube.com/watch?v=pyX4o77OYaQ&t=428s
// simplifié avec l'opérateur <=>

slint::slint!
{
    #[edition = "1.19"]
    
    import { Button, VerticalBox, HorizontalBox } from "std-widgets.slint";
        
    export component Principale inherits Window
    {
        width: 400px;
        height: 400px;
        
        in property <string> compteur_string: "0";
        
        callback incrementer();
        callback decrementer();
        
        VerticalBox
        {
            padding-bottom:100px;
            
            Text                            // ce widget est "builtin"
            {
                text <=> compteur_string;
                vertical-stretch: 1;
                vertical-alignment: center;
                horizontal-alignment: center;
                font-size: 40px;
                font-weight: 700;
            }
            
            HorizontalBox
            {
                height: 50px;
                padding: 0px;
                
                Button
                {
                    text: "Incrémenter";
                    clicked => 
                    {
                        incrementer();
                    }
                }
                
                Button
                {
                    text: "Décrémenter";
                    clicked => 
                    {
                        decrementer();
                    }
                }
                
            }
            
        }
    }
}

fn main()
{
    let compteur = std::rc::Rc::new(std::cell::Cell::new(0));
    let compteur_clone1 = compteur.clone();
    let compteur_clone2 = compteur.clone();
   
    let fenetre = Principale::new().unwrap();
    let fenetre_clone1 = fenetre.clone_strong();
    let fenetre_clone2 = fenetre.clone_strong();
    
    fenetre.on_incrementer ( move ||
    {
        let c = compteur_clone1.get() + 1;
        compteur_clone1.set ( c );
        let s = format!( "{c}" );
        fenetre_clone1.set_compteur_string ( s.into() );
    });
    
    fenetre.on_decrementer ( move ||
    {
        let c = compteur_clone2.get() - 1;
        compteur_clone2.set ( c );
        let s = format!( "{c}" );
        fenetre_clone2.set_compteur_string ( s.into() );
    });

    fenetre.run().unwrap();
}
