use crate::couleur::{BLEU, Couleur, ROUGE, VERT};
use crate::forme::Forme;
use crate::lumiere::Lumiere;
use crate::objet::Objet;
use crate::vec3::Vec3;

pub struct Scene {
    pub objets: Vec<Objet>,
    pub lumiere: Lumiere,
}

impl Scene {
    pub fn demo() -> Self {
        let lum = Lumiere {
            position: Vec3::new(-3.0, 4.0, 0.0),
        };

        let sphere_rouge = Objet {
            forme: Forme::Sphere {
                centre: Vec3::new(-1.2, 0.0, -3.0),
                rayon: 0.5,
            },
            couleur: ROUGE,
            reflexion: 0.0,
        };

        let sphere_verte = Objet {
            forme: Forme::Sphere {
                centre: Vec3::new(0.0, 0.0, -3.0),
                rayon: 0.5,
            },
            couleur: VERT,
            reflexion: 0.2,
        };

        let sphere_bleu = Objet {
            forme: Forme::Sphere {
                centre: Vec3::new(1.2, 0.0, -3.0),
                rayon: 0.5,
            },
            couleur: BLEU,
            reflexion: 0.,
        };

        let sol = Objet {
            forme: Forme::Plan {
                point: Vec3::new(0.0, -1.0, 0.0),
                normale: Vec3::new(0.0, 1.0, 0.0),
            },
            couleur: Couleur::new(0.5, 0.5, 0.5),
            reflexion: 0.2,
        };

        Scene {
            objets: vec![sol, sphere_bleu, sphere_rouge, sphere_verte],
            lumiere: lum,
        }
    }
}
