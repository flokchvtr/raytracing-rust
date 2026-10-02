use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::couleur::{BLEU, Couleur, ROUGE, VERT}; // + ta couleur de sol
use crate::forme::Forme;
use crate::objet::Objet;
use crate::rendu::couleur_pixel;
use crate::vec3::Vec3;

mod couleur;
mod forme;
mod objet;
mod ray;
mod rendu;
mod vec3;

fn main() -> io::Result<()> {
    let hauteur = 225;
    let largeur = 400;

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

    let objets = vec![sol, sphere_bleu, sphere_rouge, sphere_verte];

    let pixels: Vec<Couleur> = (0..largeur * hauteur)
        .map(|i| couleur_pixel(i % largeur, i / largeur, largeur, hauteur, &objets))
        .collect();

    let mut f = BufWriter::new(File::create("image.ppm")?);
    ecrire_ppm(&mut f, largeur, hauteur, &pixels)?;
    f.flush()?;
    Ok(())
}

fn ecrire_entete(f: &mut impl Write, largeur: usize, hauteur: usize) -> io::Result<()> {
    writeln!(f, "P3")?;
    writeln!(f, "{} {}", largeur, hauteur)?;
    writeln!(f, "255")?;
    Ok(())
}

fn ecrire_ppm(
    f: &mut impl Write,
    largeur: usize,
    hauteur: usize,
    pixels: &[couleur::Couleur],
) -> io::Result<()> {
    ecrire_entete(f, largeur, hauteur)?;
    for &c in pixels {
        let [r, g, b] = couleur::convert_bytes(c);
        writeln!(f, "{r} {g} {b}")?;
    }
    Ok(())
}
