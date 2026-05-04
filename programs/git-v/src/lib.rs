use anchor_lang::prelude::*;

declare_id!("Sg4bheXXXdMrcTmsSr3d2bQZhmGmBeFJCJYtkXst3Hp");

pub mod drive_accounts;
pub mod states;

pub use drive_accounts::*;
pub use states::*;

#[program]
pub mod git_v {

    use super::*;
}
