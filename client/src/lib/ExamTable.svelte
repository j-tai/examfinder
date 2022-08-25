<script lang="ts">
    import type { Exam } from './api';

    export let exams: Exam[];
</script>

<table>
    <thead>
        <tr>
            <th>Year</th>
            <th>Term</th>
            <th>Type</th>
            <th>Exam</th>
            <th>Solution</th>
            <th>Source</th>
        </tr>
    </thead>
    <tbody>
        {#each exams as exam}
            <tr>
                <td class="year">{exam.year}</td>
                <td class="term">{exam.term}</td>
                <td class="kind">{exam.kind}</td>
                <td class="exam">
                    {#if exam.exam_url}
                        <a href={exam.exam_url} target="_blank">Exam</a>
                    {:else}
                        -
                    {/if}
                </td>
                <td class="solution">
                    {#if exam.solution_url}
                        <a href={exam.solution_url} target="_blank">Solution</a>
                    {:else}
                        -
                    {/if}
                </td>
                <td class="source">{exam.source}</td>
            </tr>
        {/each}
    </tbody>
</table>

<style>
    tr,
    td,
    th {
        border: none;
    }

    table {
        width: 100%;
        max-width: 600px;
        margin: 0 auto;
        border-collapse: collapse;
    }

    td,
    th {
        text-align: center;
        padding: 0.1em 0.3em;
        margin: 0;
    }

    th {
        border-bottom: 1px solid #777;
    }

    tbody tr:hover {
        background-color: #e6e6e6;
    }
</style>
