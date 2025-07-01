import { invoke } from '@tauri-apps/api/core';

interface AssessmentData {
  id: number;
  title: string;
  subject: string;
  status: string;
  due: string;
  code: string;
  metaclassID: number;
  programmeID: number;
  graded: boolean;
  overdue: boolean;
  hasFeedback: boolean;
  expectationsEnabled: boolean;
  expectationsCompleted: boolean;
  reflectionsEnabled: boolean;
  reflectionsCompleted: boolean;
  availability: string;
  finalGrade?: number;
}

interface GradePrediction {
  subject: string;
  predictedGrade: number;
  confidence: number;
  reasoning: string;
}

const GEMINI_API_URL = 'https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent';

export class GeminiService {
  static async getApiKey(): Promise<string | null> {
    try {
      const settings = await invoke<any>('get_settings');
      return settings.gemini_api_key || null;
    } catch {
      return null;
    }
  }

  static async predictGrades(assessments: AssessmentData[]): Promise<GradePrediction[]> {
    const apiKey = await this.getApiKey();
    if (!apiKey) {
      throw new Error('No Gemini API key set. Please add your API key in Settings.');
    }
    try {
      // Group assessments by subject
      const assessmentsBySubject = new Map<string, AssessmentData[]>();
      
      assessments.forEach(assessment => {
        if (!assessmentsBySubject.has(assessment.subject)) {
          assessmentsBySubject.set(assessment.subject, []);
        }
        assessmentsBySubject.get(assessment.subject)!.push(assessment);
      });

      const predictions: GradePrediction[] = [];

      for (const [subject, subjectAssessments] of assessmentsBySubject) {
        // Filter for completed assessments with grades
        const completedAssessments = subjectAssessments.filter(a => 
          a.status === 'MARKS_RELEASED' && a.finalGrade !== undefined
        );

        if (completedAssessments.length === 0) {
          // No completed assessments, skip prediction
          continue;
        }

        // Prepare data for the AI
        const assessmentData = completedAssessments.map(a => ({
          title: a.title,
          grade: a.finalGrade,
          dueDate: a.due,
          status: a.status
        }));

        const prompt = this.buildPredictionPrompt(subject, assessmentData);
        
        const prediction = await this.callGeminiAPI(prompt, apiKey);
        if (prediction) {
          predictions.push(prediction);
        }
      }

      return predictions;
    } catch (error) {
      console.error('Error predicting grades:', error);
      throw new Error('Failed to generate grade predictions');
    }
  }

  private static buildPredictionPrompt(subject: string, assessments: any[]): string {
    const assessmentList = assessments.map(a => 
      `- ${a.title}: ${a.grade}% (due: ${new Date(a.due).toLocaleDateString()})`
    ).join('\n');

    const averageGrade = assessments.reduce((sum, a) => sum + a.grade, 0) / assessments.length;

    return `You are an AI educational assistant analyzing student performance data. 

Given the following assessment results for ${subject}:

${assessmentList}

Current average: ${averageGrade.toFixed(1)}%

Based on this data, predict the student's final grade for ${subject} this year. Consider:
- Performance trends
- Consistency of grades
- Subject difficulty patterns
- Recent performance improvements or declines

Respond with ONLY a JSON object in this exact format:
{
  "subject": "${subject}",
  "predictedGrade": [number between 0-100],
  "confidence": [number between 0-100 representing confidence level],
  "reasoning": "[brief explanation of prediction]"
}

Be realistic and consider that the prediction should be based on demonstrated performance patterns.`;
  }

  private static async callGeminiAPI(prompt: string, apiKey: string): Promise<GradePrediction | null> {
    try {
      const response = await fetch(`${GEMINI_API_URL}?key=${apiKey}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          contents: [{
            parts: [{
              text: prompt
            }]
          }],
          generationConfig: {
            temperature: 0.3,
            topK: 40,
            topP: 0.95,
            maxOutputTokens: 1024,
          }
        })
      });

      if (!response.ok) {
        throw new Error(`Gemini API error: ${response.status}`);
      }

      const data = await response.json();
      
      if (!data.candidates || !data.candidates[0] || !data.candidates[0].content) {
        throw new Error('Invalid response from Gemini API');
      }

      const responseText = data.candidates[0].content.parts[0].text;
      
      // Try to extract JSON from the response
      const jsonMatch = responseText.match(/\{[\s\S]*\}/);
      if (!jsonMatch) {
        throw new Error('No JSON found in response');
      }

      const prediction = JSON.parse(jsonMatch[0]);
      
      // Validate the prediction format
      if (!prediction.subject || typeof prediction.predictedGrade !== 'number' || 
          typeof prediction.confidence !== 'number' || !prediction.reasoning) {
        throw new Error('Invalid prediction format');
      }

      return prediction as GradePrediction;
    } catch (error) {
      console.error('Error calling Gemini API:', error);
      return null;
    }
  }
} 