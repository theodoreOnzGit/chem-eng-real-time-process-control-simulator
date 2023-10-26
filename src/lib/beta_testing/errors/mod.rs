use thiserror::Error;

/// Master Error type of this crate
#[derive(Debug, Error)]
pub enum ChemEngProcessControlSimulatorError {


    /// it's a generic error which is a placeholder since I used 
    /// so many string errors
    #[error("Placeholder Error Type for Strings{0} ")]
    GenericStringError(String),

    /// when transfer function is unstable when it should be 
    /// stable 
    #[error("Unstable Damping Factor for Stable Transfer Function")]
    UnstableDampingFactorForStableTransferFunction,


    
}

///  converts ThermalHydraulicsLibError from string error
impl From<String> for ChemEngProcessControlSimulatorError {
    fn from(value: String) -> Self {
        Self::GenericStringError(value)
    }
}

impl Into<String> for ChemEngProcessControlSimulatorError {
    fn into(self) -> String {
        match self {
            ChemEngProcessControlSimulatorError::GenericStringError(string) => {
                string
            },
            ChemEngProcessControlSimulatorError::UnstableDampingFactorForStableTransferFunction => {
                self.into()
            },

        }
    }
}


