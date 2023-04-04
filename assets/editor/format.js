function onFormatResult() {
    replaceWithClipboard(s => {
        return `let v = match ${s} {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}",err.to_string());
            return Err(err.to_string())?;
        }
    };`
    })

}

function onFormatUnwrap() {
    replaceWithClipboard(s => {
        return `match ${s}{
        Some(v) => Ok(v)
        None=>Err(Status::InternalServerError)
        }`
    })
}

function onFormatLog() {
    replaceWithClipboard(s => {
        return `log::error!("{}",${s}.to_string());`
    })
}

function onFormatUnwrapOr() {
    replaceWithClipboard(s => {
        return `${s}.unwrap_or(String::default()).as_str()`
    })
}

async function replaceWithClipboard(fn) {
    const strings = await readText();
    textarea.setRangeText(fn(strings.trim()), textarea.selectionStart, textarea.selectionEnd, 'end');
}