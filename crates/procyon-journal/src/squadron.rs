use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum Squadron {
    AppliedToSquadron {},
    DisbandedSquadron {},
    InvitedToSquadron {},
    JoinedSquadron {},
    KickedFromSquadron {},
    LeftSquadron {},
    SharedBookmarkToSquadron {},
    SquadronCreated {},
    SquadronDemotion {},
    SquadronPromotion {},
    SquadronStartup {},
    WonATrophyForSquadron {},
}
