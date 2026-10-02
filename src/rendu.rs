use crate::{couleur::*, forme::*, objet::*, ray::*, vec3::*};

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

pub fn couleur_pixel(
    x: usize,
    y: usize,
    largeur: usize,
    hauteur: usize,
    scene: &[Objet],
) -> Couleur {
    let r = rayon_camera(x, y, largeur, hauteur);
    match plus_proche(scene, r) {
        Some((_, objet)) => objet.couleur,
        None => ciel(r.dir),
    }
}

pub fn scene() -> Vec<Objet> {
    let sphere_rouge = Objet {
        forme: Forme::Sphere {
            centre: Vec3::new(-1.2, 0.0, -3.0),
            rayon: 0.5,
        },
        couleur: ROUGE,
    };

    let sphere_verte = Objet {
        forme: Forme::Sphere {
            centre: Vec3::new(0.0, 0.0, -3.0),
            rayon: 0.5,
        },
        couleur: VERT,
    };

    let sphere_bleu = Objet {
        forme: Forme::Sphere {
            centre: Vec3::new(1.2, 0.0, -3.0),
            rayon: 0.5,
        },
        couleur: BLEU,
    };

    let sol = Objet {
        forme: Forme::Plan {
            point: Vec3::new(0.0, -1.0, 0.0),
            normale: Vec3::new(0.0, 1.0, 0.0),
        },
        couleur: Couleur::new(0.5, 0.5, 0.5),
    };
    vec![sol, sphere_bleu, sphere_rouge, sphere_verte]
}

pub fn rendu(largeur: usize, hauteur: usize, scene: &[Objet]) -> Vec<Couleur> {
    (0..largeur * hauteur)
        .map(|i| couleur_pixel(i % largeur, i / largeur, largeur, hauteur, scene))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn milieu_est_vert() {
        let scene = scene();
        let (largeur, hauteur) = (400, 225);
        let c = couleur_pixel(largeur / 2, hauteur / 2, largeur, hauteur, &scene);
        assert_eq!(c, VERT);
    }
}
