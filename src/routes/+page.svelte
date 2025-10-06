<script lang="ts">
	import { onMount } from 'svelte';
	import init, { generate_test_data, calculate_violin_data } from 'wasm-lib';

	let results = {};
	let error = null;
	let initialized = false;

	onMount(async () => {
		try {
			await init();
			initialized = true;
			console.log('WASM module initialized.');

			const testData = generate_test_data();
			console.log('Generated Test Data:', testData);

			const calculatedResults = {};
			for (const key in testData) {
				if (Object.prototype.hasOwnProperty.call(testData, key)) {
					const data = testData[key];
					if (data && data.length > 0) {
						console.log(`Calculating violin data for ${key}...`);
						calculatedResults[key] = calculate_violin_data(data);
					} else {
						console.log(`Skipping ${key} as it has no data.`);
						calculatedResults[key] = { error: 'No data provided' };
					}
				}
			}
			results = calculatedResults;
			console.log('Violin Plot Calculation Results:', results);
		} catch (e) {
			console.error('Error during WASM execution:', e);
			error = e;
		}
	});
</script>

<h1>Svelte + Rust (WASM) Violin Plot Demo</h1>

{#if error}
	<p style="color: red;">Error: {error.message}</p>
{:else if !initialized}
	<p>Initializing WASM module...</p>
{:else}
	<p>WASM module loaded and calculations complete. See console for details.</p>
	<h2>Calculation Results:</h2>
	<pre>{JSON.stringify(results, null, 2)}</pre>
{/if}