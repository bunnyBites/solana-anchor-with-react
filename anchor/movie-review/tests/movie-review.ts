import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MovieReview } from "../target/types/movie_review";
import { expect } from "chai";

describe("movie-review", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MovieReview as Program<MovieReview>;

  const newMovie = {
    title: "Spiderman-6",
    description: "With great powers comes great responsibility",
    rating: 4,
  };

  const updatedMovie = {
    description: "Spider-Man saves NYC from Green Goblin's terror.",
    rating: 5,
  };

  const [moviePDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(newMovie.title), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(newMovie.title, newMovie.description, newMovie.rating)
      .accounts([moviePDA])
      .rpc();

    console.log("Your transaction signature", tx);

    const movieAccount = await program.account.movie.fetch(moviePDA);

    expect(movieAccount.title === newMovie.title);
    expect(movieAccount.description === newMovie.description);
    expect(movieAccount.rating === newMovie.rating);
  });

  it("Is Updated!", async () => {
    const tx = await program.methods
      .update(newMovie.title, updatedMovie.description, updatedMovie.rating)
      .accounts([moviePDA])
      .rpc();

    console.log("update successfull", tx);

    const movieAccount = await program.account.movie.fetch(moviePDA);

    expect(movieAccount.title === newMovie.title);
    expect(movieAccount.description === updatedMovie.description);
    expect(movieAccount.rating === updatedMovie.rating);
  });

  it("Is Deleted!", async () => {
    const tx = await program.methods
      .deleteMovie(newMovie.title)
      .accounts(moviePDA)
      .rpc();

    console.log("Movie Deleted: ", tx);
  });
});
