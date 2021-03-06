use std::sync::mpsc::Receiver;

use scamoku::{game::*, rule::standard::*};

#[test]
fn standard_gomoku() {
    let Handle {
        player_handles: ((tx1, _), (tx2, _)),
        event_rx: _,
        join_handle,
    } = Builder::with_rule(&StandardGomoku).strict().start();
    for i in 0..5 {
        tx1.make_move((i, i).into());
        tx2.make_move((i + 1, i).into());
    }

    assert_eq!(
        join_handle.join().unwrap(),
        GameResult {
            kind: GameResultKind::RowCompleted,
            winning_side: Some(Side::First),
        }
    );
}

#[test]
fn freestyle_gomoku_raw() {
    let RawHandle {
        msg_tx,
        event_rx: _,
        join_handle,
    } = Builder::with_rule(&FreestyleGomoku).start_raw();
    for i in 0..5 {
        msg_tx.make_move((if i == 4 { 5 } else { i }, 0).into());
        msg_tx.make_move((i * 2, 1).into());
    }
    msg_tx.claim_win((4, 0).into());

    assert_eq!(
        join_handle.join().unwrap(),
        GameResult {
            kind: GameResultKind::RowCompleted,
            winning_side: Some(Side::First),
        }
    );
}

#[test]
fn errors() {
    let Handle {
        player_handles: ((_tx1, _), (tx2, _)),
        event_rx,
        join_handle: _,
    } = Builder::with_rule(&StandardGomoku).strict().start();

    tx2.make_move((0, 0).into());
    assert_next_err(event_rx, "not your turn to move");
}

fn assert_next_err(event_rx: Receiver<Event>, msg: &str) {
    let actual_msg = event_rx
        .iter()
        .find_map(|e| {
            if let Event::Error(_, m) = e {
                Some(m)
            } else {
                None
            }
        })
        .unwrap();
    assert_eq!(actual_msg, msg);
}
