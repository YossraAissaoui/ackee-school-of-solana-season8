//-------------------------------------------------------------------------------
///
/// TASK: Implement the add comment functionality for the Twitter program
/// 
/// Requirements:
/// - Validate that comment content doesn't exceed maximum length
/// - Initialize a new comment account with proper PDA seeds
/// - Set comment fields: content, author, parent tweet, and bump
/// - Use content hash in PDA seeds for unique comment identification
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    // TODO: Implement add comment functionality
    // todo!()
    // Validate that comment content doesn't exceed maximum length
    if comment_content.len() > COMMENT_LENGTH {
        return Err(TwitterError::CommentTooLong.into());
    }
    // Initialize a new comment account with proper PDA seeds
    let comment = &mut ctx.accounts.comment;
    let tweet = &ctx.accounts.tweet;
    // Set comment fields: author, parent tweet, content and bump
    comment.comment_author = ctx.accounts.comment_author.key();
    comment.parent_tweet = tweet.key();
    comment.content = comment_content;
    comment.bump = ctx.bumps.comment;

    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    // TODO: Add required account constraints

    // New comment account with proper PDA seeds
    // Use content hash in PDA seeds for unique comment identification and import hash from solana_program
    #[account(mut)]
    pub comment_author: Signer<'info>,

    #[account(
        init,
        payer = comment_author,
        space = 8 + Comment::INIT_SPACE,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            { hash(comment_content.as_bytes()).to_bytes().as_ref() },
            tweet.key().as_ref()
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
    //pub comment_author: Signer<'info>,
    //pub comment: Account<'info, Comment>,
    //pub tweet: Account<'info, Tweet>,
    //pub system_program: Program<'info, System>,
}
