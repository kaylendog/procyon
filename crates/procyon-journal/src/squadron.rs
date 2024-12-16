use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Squadron {
    AppliedToSquadron,
    DisbandedSquadron,
    InvitedToSquadron,
    JoinedSquadron,
    KickedFromSquadron,
    LeftSquadron,
    SharedBookmarkToSquadron,
    SquadronCreated,
    SquadronDemotion,
    SquadronPromotion,
    SquadronStartup,
    WonATrophyForSquadron,
}
