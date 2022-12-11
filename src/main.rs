//Autor: Maksymilian
#![allow(non_snake_case)]                                               //Fehler ausblenden: crate `` should have a snake case name

fn main() {
    //--------------------------------------Eingabe-----------------------------------------------------------------------------
    
    let ende: u32 = 187;
    
    //--------------------------------------------------------------------------------------------------------------------------


    //-------------------------------------Primzahlgenerator--------------------------------------------------------------------
     
    let mut primzahlenkette:Vec<u32> = vec![];

    for zahl in 2..1117{                                 // primzahl Nr. 187: 1117
        let mut primzahl: bool = true;                        // Annahme: Jede Zahl ist eine Primzahl
        for divisor in 2..zahl {                         // teile mit Zahlen bis zu der Zahl die untersucht wird
            if zahl % divisor == 0 {                          // wenn die Zahl teilbar ist:
                primzahl = false;                             // keine Primzahl
                break                                         // muss nicht weiter untersucht werden
            } 
        }

        if primzahl ==true {                                   // wenn Primzahl
            primzahlenkette.push(zahl);                        // füge in Primzahlen Zahlenette hinein
        }
        let ende: usize = ende as usize;                       //.len nur mit usize
        if primzahlenkette.len() == ende  {                    // nur bis zur angegebenen Anzahl Primzahlen suchen
            break                                              
        }
    }
    //println!("primzahlenkette = {:?}",primzahlenkette);
    //println!("elemente im ergebniss = {:?}",primzahlenkette.len());

    //-----------------------------------------------------------------------------------------------------------------------------




    //------------------------------Ergebnisgenerator aus Fibonaccizahl/1,5^x------------------------------------------------------
    
    //-------------------------------------Fibonaccizahlenkette------------------------------------------
    
    let mut fibonaccikette: Vec<u128> = vec![0,1];                          // erstelle Vektor mit: erste bedingung f(0) = 0 und zweite bedingung f(1) = 1
    
    for fibzahl in 2..ende{
        let fibzahl_usize: usize = fibzahl as usize;                        // Vektorelemente müssen usize sein
        let fibzahl= fibonaccikette[fibzahl_usize-2]                  // Formel ab x > 2: f(x) = f(x-1) + f(x-2)
                    +fibonaccikette[fibzahl_usize-1];                   
        fibonaccikette.push(fibzahl);                               
    }
    //println!("fibonaccikette = {:?}",fibonaccikette);    

    //----------------------------------------------------------------------------------------------------


    //-------------------------------------Exponentialzahlenkette-----------------------------------------
    
    let mut exponentialkette: Vec<u128> = vec![1;6];                        // um anstieg auszubremsen initialisierung mit 6x 1
        for x in 1..ende-1{                                                 // x = 0 überspringen 
            let x: f64 = x as f64;                                          // konvertierung in f32 da powf kein u
            let x = f64::powf(1.57,x);                                      // 1,57^a um die Zahl klein zu halten
            exponentialkette.push(x as u128);                               // in exponentialkette einfügen
        }

    //println!("exponentialkette = {:?}",exponentialkette);

    //----------------------------------------------------------------------------------------------------


    //-------------------------------------Ergebniszahlenkette-------------------------------------------
    
    let mut ergebniskette:Vec<u32> = vec![];

        for x in 0..ende{
            let x = x as usize;                                        // indexelemente müssen usize sein
            let ergebnis= fibonaccikette[x]/exponentialkette[x];        // 
            let ergebnis= ergebnis as u32;                               // umwandlung zurück in kleineren Typ
            ergebniskette.push(ergebnis);
        }
        //println!("ergebniskette = {:?}",ergebniskette);
        //println!("elemente im ergebniss = {:?}",ergebniskette.len())

    
    //println!("ergebniskette = {:?}",ergebniskette);u




    //----------------------------------------Faltung Primzahlen und Ergebnis aus Fibonaccizahl/1,5^x -------------------------------------------------------
    let ende = ende as usize;                                           // Ab hier werden Variablen(x,r): usize 
    let mut faltungskette:Vec<u32>= vec![0;ende];                              // Initialisiere mit Nullern
    //let ende = ende as u32;
    
    for x in 0..ende{                                                   // https://de.wikipedia.org/wiki/Faltung_(Mathematik)
 
        for r in 0..x{            
            faltungskette[x] += primzahlenkette[r] * ergebniskette[x-r];      
        }
    }
    println!("Die Faltung ergibt: {:?}",faltungskette);                        // Ausgabe der Faltung


}




