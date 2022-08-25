<script lang="ts">
    import { fade, fly } from 'svelte/transition';
    import { fetchExams, type Exam } from './lib/api';
    import ExamTable from './lib/ExamTable.svelte';

    /// The current value of the text box
    let course = '';
    /// The course that was last searched for
    let courseSearched = '';
    /// The results of the search
    let results: Promise<Exam[]> = Promise.resolve([]);

    function search() {
        course = course.replaceAll(/[- ]/g, '').toUpperCase();
        courseSearched = course;
        if (courseSearched) {
            results = fetchExams(courseSearched);
        } else {
            results = Promise.resolve([]);
        }
    }
</script>

<main>
    <h1>Exam Finder</h1>
    <p>Find past exams for University of Waterloo courses.</p>
    <p>
        This website aggregates exams from the
        <a href="https://mathsoc.uwaterloo.ca/exam-bank/">MathSoc exam bank</a>
        and the
        <a href="https://services.mathsoc.uwaterloo.ca/university/exambank"
            >MathSoc Services exam bank</a
        >.
    </p>

    <form on:submit|preventDefault={search}>
        <label for="course">Course code:</label>
        <input
            name="course"
            type="text"
            placeholder="CS240"
            spellcheck="false"
            bind:value={course}
        />
        <input type="submit" value="Search" />
    </form>

    {#if courseSearched}
        {#await results}
            <h2>Loading...</h2>
        {:then exams}
            {#if exams.length}
                <div in:fly={{ y: -20, duration: 400 }}>
                    <h2>Exams for {courseSearched}</h2>
                    <ExamTable {exams} />
                </div>
            {:else}
                <div in:fly={{ y: -20, duration: 400 }}>
                    <h2>No results found &#x1f625;</h2>
                    <p class="error">
                        Try looking for exams from similar courses, or try the
                        <a href="https://exams.engsoc.uwaterloo.ca/"
                            >EngSoc exam bank</a
                        >.
                    </p>
                </div>
            {/if}
        {:catch error}
            <h2>Error loading exams &#x1f625;</h2>
            <p class="error">{error}</p>
        {/await}
    {/if}
</main>

<style>
    input[type='text'] {
        width: 8em;
    }

    input {
        padding: 3px 6px;
    }

    form,
    h2,
    .error {
        text-align: center;
    }

    h2 {
        margin-top: 1.2em;
    }
</style>
