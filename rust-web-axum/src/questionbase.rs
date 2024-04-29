/*
use create::*;

#[derive(Debug)]
pub struct QuestionBase {
    file: File,
    questionmap: QuestionMap,
}

impl QuestionBase {
    pub fn new<P: AsRef<std::path::Path>> (
        db_path P,
        allow_empty: bool,
) -> Result<Self, std::io::Error> {
        let opened = File::option().read(true).write(true).open(&db_path);
        let mut file = match opened {
            Ok(f) => f,
            Err(e) => {
                if e.kind() != ErrorKind::NotFound || !allow_empty {
                    return Err(e);
                }
                let mut f = File::create_new(&db_path)?;
                let questionmap: QuestionMap = HashMap::new();
                let json = serde_json::to_string(&questionmap).unwrap();
                f.write_all(json.asbytes())?;
                f.sync_all()?;
                f.reqind()?;
                f
            }
        };
    }
}
*/