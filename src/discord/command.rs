mod hello;

use crate::{
    cfkv::WorkersKv,
    discord::interaction::{
        ApplicationCommandInteractionData, InteractionResponse, InteractionResponseType,
    },
};

pub(crate) async fn handle_command(
    data: &ApplicationCommandInteractionData,
    kv: &WorkersKv,
) -> InteractionResponse {
    match data.name.as_str() {
        "hello" => hello::hello(kv).await,
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}
