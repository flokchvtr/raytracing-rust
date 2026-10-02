use crate::ray::Rayon;
use crate::vec3::Vec3;

pub const EPS: f64 = 1e-6;

#[derive(Debug)]
pub enum Forme {
    Sphere { centre: Vec3, rayon: f64 },
    Plan { point: Vec3, normale: Vec3 },
}

impl Forme {
    pub fn nom(&self) -> &'static str {
        match self {
            Self::Sphere { .. } => "sphère",
            Self::Plan { .. } => "plan",
        }
    }

    pub fn normale_en(&self, p: Vec3) -> Vec3 {
        match self {
            Self::Sphere { centre, rayon } => (p - *centre) * (1. / rayon),
            Self::Plan { normale, .. } => *normale,
        }
    }

    pub fn intersecte(&self, r: Rayon) -> Option<f64> {
        match self {
            Self::Sphere { centre, rayon } => {
                let oc = r.origine - *centre;
                let h = r.dir.dot(oc);
                let c = oc.length_squared() - (rayon * rayon);
                let delta = h * h - c;

                if delta < 0. {
                    return None;
                }

                let racine = delta.sqrt();
                let t1 = -h - racine;
                let t2 = -h + racine;

                if t1 > EPS {
                    Some(t1)
                } else if t2 > EPS {
                    Some(t2)
                } else {
                    None
                }
            }
            Self::Plan { point, normale } => {
                let denom = r.dir.dot(*normale); // D·N

                if denom.abs() < EPS {
                    return None;
                }

                let t = (*point - r.origine).dot(*normale) / denom;

                if t > EPS { Some(t) } else { None }
            }
        }
    }
}
