use crate::{
    couleur::{BLANC, BLEU_C, Couleur},
    objet::{Objet, plus_proche},
    ray::Rayon,
    vec3::{ORIGINE, Vec3},
};

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
