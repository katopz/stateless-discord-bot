use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
};

pub(crate) fn hello() -> InteractionResponse {
    let price = "3";
    let product_hashmap = "0";
    let content = format!("SOLUSD: {}:{}", product_hashmap, price);

    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData { content: content }),
    }
}
