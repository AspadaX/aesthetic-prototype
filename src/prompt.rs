use image::DynamicImage;

pub struct Prompt {
	content: String,
	image: DynamicImage
}

impl Prompt {
	pub fn new(
		prompt: &str,
		image_path: &str
	) -> Result<Self, Box<dyn std::error::Error>> {
		let image: DynamicImage = image::ImageReader::open(image_path)?
		    .decode()?;
		
		Ok(
			Self { content: prompt.to_string(), image: image }
		)
	}
	
	/// this reads prompts from a json file, and it loads each prompt
	/// into the memory for generations. 
	pub fn new_batch(
		prompt_file_path: &str
	) -> Result<Vec<Prompt>, Box<dyn std::error::Error>> {
		let mut prompts: Vec<Prompt> = Vec::new();
		Ok(prompts)
	}
}