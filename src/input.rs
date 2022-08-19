use syn::{parse::{Parse, ParseStream}, Result, LitStr, Ident, token, LitInt};

pub(crate) struct StaticFileMacroInput {
    pub app_ident: Ident,
    pub path: LitStr,
    pub directory: LitStr,
}

impl Parse for StaticFileMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let app_ident: Ident = input.parse()?;
        input.parse::<token::Comma>()?;
        let path: LitStr = input.parse()?;
        input.parse::<token::Comma>()?;
        let directory: LitStr = input.parse()?;
        Ok(
            StaticFileMacroInput{
                app_ident,
                path,
                directory,
            }
        )
    }
}

pub(crate) struct IncludeFileMacroInput {
    pub app_ident: Ident,
    pub path: LitStr,
    pub directory: LitStr,
    pub max_file_size: Option<LitInt>,
}

impl Parse for IncludeFileMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let app_ident: Ident = input.parse()?;
        input.parse::<token::Comma>()?;
        let path: LitStr = input.parse()?;
        input.parse::<token::Comma>()?;
        let directory: LitStr = input.parse()?;
        let mut max_file_size = None;
        if input.lookahead1().peek(token::Comma) {
            input.parse::<token::Comma>()?;
            max_file_size = Some(input.parse::<LitInt>()?);
        }
        Ok(
            IncludeFileMacroInput{
                app_ident,
                path,
                directory,
                max_file_size
            }
        )
    }
}