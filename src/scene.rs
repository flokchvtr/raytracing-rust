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
    /// Scène de présentation : un grand miroir entouré de sphères colorées.
    pub fn vitrine() -> Self {
        let sphere = |x: f64, z: f64, rayon: f64, couleur: Couleur, reflexion: f64| Objet {
            // posée sur le sol (y = -1) : le centre est à une hauteur `rayon` au-dessus
            forme: Forme::Sphere { centre: Vec3::new(x, -1.0 + rayon, z), rayon },
            couleur,
            reflexion,
        };

        Scene {
            objets: vec![
                Objet {
                    forme: Forme::Plan {
                        point: Vec3::new(0.0, -1.0, 0.0),
                        normale: Vec3::new(0.0, 1.0, 0.0),
                    },
                    couleur: Couleur::new(0.55, 0.55, 0.6),
                    reflexion: 0.3,
                },
                sphere(0.0, -4.0, 1.0, Couleur::new(0.95, 0.95, 0.95), 0.9),  // grand miroir
                sphere(-2.3, -4.6, 0.6, Couleur::new(1.0, 0.45, 0.1), 0.15),  // orange
                sphere(2.2, -3.8, 0.7, Couleur::new(0.1, 0.65, 0.7), 0.35),   // turquoise
                sphere(-0.95, -2.6, 0.3, Couleur::new(0.6, 0.2, 0.85), 0.2),  // violet
                sphere(1.05, -2.5, 0.25, Couleur::new(1.0, 0.85, 0.15), 0.1), // jaune
                sphere(-4.5, -10.0, 2.0, Couleur::new(0.8, 0.12, 0.15), 0.4), // rouge, au fond
                sphere(4.0, -8.0, 1.2, Couleur::new(0.25, 0.75, 0.3), 0.2),   // vert, au fond
            ],
            lumiere: Lumiere { position: Vec3::new(-4.0, 6.0, 1.0) },
        }
    }
}
