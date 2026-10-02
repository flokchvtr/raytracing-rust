use crate::couleur::Couleur;
use crate::forme::Forme;
use crate::ray::Rayon;

#[derive(Debug)]
pub struct Objet {
    pub forme: Forme,
    pub couleur: Couleur,
    pub reflexion: f64,
}

pub fn plus_proche(objets: &[Objet], r: Rayon) -> Option<(f64, &Objet)> {
    let mut best: Option<(f64, &Objet)> = None;

    for o in objets {
        if let Some(t) = o.forme.intersecte(r) {
            let proche = match best {
                None => true,
                Some((t_min, ..)) => t < t_min,
            };
            if proche {
                best = Some((t, o));
            }
        }
    }

    best
}
