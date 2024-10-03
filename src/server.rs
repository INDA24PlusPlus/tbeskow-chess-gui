use std::net::TcpListener;
use valterm_chess::*;
use std::io::prelude::*;
use chess_networking::*;
use bincode;



pub fn run() -> std::io::Result<()>{
    let addr: String = "127.0.0.1:5000".to_string();
    let listener = TcpListener::bind(addr)?;

    let (mut stream1, _addr) = listener.accept()?;
    let (mut stream2, _addr2) = listener.accept()?;
    // add connected verification
    
    let mut start_state = Start{
        is_white: true,
        name: Some("Player 1".to_string()),
        fen: None,
        time: None,
        inc: None,
    };
    let _ = stream1.write(&bincode::serialize(&start_state).unwrap());
    start_state.is_white = false;
    start_state.name = Some("Player 2".to_string());
    let _ = stream2.write(&bincode::serialize(&start_state).unwrap());
    
    println!("Data sent");
    let mut game = Game::new();
    game.default_board();
    loop{
        let mut buf = [0u8; 128]; 
        if game.current_move == Color::White{
            stream1.read(&mut buf)?;
        }else{
            stream2.read(&mut buf)?;
        }
        let position = Position { x: buf[0] as i8, y: buf[1] as i8 };
        let to_position = Position { x: buf[2] as i8, y: buf[3] as i8 };

        println!("{} {}", position, to_position);
        let move_type = game.move_piece(position, to_position);
        if move_type == valterm_chess::moves::MoveType::Invalid {continue;}


    }
    
    // Ok(())
}
