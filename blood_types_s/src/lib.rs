#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let vec = self.donors();
        let mut can_receive: bool = false;
        let other_bt = BloodType {
            antigen: other.antigen.clone(),
            rh_factor: other.rh_factor.clone(),
        };

        for (_, item) in vec.iter().enumerate() {
            if item == &other_bt {
                can_receive = true;
            }
        }

        can_receive
    }

    pub fn donors(&self) -> Vec<Self> {
        match self.antigen {
            Antigen::A => {
                if self.rh_factor == RhFactor::Positive {
                    return vec![
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        },
                    ];
                } else {
                    return vec![
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        },
                    ];
                }
            }
            Antigen::O => {
                if self.rh_factor == RhFactor::Positive {
                    return vec![
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        },
                    ];
                } else {
                    return vec![BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    }];
                }
            }
            Antigen::B => {
                if self.rh_factor == RhFactor::Positive {
                    return vec![
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        },
                    ];
                } else {
                    return vec![
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        },
                    ];
                }
            }
            Antigen::AB => {
                if self.rh_factor == RhFactor::Positive {
                    return vec![
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        },
                    ];
                } else {
                    return vec![
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        },
                    ];
                }
            }
        }
    }

    pub fn recipients(&self) -> Vec<Self> {
        match self.antigen {
            Antigen::A => {
                if self.rh_factor == RhFactor::Positive {
                    return vec![
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        },
                    ];
                } else {
                    return vec![
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        },
                    ];
                }
            }
            Antigen::O => {
                if self.rh_factor == RhFactor::Positive {
                    return vec![
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        },
                    ];
                } else {
                    return vec![
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::A,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::O,
                            rh_factor: RhFactor::Negative,
                        },
                    ];
                }
            }
            Antigen::B => {
                if self.rh_factor == RhFactor::Positive {
                    return vec![
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        },
                    ];
                } else {
                    return vec![
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::B,
                            rh_factor: RhFactor::Positive,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        },
                    ];
                }
            }
            Antigen::AB => {
                if self.rh_factor == RhFactor::Positive {
                    return vec![BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    }];
                } else {
                    return vec![
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Negative,
                        },
                        BloodType {
                            antigen: Antigen::AB,
                            rh_factor: RhFactor::Positive,
                        },
                    ];
                }
            }
        }
    }
}
