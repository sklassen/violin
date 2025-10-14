<script>
	import { onMount } from 'svelte';
	import init from '$lib/vio-pkg/vio.js';
	import InteractivePlot from '$lib/InteractivePlot.svelte';
	import TimeSeriesModel from '$lib/TimeSeriesModel.svelte';
	import PointAndFigureModel from '$lib/PointAndFigureModel.svelte';

	/** @type {Error | null} */
	let error = $state(null);
	let initialized = $state(false);
	let seed = $state(Math.floor(Math.random() * 1000000));

	const distributionTypes = ['uniform', 'normal', 'skewed', 'bimodal', 'fractal'];

	const defaultParams = {
		uniform: { min: -2, max: 2, ar_coeff: 0.0 },
		normal: { mean: 0, std_dev: 1, ar_coeff: 0.0 },
		skewed: { mean: 0, std_dev: 1, skew: 0.05, ar_coeff: 0.0 },
		bimodal: { mean1: -3, std_dev1: 1, mean2: 3, std_dev2: 1, weight: 0.5, ar_coeff: 0.0 },
		fractal: { hurst: 0.7 }
	};

	let plots = $state(/**
	 * @type {{
	 *   id: number;
	 *   type: string;
	 *   params: any;
	 *   rawData: number[];
	 *   pnfData: { from: number; to: number; direction: 'Up' | 'Down' }[];
	 * }[]}
	 */ ([
		{ id: 1, type: 'normal', params: { ...defaultParams.normal }, rawData: /** @type {number[]} */ ([]), pnfData: /** @type {{ from: number; to: number; direction: 'Up' | 'Down' }[]} */ ([]) },
		{ id: 2, type: 'bimodal', params: { ...defaultParams.bimodal }, rawData: /** @type {number[]} */ ([]), pnfData: /** @type {{ from: number; to: number; direction: 'Up' | 'Down' }[]} */ ([]) },
		{ id: 3, type: 'fractal', params: { ...defaultParams.fractal }, rawData: /** @type {number[]} */ ([]), pnfData: /** @type {{ from: number; to: number; direction: 'Up' | 'Down' }[]} */ ([]) }
	]));

	/**
	 * @param {number} index
	 * @param {string} newType
	 */
	function changePlotType(index, newType) {
		plots[index].type = newType;
		plots[index].params = { ...defaultParams[/** @type {keyof typeof defaultParams} */ (newType)] };
		plots[index].rawData = [];
		plots[index].pnfData = [];
	}

	function randomizeSeed() {
		seed = Math.floor(Math.random() * 1000000);
	}

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
	.global-controls {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 20px;
		padding: 20px;
		border-bottom: 1px solid #ccc;
	}
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
	.plot-selector {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
		border: 1px solid #eee;
		padding: 15px;
		border-radius: 8px;
	}
</style>

<a href="/assets/user_guide.html" class="user-guide-link">View User Guide</a>
<h1>Interactive Svelte + Rust (WASM) Violin Plot Demo</h1>

{#if error}
	<p style="color: red;">Error: {error.message}</p>
{:else if !initialized}
	<p>Initializing WASM module...</p>
{:else}
	<div class="global-controls">
		<label for="seed">Seed:</label>
		<input type="number" id="seed" bind:value={seed} />
		<button onclick={randomizeSeed}>Random</button>
	</div>
	<main>
		{#each plots as plot, i (plot.id)}
			<div class="plot-selector">
				<select
					onchange={(/** @type {Event & { currentTarget: HTMLSelectElement }} */ e) => changePlotType(i, e.currentTarget.value)}
					value={plot.type}
				>
					{#each distributionTypes as type}
						<option value={type}>{type.charAt(0).toUpperCase() + type.slice(1)}</option>
					{/each}
				</select>
				<InteractivePlot
					type={plot.type}
					initialParams={plot.params}
					{seed}
					bind:rawData={plot.rawData}
					bind:pnfData={plot.pnfData}
				/>
			</div>
		{/each}
	</main>
	<TimeSeriesModel {plots} />
	<PointAndFigureModel {plots} />
{/if}