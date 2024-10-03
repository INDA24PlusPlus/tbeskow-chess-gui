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
        let mut buf = [0u8; 1024]; // Increased buffer size to accommodate Move struct
        let bytes_read = if game.current_move == Color::White {
            stream1.read(&mut buf)?
        } else {
            stream2.read(&mut buf)?
        };
        

        if let Ok(chess_move) = bincode::deserialize::<Move>(&buf[..bytes_read]) {
            let position = Position { x: chess_move.from.0 as i8, y: chess_move.from.1 as i8 };
            let to_position = Position { x: chess_move.to.0 as i8, y: chess_move.to.1 as i8 };

            println!("{:?} {:?}", position, to_position);

            println!("From: {:?}, To: {:?}", position, to_position);
            let move_type = game.move_piece(position, to_position);
            if move_type == valterm_chess::moves::MoveType::Invalid {continue;}
            let _ = stream1.write(&bincode::serialize(&Move{
                from: (position.x as u8, position.y as u8),
                to: (to_position.x as u8, to_position.y as u8),
                promotion: None,
                forfeit: false,
                offer_draw: false
            }).unwrap());
            let _ = stream2.write(&bincode::serialize(&Move{
                from: (position.x as u8, position.y as u8),
                to: (to_position.x as u8, to_position.y as u8),
                promotion: None,
                forfeit: false,
                offer_draw: false
            }).unwrap());
        } else {
            println!("Failed to deserialize move data");
            continue;
        }


    }
    
    // Ok(())
}
