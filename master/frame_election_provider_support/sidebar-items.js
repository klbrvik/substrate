window.SIDEBAR_ITEMS = {"derive":[["RuntimeDebug",""]],"enum":[["Error","The errors that might occur in this crate and `frame-election-provider-solution-type`."]],"macro":[["generate_solution_type","Re-export the solution generation macro. Generates a struct to store the election result in a small/compact way. This can encode a structure which is the equivalent of a `sp_npos_elections::Assignment<_>`."],["runtime_benchmarks_enabled","Enable/disable the given code depending on `feature = \"runtime-benchmarks\"` being enabled for the crate or not."],["runtime_benchmarks_or_fuzz_enabled","Enable/disable the given code depending on `any(feature = \"runtime-benchmarks\", feature = \"fuzzing\")` being enabled for the crate or not."]],"mod":[["benchmarking","Election provider support pallet benchmarking. This is separated into its own crate to avoid bloating the size of the runtime."],["data_provider","Types that are used by the data provider trait."],["onchain","An implementation of [`ElectionProvider`] that uses an `NposSolver` to do the election. As the name suggests, this is meant to be used onchain. Given how heavy the calculations are, please be careful when using it onchain."],["traits","Traits for the election operations."],["weights","Autogenerated weights for frame_election_provider_support"]],"struct":[["ApprovalVoting","A wrapper for [`sp_npos_elections::approval_voting()`] that implements [`NposSolver`]. See the documentation of [`sp_npos_elections::approval_voting()`] for more info."],["Assignment","A voter’s stake assignment among a set of targets, represented as ratios."],["BalancingConfig","Utility struct to group parameters for the balancing algorithm."],["BoundedVec","A bounded vector."],["ElectionResult","Final result of the election."],["IndexAssignment","The [`IndexAssignment`] type is an intermediate between the assignments list ([`&[Assignment<T>]`][Assignment]) and `SolutionOf<T>`."],["NoElection","An election provider that does nothing whatsoever."],["PhragMMS","A wrapper for [`sp_npos_elections::phragmms()`] that implements [`NposSolver`]. See the documentation of [`sp_npos_elections::phragmms()`] for more info."],["SequentialPhragmen","A wrapper for [`sp_npos_elections::seq_phragmen`] that implements [`NposSolver`]. See the documentation of [`sp_npos_elections::seq_phragmen`] for more info."],["Support","A structure to demonstrate the election result from the perspective of the candidate, i.e. how much support each candidate is receiving."],["Weight",""]],"trait":[["ElectionDataProvider","Something that can provide the data to an [`ElectionProvider`]."],["ElectionProvider","Elect a new set of winners, bounded by `MaxWinners`."],["ElectionProviderBase","Base trait for types that can provide election"],["Get","A trait for querying a single value from a type."],["IdentifierT","an aggregator trait for a generic type of a voter/target identifier. This usually maps to substrate’s account id."],["InstantElectionProvider","A (almost) marker trait that signifies an election provider as working synchronously. i.e. being instant."],["NposSolver","Something that can compute the result to an NPoS solution."],["PerThing","Re-export some type as they are used in the interface. Something that implements a fixed point ration with an arbitrary granularity `X`, as parts per `X`."],["PerThing128","Aggregator trait for a PerThing that can be multiplied by u128 (ExtendedBalance)."],["ScoreProvider","Something that can provide the `Score` of an account. Similar to [`ElectionProvider`] and [`ElectionDataProvider`], this should typically be implementing by whoever is supposed to use `SortedListProvider`."],["SortedListProvider","A utility trait for something to implement `ElectionDataProvider` in a sensible way."]],"type":[["BoundedSupports","Same as `Supports` but bounded by `B`."],["BoundedSupportsOf","Same as `BoundedSupports` but parameterized by a `ElectionProviderBase`."],["ExtendedBalance","A type in which performing operations on vote weights are safe."],["IndexAssignmentOf","A type alias for [`IndexAssignment`] made from [`NposSolution`]."],["Supports","A target-major representation of the the election outcome."],["VoteWeight","A type which is used in the API of this crate as a numeric weight of a vote, most often the stake of the voter. It is always converted to [`ExtendedBalance`] for computation."],["Voter","A voter, at the level of abstraction of this crate."],["VoterOf","Same as [`Voter`], but parameterized by an [`ElectionDataProvider`]."]]};