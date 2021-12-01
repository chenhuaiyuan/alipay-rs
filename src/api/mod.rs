mod fund;

use crate::Client;

macro_rules! create_api {
    ($ ($ident: ident) +) => {
        $(
            pub struct $ident {
                client: Client,
            }

            impl $ident {
                pub fn new(client: Client) -> $ident {
                    $ident {client}
                }
            }
        )+
    };
}

create_api![Fund];
