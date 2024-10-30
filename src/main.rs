use async_openai;
use tokio;
use image;
use base64::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let vars = std::env::vars();
	
	let mut api_base = String::new();
	for var in vars {
		// if var.0 == "XINFERENCE_API_BASE".to_string() {
		// 	api_base = var.1;
		// }
		if var.0 == "OLLAMA_API_BASE".to_string() {
			api_base = var.1;
		}
	}
	println!("using {api_base} as LLM api endpoint...");
	let configuration = async_openai::config::OpenAIConfig::new()
		.with_api_key("lm-studio")
		.with_api_base(api_base);
	let client = async_openai::Client::with_config(configuration);
	
	println!("loading image...");
	let image_bytes = image::ImageReader::open("/Users/xinyubao/Downloads/C1416896449951.jpg")?
	    .decode()?;
	let mut raw_image_bytes: Vec<u8> = Vec::new();
	image_bytes.write_to(&mut std::io::Cursor::new(&mut raw_image_bytes), image::ImageFormat::Png)?;
	let base64_image = BASE64_STANDARD.encode(raw_image_bytes);
	let image = format!("data:image/jpeg;base64,{base64_image}");
	
	println!("trying to create a request...");
	let request = async_openai::types::CreateChatCompletionRequestArgs::default()
	    .model("minicpm-v")
		.response_format(
			async_openai::types::ResponseFormat::JsonObject
		)
		.messages(
			[
				async_openai::types::ChatCompletionRequestUserMessageArgs::default()
					.content(
						vec![
							async_openai::types::ChatCompletionRequestMessageContentPartTextArgs::default()
								.text(
									"Evaluate the overall shape of the person’s face. Determine if the face is round, long, or has a standard length-to-width ratio. For standard proportions, use a numerical ratio approximately 1.618:1. Assign points as follows: add 2 to softness if the face is round; add 2 to hardness if the face is long; no points are added for standard proportions. Provide the results in JSON format as {“hardness”: int, “softness”: int}. Possible scores: hardness can be 0 or 2; softness can be 0 or 2."
								)
								.build()?
								.into(),
							async_openai::types::ChatCompletionRequestMessageContentPartImageArgs::default()
								.image_url(
									async_openai::types::ImageUrlArgs::default()
										.url(image)
										.detail(async_openai::types::ImageDetail::Auto)
										.build()?
								)
								.build()?
								.into()
						]
					)
					.build()?
					.into(),
			]
		)
		.build()?;
	
	println!("trying to fetch response...");
	let response = client.chat().create(request).await?;
	
	println!("accessing the response...");
	println!("{:?}", response.choices[0].message.content);
    
    Ok(())
}
