use anchor_lang::prelude::*;
use std::cmp::Ordering;

declare_id!("6s5JqB54MEjW6SX4AtYgWQJEsLKj6C1Q5pj1LCD6PnY7");

#[program]
pub mod strawsoll {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, options: Vec<String>) -> Result<()> {
        ctx.accounts.poll.init(options)
    }

    pub fn vote(ctx: Context<Vote>, vote_id: u8) -> Result<()> {
        ctx.accounts.poll.vote(vote_id, ctx.accounts.voter.key())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PollOption {
    // Size: 54 + 1 + 4 = 59 bytes
    pub label: String, // up to 50 char. Size: 4 + 50 = 54 bytes
    pub id: u8,        // Size: 1 byte
    pub votes: u32,    // Size: 4 bytes
}

#[account]
pub struct Poll {
    // Size: 1 + 299 + 1604 = 1904
    pub options: Vec<PollOption>, // 5 PollOption array = 4 + (59 * 5) = 299
    pub voters: Vec<Pubkey>, // 50 voters array = 4 + (32 * 50) = 1604
    pub finished: bool, // bool = 1
}

impl Poll {
    pub const MAXIMUM_SIZE: usize = 1904;

    pub fn init(&mut self, options: Vec<String>) -> Result<()> {
        require_eq!(self.finished, false, StarSollError::PollAlreadyFinished);
        let mut c = 0;

        self.options = options
            .iter()
            .map(|option| {
                c += 1;

                PollOption {
                    label: option.clone(),
                    id: c,
                    votes: 0,
                }
            })
            .collect();
        self.finished = false;
        Ok(())
    }

    pub fn vote(&mut self, vote_id: u8, voter_key: Pubkey) -> Result<()> {
        require_eq!(self.finished, false, StarSollError::PollAlreadyFinished);
        require_eq!(self.options.iter().filter(|option| option.id == vote_id).collect::<Vec<&PollOption>>().len(), 1, StarSollError::PollOptionNotFound);
        require_eq!(self.voters.iter().filter(|voter| voter.cmp(&&voter_key) == Ordering::Equal).collect::<Vec<&Pubkey>>().len(), 0, StarSollError::UserAlreadyVoted);

        self.voters.push(voter_key);
        self.options = self.options
            .iter()
            .map(|option| {
                let mut _option = option.clone();

                if _option.id == vote_id {
                    _option.votes += 1;
                }

                _option
            })
            .collect();
        
        Ok(())
    }
}

#[error_code]
pub enum StarSollError {
    PollAlreadyFinished,
    PollOptionNotFound,
    UserAlreadyVoted
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + Poll::MAXIMUM_SIZE)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub voter: Signer<'info>,
}
