<script>
	import { onMount } from 'svelte';
	import init from '$lib/vio-pkg/vio.js';
	import InteractivePlot from '$lib/InteractivePlot.svelte';

	/** @type {Error | null} */
	let error = null;
	let initialized = false;

	// Initial parameters for each plot type
	const uniformParams = { min: -2, max: 2 };
	const normalParams = { mean: 0, std_dev: 1 };
	const skewedParams = { mean: 0, std_dev: 1, skew: 5 };
	const bimodalParams = { mean1: -3, std_dev1: 1, mean2: 3, std_dev2: 1, weight: 0.5 };

	onMount(async () => {
		try {
			console.log('WASM module initializing...');
			await init();
			console.log('WASM module initialized.');
			initialized = true;
		} catch (/** @type {any} */ e) {
			console.error('Error during WASM execution:', e);
			error = e;
		}
	});
</script>

<style>
	main {
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
		justify-content: center;
		padding: 20px;
	}
	h1 {
		width: 100%;
		text-align: center;
	}
</style>

<h1>Interactive Svelte + Rust (WASM) Violin Plot Demo</h1>

{#if error}
	<p style="color: red;">Error: {error.message}</p>
{:else if !initialized}
	<p>Initializing WASM module...</p>
{:else}
	<main>
		<InteractivePlot type="uniform" initialParams={uniformParams} />
		<InteractivePlot type="normal" initialParams={normalParams} />
		<InteractivePlot type="skewed" initialParams={skewedParams} />
		<InteractivePlot type="bimodal" initialParams={bimodalParams} />
	</main>
{/if}