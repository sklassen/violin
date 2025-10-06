<script>
	import { onMount } from 'svelte';
	import init, { calculate_violin_data, generate_test_data } from '$lib/vio-pkg/vio.js';

	/** @type {Object.<string, any>} */
	let results = {};
	/** @type {Error | null} */
	let error = null;
	let initialized = false;

	onMount(async () => {
		try {
			console.log('WASM module initializing...');
			await init();
			console.log('WASM module initialized.');

			const testData = generate_test_data();
			console.log('Generated Test Data:', testData);

			/** @type {Object.<string, any>} */
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
			initialized = true;
			console.log('Violin Plot Calculation Results:', results);
		} catch (/** @type {any} */ e) {
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