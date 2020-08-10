use dcd::{DCD, Message};
use dcd::Database;
use dcd::{Commit,   Project,   User, };
use dcd::{CommitId, ProjectId, UserId};
use std::io::Read;

pub struct DCDMock {
    database: DCD,
    commit_messages: Vec<Message>,
}

impl DCDMock {
    pub fn new(root: String) -> DCDMock {
        let database = DCD::new(root.clone());
        let commit_messages =
            DCDMock::load_commit_messages("data/mock_commit_messages.csv".to_string());

        DCDMock { database, commit_messages }
    }

    fn get_mock_message(&self, id: CommitId) -> Option<&Message> {
        self.commit_messages.get((id % self.commit_messages.len() as u64) as usize)
    }

    fn load_commit_messages(path: String) -> Vec<Message> {
        let mut messages = vec![];

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)
            .unwrap();

        for result in rdr.byte_records() {
            let record = result.unwrap();
            // println!("{:?}\n", record);
            match record.get(1) {
                Some(field) => messages.push(field.to_vec()),
                None => messages.push(vec![]),
            }
        }

        messages
    }
}

impl Database for DCDMock {
    fn num_projects(&self) -> u64 {
        self.database.num_projects()
    }

    fn num_commits(&self) -> u64 {
        self.database.num_commits()
    }

    fn num_users(&self) -> u64 {
        self.database.num_users()
    }

    fn get_project(&self, id: ProjectId) -> Option<Project> {
        self.database.get_project(id)
    }

    fn get_commit(&self, id: CommitId) -> Option<Commit> {
        match self.database.get_commit(id) {
            Some(mut commit) => {
                match commit.message {
                    Some(_) => Some(commit),
                    None => {
                        commit.message = self.get_mock_message(id).map(|m| m.clone());
                        Some(commit)
                    }
                }
            }
            None => None,
        }
    }

    fn get_user(&self, id: UserId) -> Option<&User> {
        self.database.get_user(id)
    }

    // fn get_snapshot(&self, id: BlobId) -> Option<Snapshot> {
    //     self.snapshots.get(&id).map(|snapshot| snapshot.clone())
    // }

    // fn get_file_path(&self, id: PathId) -> Option<FilePath> {
    //     self.paths.get(&id).map(|path| path.clone())
    // }
}

#[cfg(test)]
mod tests {
    use crate::DCDMock;
    use dcd::{Database, Message};

    #[test] fn test_load_commits() {
        DCDMock::load_commit_messages("data/mock_commit_messages.csv".to_string());
    }

    #[test] fn test_load_whole_DCD() {
        let dcd = DCDMock::new("/dejavuii/dejacode/dataset-small/".to_string());

        let expected: Option<Message> = Some("ostylowanie\n".as_bytes().to_vec());
        let actual: Option<Message> = dcd.get_commit(0).unwrap().message;

        assert_eq!(expected, actual);
    }
}