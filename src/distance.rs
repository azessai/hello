pub struct Distance {
    // Ce champ n'est pas public donc impossible d'y accéder directement
    // en-dehors de ce fichier !
    metre: i32,
}

impl Distance {
    pub fn new() -> Distance {
        Distance {
            metre: 0,
        }
    }

    pub fn new_with_value(valeur: i32) -> Distance {
        Distance {
            metre: valeur,
        }
    }
    pub fn get_metre(&self) -> i32 {
        self.metre
    }
    pub fn set_distance(&mut self, nouvelle_distance: i32) {
        self.metre = nouvelle_distance;
    }
}
impl Drop for Distance {
    fn drop(&mut self) {
        println!("La structure Distance a été détruite ! {}", self.metre);
    }
}