use crate::{couleur::*, objet::*, ray::*, scene::*, vec3::*};

const AMBIANTE: f64 = 0.1;
const PROFONDEUR_MAX: u32 = 50;

pub fn rayon_camera(x: usize, y: usize, largeur: usize, hauteur: usize) -> Rayon {
    let ratio = largeur as f64 / hauteur as f64;
    let u = ratio * (2.0 * (x as f64 + 0.5) / largeur as f64 - 1.0);
    let v = 1.0 - 2.0 * (y as f64 + 0.5) / hauteur as f64;
    Rayon {
        origine: ORIGINE,
        dir: Vec3::new(u, v, -1.0).normalized(),
    }
}

pub fn ciel(dir: Vec3) -> Couleur {
    let a = 0.5 * (dir.y + 1.0);
    (1. - a) * BLANC + a * BLEU_C
}

fn eclairement(n: Vec3, l: Vec3) -> f64 {
    AMBIANTE + (1. - AMBIANTE) * n.dot(l).max(0.)
}

fn dans_l_ombre(scene: &Scene, p: Vec3) -> bool {
    let vers_lumiere = scene.lumiere.position - p;
    let distance = vers_lumiere.length();

    let r = Rayon {
        origine: p,
        dir: vers_lumiere.normalized(),
    };

    scene.objets.iter().any(|o| match o.forme.intersecte(r) {
        Some(t) => t <= distance,
        None => false,
    })
}

pub fn couleur_pixel(x: usize, y: usize, largeur: usize, hauteur: usize, scene: &Scene) -> Couleur {
    let r = rayon_camera(x, y, largeur, hauteur);
    couleur_rayon(scene, r, PROFONDEUR_MAX)
}

pub fn couleur_rayon(scene: &Scene, r: Rayon, profondeur: u32) -> Couleur {
    match plus_proche(&scene.objets, r) {
        Some((t, objet)) => {
            let p = r.at(t);
            let n = objet.forme.normale_en(p);
            let local = if dans_l_ombre(scene, p) {
                objet.couleur * AMBIANTE
            } else {
                let l = (scene.lumiere.position - p).normalized();
                objet.couleur * eclairement(n, l)
            };
            if objet.reflexion == 0.0 || profondeur == 0 {
                local
            } else {
                let r = Rayon {
                    origine: p,
                    dir: r.dir.reflechi(n),
                };
                let reflet = couleur_rayon(scene, r, profondeur - 1);
                (1. - objet.reflexion) * local + objet.reflexion * (objet.couleur * reflet)
            }
        }
        None => ciel(r.dir),
    }
}

pub fn rendu(largeur: usize, hauteur: usize, scene: &Scene) -> Vec<Couleur> {
    (0..largeur * hauteur)
        .map(|i| couleur_pixel(i % largeur, i / largeur, largeur, hauteur, scene))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forme::Forme;
    use crate::lumiere::Lumiere;

    #[test]
    fn milieu_touche_la_sphere_verte() {
        let scene = Scene::demo();
        let (largeur, hauteur) = (400, 225);
        let c = couleur_pixel(largeur / 2, hauteur / 2, largeur, hauteur, &scene);
        assert!(c.x < c.y);
        assert!(c.z < c.y);
    }

    #[test]
    fn eclairement_test() {
        let n = Vec3::new(0., 1., 0.);
        assert!((eclairement(n, n) - 1.0).abs() < 1e-9);
        assert_eq!(eclairement(n, -n), AMBIANTE);
    }

    #[test]
    fn ombre_sous_la_sphere() {
        let scene = Scene {
            objets: vec![
                Objet {
                    forme: Forme::Sphere {
                        centre: Vec3::new(0.0, 0.0, -3.0),
                        rayon: 0.5,
                    },
                    couleur: VERT,
                    reflexion: 1.,
                },
                Objet {
                    forme: Forme::Plan {
                        point: Vec3::new(0.0, -1.0, 0.0),
                        normale: Vec3::new(0.0, 1.0, 0.0),
                    },
                    couleur: BLANC,
                    reflexion: 1.,
                },
            ],
            lumiere: Lumiere {
                position: Vec3::new(0.0, 5.0, -3.0),
            },
        };

        assert!(dans_l_ombre(&scene, Vec3::new(0.0, -1.0, -3.0)));
        assert!(!dans_l_ombre(&scene, Vec3::new(5.0, -1.0, -3.0)));
    }
}
