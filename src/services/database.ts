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
  tags?: QuestionTag[]
}

export interface QuestionTag {
  id?: number
  name: string
  color: string
  created_at: string
}

export interface SessionReport {
  id?: number
  session_id: number
  overall_score: number
  summary: string
  improvements: string
  key_takeaways: string
  reference_answers?: string
  generated_at: string
  api_response_time?: number
  content_analysis?: string
  expression_analysis?: string
}

export interface TrendDataPoint {
  timestamp: number
  overallScore: number
  communicationScore: number
  problemSolvingScore: number
  technicalDepthScore: number
  presentationScore: number
}

export interface PerformanceStatistics {
  totalSessions: number
  averageOverall: number
  highestOverall: number
  improvementRate: number
  recentTrend: 'improving' | 'stable' | 'declining'
}

export interface TrendAnalytics {
  dataPoints: TrendDataPoint[]
  statistics: PerformanceStatistics
}

export interface DashboardStats {
  totalSessions: number
  averageScore: number
  highestScore: number
  recentImprovement: number
}

export interface TopQuestion {
  question: string
  count: number
}

export interface WeakArea {
  area: string
  averageScore: number
  suggestion: string
}

export interface RecentSessionInfo {
  id: number
  createdAt: string
  questionCount: number
  overallScore?: number
}

export interface DashboardData {
  stats: DashboardStats
  topQuestions: TopQuestion[]
  weakAreas: WeakArea[]
  recentSessions: RecentSessionInfo[]
}

// History management types
export interface AnswerComparisonItem {
  timestamp: string
  answer: string
  feedback: string
  score: string
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

/**
 * Analyze answer with content scoring and save to answer_analysis table
 * This enables profile dimension calculations
 */
export async function analyzeAnswerWithScoring(
  answerId: number,
  answer: string,
  question: string,
  jobDescription: string
): Promise<void> {
  await invoke('analyze_answer_with_scoring', {
    answerId,
    answer,
    question,
    jobDescription
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

// Question tag operations
export async function createTag(name: string, color: string): Promise<number> {
  return await invoke('db_create_tag', { name, color })
}

export async function getAllTags(): Promise<QuestionTag[]> {
  return await invoke('db_get_all_tags')
}

export async function updateTag(id: number, name: string, color: string): Promise<void> {
  return await invoke('db_update_tag', { id, name, color })
}

export async function deleteTag(id: number): Promise<void> {
  return await invoke('db_delete_tag', { id })
}

export async function addTagToQuestion(questionId: number, tagId: number): Promise<void> {
  return await invoke('db_add_tag_to_question', { questionId, tagId })
}

export async function removeTagFromQuestion(questionId: number, tagId: number): Promise<void> {
  return await invoke('db_remove_tag_from_question', { questionId, tagId })
}

export async function getTagsForQuestion(questionId: number): Promise<QuestionTag[]> {
  return await invoke('db_get_tags_for_question', { questionId })
}

export async function getQuestionsByTag(tagId: number): Promise<QuestionBankItem[]> {
  return await invoke('db_get_questions_by_tag', { tagId })
}

// Report operations
export async function generateReport(sessionId: number): Promise<SessionReport> {
  return await invoke('generate_comprehensive_report', { sessionId })
}

export async function getReport(sessionId: number): Promise<SessionReport | null> {
  return await invoke('db_get_report', { sessionId })
}

export async function exportReportText(
  sessionId: number,
  filePath: string
): Promise<void> {
  return await invoke('export_report_text', { sessionId, filePath })
}

export async function exportReportHtml(
  sessionId: number,
  filePath: string
): Promise<void> {
  return await invoke('export_report_html', { sessionId, filePath })
}

// Analytics operations
export async function getTrendAnalytics(
  timeRangeDays?: number
): Promise<TrendAnalytics> {
  return await invoke('get_trend_analytics', { timeRangeDays: timeRangeDays || null })
}

// Dashboard operations
export async function getDashboardData(): Promise<DashboardData> {
  return await invoke('get_dashboard_data')
}

export interface ActivityData {
  date: string
  count: number
}

export async function getActivityData(): Promise<ActivityData[]> {
  const results = await invoke<[string, number][]>('get_activity_data')
  return results.map(([date, count]) => ({ date, count }))
}

// History management operations
export async function getAnswersComparison(
  question: string
): Promise<AnswerComparisonItem[]> {
  const results = await invoke<[string, string, string, string][]>(
    'get_answers_comparison',
    { question }
  )
  return results.map(([timestamp, answer, feedback, score]) => ({
    timestamp,
    answer,
    feedback,
    score
  }))
}

export async function deleteSession(sessionId: number): Promise<void> {
  return await invoke('delete_session', { sessionId })
}

export async function deleteAllSessions(): Promise<void> {
  return await invoke('delete_all_sessions')
}

export async function backupData(filePath: string): Promise<void> {
  return await invoke('backup_data', { filePath })
}

export async function restoreData(filePath: string): Promise<void> {
  return await invoke('restore_data', { filePath })
}

// Pagination and filtering operations
export interface PaginationResult<T> {
  data: T[]
  total: number
}

export async function getSessionsPaginated(
  page: number,
  pageSize: number
): Promise<PaginationResult<InterviewSession>> {
  const [data, total] = await invoke<[InterviewSession[], number]>(
    'get_sessions_paginated',
    { page, pageSize }
  )
  return { data, total }
}

export async function getAnswersPaginated(
  sessionId: number,
  page: number,
  pageSize: number
): Promise<PaginationResult<InterviewAnswer>> {
  const [data, total] = await invoke<[InterviewAnswer[], number]>(
    'get_answers_paginated',
    { sessionId, page, pageSize }
  )
  return { data, total }
}

export async function getSessionsByDateRange(
  startDate: string,
  endDate: string
): Promise<InterviewSession[]> {
  return await invoke('get_sessions_by_date_range', { startDate, endDate })
}

export async function getReportsByDateRange(
  startDate: string,
  endDate: string
): Promise<SessionReport[]> {
  return await invoke('get_reports_by_date_range', { startDate, endDate })
}

// Profile operations
export interface ProfileDimension {
  technical_depth: number
  communication: number
  problem_solving: number
  domain_knowledge: number
  adaptability: number
  job_intention: number
}

export interface InterviewProfile {
  user_id: string
  dimensions: ProfileDimension
  total_sessions: number
  average_score: number
  strongest_dimension: string
  weakest_dimension: string
  improvement_suggestions: string[]
  generated_at: string
}

export async function generateInterviewProfile(
  userId: string = 'default_user',
  sessionLimit?: number
): Promise<InterviewProfile> {
  return await invoke('generate_interview_profile', { 
    userId, 
    sessionLimit: sessionLimit || null 
  })
}

// Recommendation operations
export interface PracticeRecommendation {
  question_id: number
  question: string
  reason: string
  priority: number
  dimension: string
  estimated_improvement: number
}

export interface RecommendationResult {
  recommendations: PracticeRecommendation[]
  weak_dimensions: string[]
  total_available: number
  generated_at: string
}

export async function generatePracticeRecommendations(
  userId: string = 'default_user',
  limit: number = 5
): Promise<RecommendationResult> {
  return await invoke('generate_practice_recommendations', { userId, limit })
}

// Best practices operations
export interface BestPractice {
  question: string
  answer: string
  score: number
  session_id: number
  extracted_at: string
  key_points: string[]
}

export interface BestPracticesResult {
  practices: BestPractice[]
  total_analyzed: number
  threshold_score: number
  generated_at: string
}

export async function extractBestPractices(
  scoreThreshold: number = 7.5,
  limit: number = 10
): Promise<BestPracticesResult> {
  return await invoke('extract_best_practices', { scoreThreshold, limit })
}

// Industry comparison operations
export interface IndustryBenchmark {
  dimension: string
  user_score: number
  industry_avg: number
  industry_top: number
  percentile: number
}

export interface IndustryComparisonResult {
  benchmarks: IndustryBenchmark[]
  overall_percentile: number
  user_level: string
  comparison_count: number
  generated_at: string
}

export async function generateIndustryComparison(
  userId: string = 'default_user'
): Promise<IndustryComparisonResult> {
  return await invoke('generate_industry_comparison', { userId })
}

// STAR scoring
export interface STARScoreBreakdown {
  situation: number
  task: number
  action: number
  result: number
}

export interface STARScoringResult {
  overall_score: number
  breakdown: STARScoreBreakdown
  completeness: number
  suggestions: string[]
}

export async function analyzeSTARScore(
  answer: string
): Promise<STARScoringResult> {
  const resultJson = await invoke<string>('analyze_star_score', { answer })
  return JSON.parse(resultJson)
}

// User management types and operations
export interface User {
  id?: number
  username: string
  avatar_color: string
  created_at: string
}

export async function createUser(
  username: string,
  avatarColor: string
): Promise<number> {
  return await invoke('create_user', { username, avatarColor })
}

export async function getAllUsers(): Promise<User[]> {
  return await invoke('get_all_users')
}

export async function getCurrentUser(): Promise<User | null> {
  return await invoke('get_current_user')
}

export async function switchUser(userId: number): Promise<void> {
  return await invoke('switch_user', { userId })
}

export async function updateUser(
  id: number,
  username: string,
  avatarColor: string
): Promise<void> {
  return await invoke('update_user', { id, username, avatarColor })
}

export async function deleteUser(id: number): Promise<void> {
  return await invoke('delete_user', { id })
}
