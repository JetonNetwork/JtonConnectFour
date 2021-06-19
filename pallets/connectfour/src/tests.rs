use super::*;
use crate::{Error, mock::*};

use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(ConnectFour::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(ConnectFour::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			ConnectFour::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn test_game_creation() {
	new_test_ext().execute_with(|| {

		// Test player can not play against himself
		assert_noop!(
			ConnectFour::new_game(Origin::signed(1), 1),
			Error::<Test>::NoFakePlay
		);

		// Test game creation between to different players
		assert_ok!(ConnectFour::new_game(Origin::signed(1), 2));
		run_to_block(1);

		let board_id_1 = ConnectFour::player_board(1);
		let board_id_2 = ConnectFour::player_board(2);

		assert_eq!(board_id_1, board_id_2);

		assert_noop!(
			ConnectFour::new_game(Origin::signed(1), 3),
			Error::<Test>::PlayerBoardExists
		);

		assert_noop!(
			ConnectFour::new_game(Origin::signed(3), 2),
			Error::<Test>::PlayerBoardExists
		);

		let board = ConnectFour::boards(board_id_1);

		assert_eq!(board.last_turn, 0);

	});
}

#[test]
fn test_game_play() {
	new_test_ext().execute_with(|| {

		// Test game creation between to different players
		assert_ok!(ConnectFour::new_game(Origin::signed(1), 2));
		run_next_block();

		let board_id = ConnectFour::player_board(1);

		let board = ConnectFour::boards(board_id);

		if board.board_state == BoardState::Red {
			assert_ok!(ConnectFour::play_turn(Origin::signed(1)));
			run_next_block();
		}

		//assert_eq!(board.board_state, BoardState::Blue);
		assert_eq!(board.last_turn, 0);

	});
}