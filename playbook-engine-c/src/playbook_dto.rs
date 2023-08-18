use serde::Deserialize;

/// Structures to communicate with VC

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookDto {
    pub task: StepDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepsBlockDto {
    #[serde(default)]
    pub condition: Option<String>,
    pub steps: Vec<StepDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParallelBlockDto {
    #[serde(default)]
    pub condition: Option<String>,
    pub steps: Vec<StepDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StepDto {
    Single(SingleStepDto),
    Steps(StepsBlockDto),
    Parallel(ParallelBlockDto),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleStepDto {
    pub seq: u32,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub condition: Option<String>,
    pub run: String,
    #[serde(default)]
    pub params: Vec<StepParamDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepParamDto {
    pub name: String,
    pub value: String,
}
