mod hello;

use crate::discord::interaction::{
    ApplicationCommandInteractionData, InteractionResponse, InteractionResponseType,
};

pub(crate) async fn handle_command(
    data: &ApplicationCommandInteractionData,
) -> InteractionResponse {
    match data.name.as_str() {
        "hello" => hello::hello().await,
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}
