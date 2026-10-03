use crate::vec3::Vec3;

pub type Couleur = Vec3;

#[cfg(test)]
pub const ROUGE: Couleur = Couleur::new(1.0, 0.0, 0.0);
#[cfg(test)]
pub const VERT: Couleur = Couleur::new(0.0, 1.0, 0.0);
#[cfg(test)]
pub const BLEU: Couleur = Couleur::new(0.0, 0.0, 1.0);
pub const BLEU_C: Couleur = Couleur::new(0.5, 0.7, 1.0);
pub const BLANC: Couleur = Couleur::new(1.0, 1.0, 1.0);

fn composante(x: f64) -> u8 {
    (x.clamp(0.0, 1.0).sqrt() * 255.999) as u8
}

pub fn convert_bytes(c: Couleur) -> [u8; 3] {
    [composante(c.x), composante(c.y), composante(c.z)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_bornee() {
        assert_eq!(convert_bytes(Couleur::new(0.0, 0.0, 0.0)), [0, 0, 0]);
        assert_eq!(convert_bytes(Couleur::new(1.0, 1.0, 1.0)), [255, 255, 255]);
        assert_eq!(convert_bytes(Couleur::new(1.5, -0.2, 0.5)), [255, 0, 181]); // sqrt
    }
}
