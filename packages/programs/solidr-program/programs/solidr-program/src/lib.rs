use anchor_lang::prelude::*;

use crate::instructions::{global::*, members::*, sessions::*};

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("2xTttZsc5s65KyLmG1M6D5NpanUdYGj9SydbYnQFjnUP");

#[program]
pub mod solidr {

    use instructions::*;

    use super::*;

    pub fn init_global(ctx: Context<InitGlobalContextData>) -> Result<()> {
        global::init_global(ctx)
    }

    /**
     * Anyone can open new session. Session's creator becomes session administrator.
     *
     * @dev An event SessionCreated is emitted
     *
     * @param name The session name
     * @param description The session description
     * @param member_name The administrator's name
     */
    pub fn open_session(
        ctx: Context<OpenSessionContextData>,
        name: String,
        description: String,
        member_name: String,
    ) -> Result<()> {
        sessions::open_session(ctx, name, description, member_name)
    }

    /**
     * Administrator can close sessions he created.
     *
     * @dev An event SessionClosed is emitted
     */
    pub fn close_session(ctx: Context<CloseSessionContextData>) -> Result<()> {
        sessions::close_session(ctx)
    }

    /**
     * Session's administrator can set invitation token hash
     *
     * @param hash The token hash to store in session
     */
    pub fn set_session_token_hash(
        ctx: Context<SetSessionHashContextData>,
        hash: [u8; 32],
    ) -> Result<()> {
        sessions::set_session_token_hash(ctx, hash)
    }

    /**
     * Session administrator can add members.
     *
     * @dev members can be added only by session administrator when session is opened
     * An event MemberAdded is emitted
     *
     * @param addr The address of the member to add
     * @param name The nickname of the member to add
     */
    pub fn add_session_member(
        ctx: Context<AddSessionMemberContextData>,
        addr: Pubkey,
        name: String,
    ) -> Result<()> {
        members::add_session_member(ctx, addr, name)
    }

    /**
     * Anyone can join a session with correct information provided with a share link.
     *
     * An event MemberAdded is emitted
     *
     * @param name The nickname of the member to add
     * @param token The token shared by session's administrator
     */
    pub fn join_session_as_member(
        ctx: Context<JoinSessionAsMemberContextData>,
        name: String,
        token: String,
    ) -> Result<()> {
        members::join_session_as_member(ctx, name, token)
    }
}
