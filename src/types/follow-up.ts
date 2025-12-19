/**
 * Follow-up question types and interfaces
 */

export type FollowUpType = 
  | 'clarification'    // Clarify unclear points
  | 'deepening'        // Deepen technical details
  | 'scenario'         // Scenario-based questions
  | 'challenge'        // Challenge assumptions
  | 'extension'        // Extend to related topics

export interface ConversationTurn {
  role: 'interviewer' | 'candidate'
  content: string
  timestamp: number
  questionType?: FollowUpType
}

export interface FollowUpContext {
  originalQuestion: string
  conversationHistory: ConversationTurn[]
  currentAnswer: string
  jobDescription: string
  personaId?: string
}

export interface FollowUpQuestion {
  question: string
  type: FollowUpType
  reason: string  // Why this follow-up is asked
  context: string // Brief context for the question
}

export interface FollowUpAnalysis {
  shouldFollowUp: boolean
  followUpQuestions: FollowUpQuestion[]
  reasoning: string
  answerQuality: 'excellent' | 'good' | 'acceptable' | 'poor'
}

export interface FollowUpSettings {
  enabled: boolean
  maxFollowUps: number           // Max follow-ups per question (1-3)
  autoTrigger: boolean           // Auto trigger or manual
  triggerThreshold: number       // Quality threshold to trigger (1-5)
  preferredTypes: FollowUpType[] // Preferred follow-up types
}

export const DEFAULT_FOLLOWUP_SETTINGS: FollowUpSettings = {
  enabled: true,
  maxFollowUps: 2,
  autoTrigger: true,
  triggerThreshold: 3,
  preferredTypes: ['clarification', 'deepening', 'scenario']
}

export const FOLLOWUP_TYPE_LABELS: Record<FollowUpType, string> = {
  clarification: '澄清细节',
  deepening: '深入技术',
  scenario: '场景应用',
  challenge: '挑战假设',
  extension: '扩展延伸'
}

export const FOLLOWUP_TYPE_DESCRIPTIONS: Record<FollowUpType, string> = {
  clarification: '针对回答中不清楚的部分进行追问',
  deepening: '深入挖掘技术实现细节和原理',
  scenario: '提出实际场景让候选人应用知识',
  challenge: '质疑回答中的观点或方案',
  extension: '延伸到相关技术或更深层次的话题'
}
