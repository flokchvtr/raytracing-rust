use crate::rendu::rendu;
use crate::scene::Scene;

use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::couleur::{Couleur, convert_bytes};
mod couleur;
mod forme;
mod lumiere;
mod objet;
mod ray;
mod rendu;
mod scene;
mod vec3;
fn main() -> io::Result<()> {
    let hauteur = 225;
    let largeur = 400;

    let scene = Scene::demo();

    let debut = std::time::Instant::now();
    let pixels: Vec<Couleur> = rendu(largeur, hauteur, &scene);
    eprintln!("calcul : {:?}", debut.elapsed());

    let mut f = BufWriter::new(File::create("image.ppm")?);

    let debut = std::time::Instant::now();
    ecrire_ppm(&mut f, largeur, hauteur, &pixels)?;
    f.flush()?;
    eprintln!("écriture : {:?}", debut.elapsed());

    Ok(())
}

fn ecrire_entete(f: &mut impl Write, largeur: usize, hauteur: usize) -> io::Result<()> {
    writeln!(f, "P6")?;
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
        f.write_all(&convert_bytes(c))?;
    }
    Ok(())
}
