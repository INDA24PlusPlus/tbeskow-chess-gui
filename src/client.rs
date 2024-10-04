use std::io::prelude::*;
use std::net::TcpStream;
use chess_networking::*;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::{Context, ContextBuilder, GameResult};
use std::path::PathBuf;
use valterm_chess::*;
use std::sync::{Arc, Mutex};
use std::io::{self, Read};


const SQUARE_SIZE: i32 = 100;

#[derive(Clone)]
struct GameWrapper {
    game: Game,
    piece_images: PieceImages,
    selected_piece: Option<Position>,  
    is_white: bool,
    stream: Arc<Mutex<TcpStream>>,
}

#[derive(Clone)]
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
        let game = Game::new();
        let piece_images = PieceImages::new(ctx)?;

        let addr = "127.0.0.1:5000";
        let stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;

        Ok(GameWrapper { 
            game, 
            piece_images, 
            selected_piece: None, 
            is_white: true,
            stream: Arc::new(Mutex::new(stream)),
        })
    }

    fn handle_click(&mut self, mouse_x: f32, mouse_y: f32) {
        if self.is_white^(self.game.current_move == valterm_chess::Color::White){
            self.selected_piece = None;
        }
        let board_x = (mouse_x / SQUARE_SIZE as f32).floor() as i32;
        let board_y = 7-(mouse_y / SQUARE_SIZE as f32).floor() as i32;

        if board_x >= 0 && board_x < 8 && board_y >= 0 && board_y < 8 {
            let position = Position { 
                x: if !self.is_white { 7 - board_x } else { board_x } as i8,
                y: if !self.is_white { 7 - board_y } else { board_y } as i8 
            };
            println!("{:?}", position);
            println!("{:?}", self.selected_piece);

            
            if let Some(selected_position) = self.selected_piece {

                let _ = (*self.stream.lock().unwrap()).write(&bincode::serialize(&Move{
                    from: (selected_position.x as u8, selected_position.y as u8),
                    to: (position.x as u8, position.y as u8),
                    promotion: None,
                    forfeit: false,
                    offer_draw: false
                }).unwrap());
                
                self.selected_piece = Some(position);
            }else{
                self.selected_piece = Some(position);
            }
        }
    }

    fn handle_network(&mut self) -> io::Result<()> {
        let mut buffer = [0; 1024]; 
        loop {
            let mut stream = self.stream.lock().unwrap();
            match (*stream).read(&mut buffer) {
                Ok(0) => {
                    println!("Server closed the connection");
                    break;
                }
                Ok(n) => {
                    if let Ok(received) = bincode::deserialize::<Start>(&buffer[..n]) {
                        println!("Received: {:?}, is_white: {}", received.name, received.is_white);
                        let is_white = received.is_white;
                        self.is_white = is_white;
                        self.game.default_board();
                    } else if let Ok(received) = bincode::deserialize::<Move>(&buffer[..n]) {
                        self.game.move_piece(
                            Position { x: received.from.0 as i8, y: received.from.1 as i8 },
                            Position { x: received.to.0 as i8, y: received.to.1 as i8 }
                        );
                    } else {
                        println!("Failed to deserialize data");
                    }
                }
                Err(_e) => {
                    break;
                }
            }
        }
        Ok(())
    
    }
}

impl EventHandler for GameWrapper {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if let Err(e) = self.handle_network() {
            eprintln!("Network error: {}", e);
        }
        
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
        
        // Draw pieces
        let pieces = self.game.get_pieces();
        for piece in pieces {
            let image = self.piece_images.get_image(&piece);
            let draw_params = DrawParam::default()
                .dest([
                    (SQUARE_SIZE * (if self.is_white { piece.position.x } else { 7 - piece.position.x }) as i32) as f32,
                    (SQUARE_SIZE * (if self.is_white { 7 - piece.position.y } else { piece.position.y }) as i32) as f32,
                ])
                .scale([SQUARE_SIZE as f32 / image.width() as f32, SQUARE_SIZE as f32 / image.height() as f32]); 

            graphics::draw(ctx, image, draw_params)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.handle_click(x, y);
        }
    }
}

pub fn run() -> std::io::Result<()> {

    
    let resource_dir = PathBuf::from("./resources");

    let mode = ggez::conf::WindowMode::default().dimensions((SQUARE_SIZE * 8) as f32, (SQUARE_SIZE * 8) as f32);
    let setup = ggez::conf::WindowSetup::default().title("tbeskow-Chess");

    let (mut ctx, event_loop) = ContextBuilder::new("chess", "Theodor")
        .add_resource_path(resource_dir)
        .window_mode(mode)
        .window_setup(setup)
        .build()
        .expect("Error: Could not create Ggez content");

    let game_wrapper = Arc::new(Mutex::new(GameWrapper::new(&mut ctx).expect("Failed to initialize GameWrapper")));
    

    event::run(ctx, event_loop, (*game_wrapper.lock().unwrap()).clone());
}

