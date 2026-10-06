use nova_types::Digest;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct JobSpec{pub title:String,pub objective:String,pub capabilities:Vec<String>,pub tools:Vec<String>,pub policies:Vec<String>,pub evaluation:Vec<String>}

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct AgentSpec{pub role_digest:Digest,pub capabilities:Vec<String>,pub tools:Vec<String>,pub policies:Vec<String>,pub evaluation:Vec<String>}

#[derive(Debug,Clone,Eq,PartialEq)]
pub enum CompileError{MissingTitle,MissingObjective,MissingCapability,MissingEvaluation}

pub fn compile(job:&JobSpec)->Result<AgentSpec,CompileError>{
 if job.title.is_empty(){return Err(CompileError::MissingTitle)}
 if job.objective.is_empty(){return Err(CompileError::MissingObjective)}
 if job.capabilities.is_empty(){return Err(CompileError::MissingCapability)}
 if job.evaluation.is_empty(){return Err(CompileError::MissingEvaluation)}
 let mut b=Vec::new();for s in [&job.title,&job.objective]{b.extend_from_slice(&(s.len() as u64).to_le_bytes());b.extend_from_slice(s.as_bytes())}for xs in [&job.capabilities,&job.tools,&job.policies,&job.evaluation]{for s in xs{b.extend_from_slice(&(s.len() as u64).to_le_bytes());b.extend_from_slice(s.as_bytes())}}
 Ok(AgentSpec{role_digest:Digest::of(b"nova.workforce.role.v1",&b),capabilities:job.capabilities.clone(),tools:job.tools.clone(),policies:job.policies.clone(),evaluation:job.evaluation.clone()})
}

#[cfg(test)]
mod tests{use super::*;fn job()->JobSpec{JobSpec{title:"Security Researcher".into(),objective:"find reproducible defects".into(),capabilities:vec!["analyze".into()],tools:vec!["repo".into()],policies:vec!["sandbox".into()],evaluation:vec!["reproduce".into()]}}
#[test]fn compilation_is_deterministic(){assert_eq!(compile(&job()).unwrap(),compile(&job()).unwrap())}
#[test]fn missing_evaluation_is_rejected(){let mut j=job();j.evaluation.clear();assert_eq!(compile(&j),Err(CompileError::MissingEvaluation))}
#[test]fn capability_is_explicit(){let a=compile(&job()).unwrap();assert_eq!(a.capabilities,vec!["analyze"])} }
