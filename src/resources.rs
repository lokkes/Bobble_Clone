use ggez::graphics;

pub struct Resources {
    pub player_images: Vec<graphics::Image>,
    pub grid_image: graphics::Image,
    pub press_space_image: graphics::Image,
    pub bullet_left_image: graphics::Image,
    pub bullet_right_image: graphics::Image,
    pub bobble_image: Vec<graphics::Image>,
}

impl Resources {
    pub fn load(ctx: &mut ggez::Context) -> Self {
        Resources {
            player_images: vec![
                graphics::Image::from_path(ctx, "/still.png").unwrap(),
                graphics::Image::from_path(ctx, "/run00.png").unwrap(),
                graphics::Image::from_path(ctx, "/run01.png").unwrap(),
                graphics::Image::from_path(ctx, "/run02.png").unwrap(),
                graphics::Image::from_path(ctx, "/run03.png").unwrap(),
                graphics::Image::from_path(ctx, "/run10.png").unwrap(),
                graphics::Image::from_path(ctx, "/run11.png").unwrap(),
                graphics::Image::from_path(ctx, "/run12.png").unwrap(),
                graphics::Image::from_path(ctx, "/run13.png").unwrap(),
                graphics::Image::from_path(ctx, "/jump0.png").unwrap()
            ],
            grid_image: graphics::Image::from_path(ctx, "/block0.png").unwrap(),
            press_space_image: graphics::Image::from_path(ctx, "/space1.png").unwrap(),
            bullet_left_image: graphics::Image::from_path(ctx, "/orb6.png").unwrap(),
            bullet_right_image: graphics::Image::from_path(ctx, "/bolt10.png").unwrap(),
            bobble_image: vec![
                graphics::Image::from_path(ctx, "/trap00.png").unwrap(),
                graphics::Image::from_path(ctx, "/trap01.png").unwrap(),
                graphics::Image::from_path(ctx, "/trap02.png").unwrap(),
                graphics::Image::from_path(ctx, "/trap03.png").unwrap(),
                graphics::Image::from_path(ctx, "/trap04.png").unwrap(),
                graphics::Image::from_path(ctx, "/trap05.png").unwrap(),
                graphics::Image::from_path(ctx, "/trap06.png").unwrap(),
                graphics::Image::from_path(ctx, "/trap07.png").unwrap()
            ],
        }
    }
}
