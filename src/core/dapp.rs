use crate::entities::dapp::{Dapp};

#[tracing::instrument]
pub async fn get_dapps() -> Vec<Dapp> {
    let mut dapp_list: Vec<Dapp> = Vec::new();
    let dapp = Dapp {
        title: String::from("Shadow-X"),
        description: String::from("Shadow-X 666"),
    };
    dapp_list.push(dapp);
    return dapp_list;
}

