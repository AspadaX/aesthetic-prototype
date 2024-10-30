use rustface::{
	Detector,
	FaceInfo,
	ImageData
};
use image::{self, EncodableLayout, Luma};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

pub fn extract_face_information(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut image_file = image::ImageReader::open(filepath)?
	    .with_guessed_format()?
	    .decode()?
		.to_luma8();
	let model_path = "seeta_fd_frontal_v1.0.bin";
	let mut detector = rustface::create_detector(
		&model_path
	)
		.unwrap();
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);
    
    let grey_image_file = image_file.as_bytes();
    let (width, height) = image_file.dimensions();
    let mut image = ImageData::new(
    	&grey_image_file, 
     	width, 
      	height
    );
    for face in detector.detect(&mut image).into_iter() {
        // print confidence score and coordinates
        println!("found face: {:?}", face);
        let bbox = face.bbox();
        let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());

        draw_hollow_rect_mut(&mut image_file, rect, Luma([255]));
    }
    
    image_file.save("./result.png")?;
    
    Ok(())
}
