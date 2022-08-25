export type Term = 'Winter' | 'Spring' | 'Fall';
export type Kind = 'Quiz' | 'Test' | 'Midterm' | 'Final';

export interface Exam {
    year: number;
    term: Term;
    kind: Kind;
    exam_url: string;
    solution_url: string;
    source: string;
}

export async function fetchExams(course: string): Promise<Exam[]> {
    const response = await fetch(
        `/api/v1/get?course=${encodeURIComponent(course)}`,
        { headers: { Accept: 'application/json' } },
    );
    if (!response.ok) {
        throw Error(`${response.status} (${response.statusText})`);
    }
    const json = await response.json();
    return json as Exam[];
}
