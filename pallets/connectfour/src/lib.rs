#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

use codec::{Encode, Decode};
use frame_support::{
	log,
	traits::{Randomness},
};
use frame_system::{
	WeightInfo
};
use sp_runtime::{
	traits::{Hash, TrailingZeroInput}
};
use sp_std::vec::{
	Vec
};
use log::info;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// importing the `weights.rs` here
pub mod weights;

/// Implementations of some helper traits passed into runtime modules as associated types.
pub mod connectfour;
use connectfour::{Logic};


/// A type alias for the balance type from this pallet's point of view.
//type BalanceOf<T> = <T as pallet_balances::Config>::Balance;
//const MILLICENTS: u32 = 1_000_000_000;

#[derive(Encode, Decode, Clone, PartialEq)]
pub enum BoardState {
	None = 0,
	Red = 1,
	Blue = 2,
	//Winner(AccountId) = 3,
}

impl Default for BoardState { fn default() -> Self { Self::None } }


/// Connect four board structure containing two players and the board
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct BoardStruct<Hash, AccountId, BlockNumber, BoardState> {
	id: Hash,
	red: AccountId,
	blue: AccountId,
	board: [[u8; 7]; 6],
	last_turn: BlockNumber,
	board_state: BoardState,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	// important to use outside structs and consts
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		/// The generator used to supply randomness to contracts through `seal_random`.
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		// /// Weight information for extrinsics in this pallet.
		//type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn founder_key)]
	pub type FounderKey<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn boards)]
	/// Store all boards that are currently being played.
	pub type Boards<T: Config> = StorageMap<_, Identity, T::Hash, BoardStruct<T::Hash, T::AccountId, T::BlockNumber, BoardState>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn player_board)]
	/// Store players active board, currently only one board per player allowed.
	pub type PlayerBoard<T: Config> = StorageMap<_, Identity, T::AccountId, T::Hash, ValueQuery>;

	// Default value for Nonce
	#[pallet::type_value]
	pub fn NonceDefault<T: Config>() -> u64 { 0 }
	// Nonce used for generating a different seed each time.
	#[pallet::storage]
	pub type Nonce<T: Config> = StorageValue<_, u64, ValueQuery, NonceDefault<T>>;

	// The genesis config type.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub founder_key: T::AccountId,
	}

	// The default value for the genesis config type.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				founder_key: Default::default(),
			}
		}
	}

	// The build of genesis for the pallet.
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<FounderKey<T>>::put(&self.founder_key);
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		
		/// A new board got created.
		NewBoard(T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Player already has a board which is being played.
		PlayerBoardExists,
		/// Player can't play against them self.
		NoFakePlay,
	}

	// Pallet implements [`Hooks`] trait to define some logic to execute in some context.
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// `on_initialize` is executed at the beginning of the block before any extrinsic are
		// dispatched.
		//
		// This function must return the weight consumed by `on_initialize` and `on_finalize`.
		fn on_initialize(_: T::BlockNumber) -> Weight {
			// Anything that needs to be done at the start of the block.
			// We don't do anything here.
			0
		}

		// `on_finalize` is executed at the end of block after all extrinsic are dispatched.
		fn on_finalize(_n: BlockNumberFor<T>) {
			// Perform necessary data/state clean up here.
		}

		// A runtime code run after every block and have access to extended set of APIs.
		//
		// For instance you can generate extrinsics for the upcoming produced block.
		fn offchain_worker(_n: T::BlockNumber) {
			// We don't do anything here.
			// but we could dispatch extrinsic (transaction/unsigned/inherent) using
			// sp_io::submit_extrinsic.
			// To see example on offchain worker, please refer to example-offchain-worker pallet
		 	// accompanied in this repository.
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Print out log or debug message in the console via log::{error, warn, info, debug, trace},
			// accepting format strings similar to `println!`.
			// https://substrate.dev/rustdocs/v3.0.0/log/index.html
			info!("New value is now: {:?}", something);

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		/// Create game for two players
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn new_game(origin: OriginFor<T>, opponent: T::AccountId) -> DispatchResult {
			
			let sender = ensure_signed(origin)?;

			// Don't allow playing against yourself.
			ensure!(sender != opponent, Error::<T>::NoFakePlay);

			// Make sure players have no board open.
			ensure!(!PlayerBoard::<T>::contains_key(&sender), Error::<T>::PlayerBoardExists);
			ensure!(!PlayerBoard::<T>::contains_key(&opponent), Error::<T>::PlayerBoardExists);
			
			// Create new game
			let board_id = Self::create_game(sender.clone(), opponent.clone());

			// Add board to the players playing it.
			<PlayerBoard<T>>::insert(sender, board_id);
			<PlayerBoard<T>>::insert(opponent, board_id);

			Ok(())
		}

		/// Create game for two players
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn play_turn(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {

	/// Update nonce once used. 
	fn encode_and_update_nonce(
	) -> Vec<u8> {
		let nonce = <Nonce<T>>::get();
		<Nonce<T>>::put(nonce.wrapping_add(1));
		nonce.encode()
	}

	/// Generates a random hash out of a seed.
	fn generate_random_hash(
		phrase: &[u8], 
		sender: T::AccountId
	) -> T::Hash {
		let (seed, _) = T::Randomness::random(phrase);
		let seed = <[u8; 32]>::decode(&mut TrailingZeroInput::new(seed.as_ref()))
			.expect("input is padded with zeroes; qed");
		return (seed, &sender, Self::encode_and_update_nonce()).using_encoded(T::Hashing::hash);
	}

	/// Generate a new game between two players.
	fn create_game(
		red: T::AccountId, 
		blue: T::AccountId
	) -> T::Hash {
		// get a random hash as board id
		let board_id = Self::generate_random_hash(b"create", red.clone());
		// calculate plyer to start the first turn, with the first byte of the board_id random hash
		let turn_start = if board_id.as_ref()[0] < 128 { BoardState::Red } else { BoardState::Blue };
		// get current blocknumber
		let block_number = <frame_system::Pallet<T>>::block_number();
		// create a new empty bgame oard
		let board = BoardStruct {
			id: board_id,
			red: red,
			blue: blue,
			board: [[0u8; 7]; 6],
			last_turn: block_number,
			board_state: turn_start,
		};
		// insert the new board into the storage
		<Boards<T>>::insert(board_id, board);
		// emit event for a new board creation
		// Emit an event.
		Self::deposit_event(Event::NewBoard(board_id));

		return board_id;
	}
}


