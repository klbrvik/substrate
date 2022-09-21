initSidebarItems({"enum":[["Error","The errors that might occur in this crate and `frame-election-provider-solution-type`."]],"fn":[["setup_inputs","Converts raw inputs to types used in this crate."],["to_support_map","Build the support map from the assignments."],["to_supports","Same as [`to_support_map`] except it returns a flat vector."]],"mod":[["balancing","Balancing algorithm implementation."],["helpers","Helper methods for npos-elections."],["node","(very) Basic implementation of a graph node used in the reduce algorithm."],["phragmen","Implementation of the sequential-phragmen election method."],["phragmms","Implementation of the PhragMMS method."],["pjr","Implements functions and interfaces to check solutions for being t-PJR."],["reduce","Rust implementation of the Phragmén reduce algorithm. This can be used by any off chain application to reduce cycles from the edge assignment, which will result in smaller size."],["traits","Traits for the npos-election operations."]],"struct":[["Assignment","A voter’s stake assignment among a set of targets, represented as ratios."],["BalancingConfig","Utility struct to group parameters for the balancing algorithm."],["Candidate","A candidate entity for the election."],["Edge","A vote being casted by a [`Voter`] to a [`Candidate`] is an `Edge`."],["ElectionResult","Final result of the election."],["ElectionScore","The score of an election. This is the main measure of an election’s quality."],["StakedAssignment","A voter’s stake assignment among a set of targets, represented as absolute values in the scale of [`ExtendedBalance`]."],["Support","A structure to demonstrate the election result from the perspective of the candidate, i.e. how much support each candidate is receiving."],["Voter","A voter entity."]],"trait":[["EvaluateSupport","Extension trait for evaluating a support map or vector."]],"type":[["CandidatePtr","A pointer to a candidate struct with interior mutability."],["ExtendedBalance","A type in which performing operations on vote weights are safe."],["SupportMap","Linkage from a winner to their [`Support`]."],["Supports","A target-major representation of the the election outcome."],["VoteWeight","A type which is used in the API of this crate as a numeric weight of a vote, most often the stake of the voter. It is always converted to [`ExtendedBalance`] for computation."]]});