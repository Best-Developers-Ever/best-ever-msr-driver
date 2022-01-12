use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::ntstatus::STATUS_NOT_IMPLEMENTED;

#[derive(Clone, Copy, Debug)]
pub struct Error(NTSTATUS);

impl Error {
    pub const NOT_IMPLEMENTED: Error = Error(STATUS_NOT_IMPLEMENTED);

    pub fn from_nt_status(status: NTSTATUS) -> Error {
        Error(status)
    }

    pub fn to_nt_status(&self) -> NTSTATUS {
        self.0
    }
}

pub trait IntoResult {
    fn into_result(self) -> Result<(), Error>;
}

impl IntoResult for NTSTATUS {
    fn into_result(self) -> Result<(), Error> {
        match self {
            winapi::shared::ntstatus::STATUS_SUCCESS => {
                Ok(())
            }
            status => {
                Err(Error::from_nt_status(status))
            }
        }
    }
}