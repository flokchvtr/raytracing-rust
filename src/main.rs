use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

mod couleur;
mod forme;
mod ray;
mod vec3;

fn main() -> io::Result<()> {
    let hauteur = 225;
    let largeur = 400;

    let rouge = vec![couleur::ROUGE; hauteur * largeur];
    let mut f = BufWriter::new(File::create("image.ppm")?);
    ecrire_ppm(&mut f, hauteur, hauteur, &rouge)?;
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
