use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::{Context, ContextBuilder, GameResult};
use std::path::{PathBuf};
use valterm_chess::*;

const SQUARE_SIZE: i32 = 100;

struct GameWrapper {
    game: Game,
    piece_images: PieceImages,
}

struct PieceImages {
    white_pawn: Image,
    white_knight: Image,
    white_bishop: Image,
    white_rook: Image,
    white_queen: Image,
    white_king: Image,
    black_pawn: Image,
    black_knight: Image,
    black_bishop: Image,
    black_rook: Image,
    black_queen: Image,
    black_king: Image,
}

impl PieceImages {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let white_pawn = Image::new(ctx, "/wP.png")?;
        let white_knight = Image::new(ctx, "/wN.png")?;
        let white_bishop = Image::new(ctx, "/wB.png")?;
        let white_rook = Image::new(ctx, "/wR.png")?;
        let white_queen = Image::new(ctx, "/wQ.png")?;
        let white_king = Image::new(ctx, "/wK.png")?;

        let black_pawn = Image::new(ctx, "/bP.png")?;
        let black_knight = Image::new(ctx, "/bN.png")?;
        let black_bishop = Image::new(ctx, "/bB.png")?;
        let black_rook = Image::new(ctx, "/bR.png")?;
        let black_queen = Image::new(ctx, "/bQ.png")?;
        let black_king = Image::new(ctx, "/bK.png")?;

        Ok(PieceImages {
            white_pawn,
            white_knight,
            white_bishop,
            white_rook,
            white_queen,
            white_king,
            black_pawn,
            black_knight,
            black_bishop,
            black_rook,
            black_queen,
            black_king,
        })
    }
    

    fn get_image(&self, piece: &Piece) -> &Image {
        match (piece.color, piece.piece_type) {
            (valterm_chess::Color::White, PieceType::Pawn) => &self.white_pawn,
            (valterm_chess::Color::White, PieceType::Knight) => &self.white_knight,
            (valterm_chess::Color::White, PieceType::Bishop) => &self.white_bishop,
            (valterm_chess::Color::White, PieceType::Rook) => &self.white_rook,
            (valterm_chess::Color::White, PieceType::Queen) => &self.white_queen,
            (valterm_chess::Color::White, PieceType::King) => &self.white_king,
            (valterm_chess::Color::Black, PieceType::Pawn) => &self.black_pawn,
            (valterm_chess::Color::Black, PieceType::Knight) => &self.black_knight,
            (valterm_chess::Color::Black, PieceType::Bishop) => &self.black_bishop,
            (valterm_chess::Color::Black, PieceType::Rook) => &self.black_rook,
            (valterm_chess::Color::Black, PieceType::Queen) => &self.black_queen,
            (valterm_chess::Color::Black, PieceType::King) => &self.black_king,
        }
    }
}

impl GameWrapper {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut game = Game::new();
        game.default_board();
        let piece_images = PieceImages::new(ctx)?;
        Ok(GameWrapper { game, piece_images })
    }
}

impl EventHandler for GameWrapper {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        let light_color = Color::from_rgb(240, 217, 181);  
        let dark_color = Color::from_rgb(181, 136, 99);   
        
        for r in 0..8 {
            for c in 0..8 {
                let rect = graphics::Rect::new(
                    (SQUARE_SIZE * c) as f32, 
                    (SQUARE_SIZE * r) as f32, 
                    SQUARE_SIZE as f32, 
                    SQUARE_SIZE as f32
                );
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    if (r + c) % 2 == 0 { light_color } else { dark_color }
                )?;
                graphics::draw(ctx, &rectangle, DrawParam::default())?;
            }
        }
        
        let pieces = self.game.get_pieces();
        println!("{:?} {} {}", pieces[0].color, pieces[0].position.x, pieces[0].position.y);
        for piece in pieces {
            let image = self.piece_images.get_image(&piece);
            let draw_params = DrawParam::default()
                .dest([
                    (SQUARE_SIZE * piece.position.x as i32) as f32,
                    (SQUARE_SIZE * piece.position.y as i32) as f32,
                ])
                .scale([SQUARE_SIZE as f32 / image.width() as f32, SQUARE_SIZE as f32 / image.height() as f32]);  // Scale the image to fit the square

            graphics::draw(ctx, image, draw_params)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let resource_dir = PathBuf::from("./resources");

    let mode = ggez::conf::WindowMode::default().dimensions((SQUARE_SIZE * 8) as f32, (SQUARE_SIZE * 8) as f32);
    let setup = ggez::conf::WindowSetup::default().title("tbeskow-Chess");

    let (mut ctx, event_loop) = ContextBuilder::new("chess", "Theodor")
        .add_resource_path(resource_dir)
        .window_mode(mode)
        .window_setup(setup)
        .build()
        .expect("Error: Could not create Ggez content");

    let game_wrapper = GameWrapper::new(&mut ctx).expect("Failed to initialize GameWrapper");
    event::run(ctx, event_loop, game_wrapper);
}
