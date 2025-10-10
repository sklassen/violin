<script>
	import { onMount } from 'svelte';
	import init from '$lib/vio-pkg/vio.js';
	import InteractivePlot from '$lib/InteractivePlot.svelte';

	let error = $state(null);
	let initialized = $state(false);

	// Initial parameters for each plot type
	const uniformParams = { min: -2, max: 2, ar_coeff: 0.0 };
	const normalParams = { mean: 0, std_dev: 1, ar_coeff: 0.0 };
	const skewedParams = { mean: 0, std_dev: 1, skew: 5, ar_coeff: 0.0 };
	const bimodalParams = { mean1: -3, std_dev1: 1, mean2: 3, std_dev2: 1, weight: 0.5, ar_coeff: 0.0 };
	const fractalParams = { hurst: 0.7 };

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
	.user-guide-link {
		display: block;
		text-align: center;
		margin: 10px 0;
	}
</style>

<a href="/assets/user_guide.html" class="user-guide-link">View User Guide</a>
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
		<InteractivePlot type="fractal" initialParams={fractalParams} />
	</main>
{/if}