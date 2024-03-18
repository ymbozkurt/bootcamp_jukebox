use anchor_lang::prelude::*;

declare_id!("A3wMAxHeZ1rRgHT6NGRATXUtV69KVaN5YtR5gZwweA44");

#[program]
mod jukebox {
    use super::*;

    pub fn create_jukebox(ctx: Context<CreateJukebox>) -> Result<()> {
        let jukebox = &mut ctx.accounts.jukebox;
        jukebox.initialize();
        jukebox.owner = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn add_to_queue(ctx: Context<AddToQueue>, song_no: u8) -> Result<()> {
        let jukebox = &mut ctx.accounts.jukebox;
        let song_name: String = (&jukebox.song_list[(song_no-1) as usize]).to_string();
        jukebox.playing_next.push(song_name);

        Ok(())
    }

    pub fn get_list(ctx: Context<GetList>) -> Result<()> {
        let jukebox = & ctx.accounts.jukebox;
        (&jukebox.playing_next).to_vec();
        Ok(())
    }


}

#[derive(Accounts)]
pub struct AddToQueue<'info> {
    authority: Signer<'info>,

    #[account(mut)]
    jukebox: Account<'info, Jukebox>,
}

#[derive(Accounts)]
pub struct GetList<'info> {
    #[account()]
    jukebox: Account<'info, Jukebox>,
}

#[derive(Accounts)]
pub struct CreateJukebox<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 10 * 32 + 10 + 1000)]
    jukebox: Account<'info, Jukebox>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}



#[account]
pub struct Jukebox {
    owner: Pubkey,
    song_list: [String; 10], // JukeBox'ta bulunan tüm şarkıların listesi
    playing_next: Vec<String>,   // Sıradaki şarkı listesi
    }

impl<'info> Jukebox {
    pub fn initialize(&mut self) {
        self.song_list = [
            "The Weekend - Moth To A Flame".to_string(),
            "Özkan Uğur - Bir Bakman Lazım".to_string(),
            "Metalicca - Wherever I May Roam".to_string(),
            "The Heavy - Short Change Hero".to_string(),
            "OneRepublic - I Aint't Worried".to_string(),
            "Daft Punk - Lose Yourself to Dance".to_string(),
            "Ceza - Feyz Al".to_string(),
            "Drake - Nonstop".to_string(),
            "Ezhel - Allah'ından Bul".to_string(),
            "Led Zeppelin - Baby I'm Gonna Leave You".to_string(),
        ];
    }
}