/**
 * Database service layer - TypeScript wrapper for Tauri database commands
 */

import { invoke } from '@tauri-apps/api/core'

export interface Resume {
  id?: number
  title: string
  content: string
  created_at: string
  updated_at: string
}

export interface JobDescription {
  id?: number
  title: string
  content: string
  created_at: string
  updated_at: string
}

export interface InterviewSession {
  id?: number
  resume_id?: number
  job_description_id?: number
  questions: string[]
  created_at: string
}

export interface InterviewAnswer {
  id?: number
  session_id: number
  question_index: number
  question: string
  answer: string
  feedback: string
  created_at: string
}

export interface QuestionBankItem {
  id?: number
  question: string
  best_answer?: string
  notes?: string
  job_category?: string
  created_at: string
  updated_at: string
}

// Resume operations
export async function saveResume(title: string, content: string): Promise<number> {
  return await invoke('db_save_resume', { title, content })
}

export async function getResumes(): Promise<Resume[]> {
  return await invoke('db_get_resumes')
}

export async function deleteResume(id: number): Promise<void> {
  return await invoke('db_delete_resume', { id })
}

// Job description operations
export async function saveJobDescription(
  title: string,
  content: string
): Promise<number> {
  return await invoke('db_save_job_description', { title, content })
}

export async function getJobDescriptions(): Promise<JobDescription[]> {
  return await invoke('db_get_job_descriptions')
}

export async function deleteJobDescription(id: number): Promise<void> {
  return await invoke('db_delete_job_description', { id })
}

// Interview session operations
export async function createSession(
  resumeId: number | null,
  jobDescriptionId: number | null,
  questions: string[]
): Promise<number> {
  return await invoke('db_create_session', {
    resumeId,
    jobDescriptionId,
    questions
  })
}

export async function getSessions(): Promise<InterviewSession[]> {
  return await invoke('db_get_sessions')
}

export async function getSession(sessionId: number): Promise<InterviewSession | null> {
  return await invoke('db_get_session', { sessionId })
}

export async function saveAnswer(
  sessionId: number,
  questionIndex: number,
  question: string,
  answer: string,
  feedback: string
): Promise<number> {
  return await invoke('db_save_answer', {
    sessionId,
    questionIndex,
    question,
    answer,
    feedback
  })
}

export async function getAnswers(sessionId: number): Promise<InterviewAnswer[]> {
  return await invoke('db_get_answers', { sessionId })
}

// Question bank operations
export async function addToBank(
  question: string,
  bestAnswer?: string,
  notes?: string,
  jobCategory?: string
): Promise<number> {
  return await invoke('db_add_to_bank', {
    question,
    bestAnswer: bestAnswer || null,
    notes: notes || null,
    jobCategory: jobCategory || null
  })
}

export async function getBank(): Promise<QuestionBankItem[]> {
  return await invoke('db_get_bank')
}

export async function updateBankItem(
  id: number,
  bestAnswer?: string,
  notes?: string
): Promise<void> {
  return await invoke('db_update_bank_item', {
    id,
    bestAnswer: bestAnswer || null,
    notes: notes || null
  })
}

export async function deleteFromBank(id: number): Promise<void> {
  return await invoke('db_delete_from_bank', { id })
}
