<script>
	import * as tf from '@tensorflow/tfjs';

	/**
	 * @typedef {{
	 *   id: number;
	 *   type: string;
	 *   params: any;
	 *   rawData: number[];
	 *   pnfData: { from: number; to: number; direction: 'Up' | 'Down' }[];
	 * }[]} Plots
	 */

	/** @type {{ plots: Plots }} */
	let { plots } = $props();

	/** @type {tf.LayersModel | null} */
	let pnfModel = $state(null);
	let pnfTrainingStatus = $state('Not started');
	/** @type {string | null} */
	let pnfPredictionResult = $state(null);

	let pnfEpochs = $state(100);
	let pnfBatchSize = $state(16);
	let pnfTrainingProgress = $state(0);
	let isPnfTraining = $state(false);
	let stopPnfTrainingFlag = false;

	function resetPnfModel() {
		if (pnfModel) {
			tf.dispose(/** @type {any} */ (pnfModel));
		}
		pnfModel = null;
		pnfTrainingStatus = 'Not started';
		pnfPredictionResult = null;
		pnfTrainingProgress = 0;
		isPnfTraining = false;
		stopPnfTrainingFlag = false;
	}

	function stopPnfTraining() {
		stopPnfTrainingFlag = true;
	}

	const PNF_TIME_STEP = 5;
	/** @type {string[]} */
	let pnfVocab = $state([]);
	let selectedPlotIndex = $state(0);

	/**
	 * @param {{ from: number; to: number; direction: 'Up' | 'Down' }} pnf
	 * @returns {string}
	 */
	function pnfToString(pnf) {
		return `${pnf.direction[0]}(${pnf.from.toFixed(1)},${pnf.to.toFixed(1)})`;
	}

	async function trainPnfModel() {
		isPnfTraining = true;
		stopPnfTrainingFlag = false;
		pnfTrainingStatus = 'Preparing P&F data...';
		pnfTrainingProgress = 0;

		setTimeout(async () => {
			try {
				const allPnfStrings = plots.flatMap(p => p.pnfData.map(pnfToString));
				pnfVocab = [...new Set(allPnfStrings)];
				const pnfVocabSize = pnfVocab.length;

				const sequences = [];
				const nextBars = [];
				for (const plot of plots) {
					if (plot.pnfData.length > PNF_TIME_STEP) {
						const stringData = plot.pnfData.map(pnfToString);
						for (let i = 0; i < stringData.length - PNF_TIME_STEP; i++) {
							sequences.push(stringData.slice(i, i + PNF_TIME_STEP));
							nextBars.push(stringData[i + PNF_TIME_STEP]);
						}
					}
				}

				if (sequences.length === 0) {
					pnfTrainingStatus = 'Not enough data to train P&F model.';
					isPnfTraining = false;
					return;
				}

				const X = sequences.map(seq => seq.map(bar => pnfVocab.indexOf(bar)));
				const y = nextBars.map(bar => pnfVocab.indexOf(bar));

				const tensorX = tf.tensor2d(X, [sequences.length, PNF_TIME_STEP]);
				const tensorY = tf.oneHot(tf.tensor1d(y, 'int32'), pnfVocabSize);

				const input = tf.input({ shape: [PNF_TIME_STEP] });
				const embedding = tf.layers.embedding({ inputDim: pnfVocabSize, outputDim: 32 }).apply(input);
				const lstm = /** @type {tf.SymbolicTensor} */ (
					tf.layers.lstm({ units: 32, returnSequences: false }).apply(embedding)
				);
				const output = tf.layers.dense({ units: pnfVocabSize, activation: 'softmax' }).apply(lstm);

				const newPnfModel = tf.model({
					inputs: input,
					outputs: /** @type {tf.SymbolicTensor} */ (output)
				});
				newPnfModel.compile({ loss: 'categoricalCrossentropy', optimizer: 'adam', metrics: ['accuracy'] });

				pnfTrainingStatus = 'Training P&F model...';
				await newPnfModel.fit(tensorX, tensorY, {
					epochs: pnfEpochs,
					batchSize: pnfBatchSize,
					callbacks: {
						onEpochEnd: (epoch, logs) => {
							if (stopPnfTrainingFlag) {
								newPnfModel.stopTraining = true;
							}
							if (logs) {
								pnfTrainingStatus = `Epoch ${epoch}/${pnfEpochs - 1}: loss = ${logs.loss.toFixed(
									4
								)}, acc = ${logs.acc}`;
								pnfTrainingProgress = epoch + 1;
							}
						}
					}
				});

				if (stopPnfTrainingFlag) {
					pnfTrainingStatus = 'P&F Model training stopped by user.';
				} else {
					pnfModel = newPnfModel;
					pnfTrainingStatus = 'P&F Model training complete!';
				}
			} catch (/** @type {any} */ e) {
				pnfTrainingStatus = `Error: ${e.message}`;
			} finally {
				isPnfTraining = false;
			}
		}, 10);
	}

	async function predictNextPnfBar() {
		if (!pnfModel) {
			pnfPredictionResult = 'P&F model not trained yet.';
			return;
		}

		const plot = plots[selectedPlotIndex];
		if (plot.pnfData.length < PNF_TIME_STEP) {
			pnfPredictionResult = 'Not enough P&F data for prediction.';
			return;
		}

		const sequence = plot.pnfData.slice(plot.pnfData.length - PNF_TIME_STEP).map(pnfToString);
		const inputX = sequence.map(bar => pnfVocab.indexOf(bar));
		const inputTensor = tf.tensor2d([inputX], [1, PNF_TIME_STEP]);
		const prediction = /** @type {tf.Tensor} */ (pnfModel.predict(inputTensor));
		const predictedIndex = await prediction.argMax(-1).data();
		pnfPredictionResult = `Predicted next P&F bar: ${pnfVocab[predictedIndex[0]]}`;
	}
</script>

<style>
	.ml-container {
		border: 1px solid #ccc;
		padding: 20px;
		margin-top: 20px;
		border-radius: 8px;
	}
	.training {
		margin-bottom: 20px;
	}
	button {
		margin-right: 10px;
	}
	.training-controls {
		display: flex;
		gap: 15px;
		align-items: center;
		margin-bottom: 15px;
	}
	.training-controls input {
		width: 60px;
	}
	progress {
		width: 100%;
		margin-top: 10px;
	}
</style>

<div class="ml-container">
	<div class="training">
		<h2>P&F Prediction (LLM-like)</h2>
		<div class="training-controls">
			<label for="pnfEpochs">Epochs:</label>
			<input id="pnfEpochs" type="number" bind:value={pnfEpochs} disabled={isPnfTraining} />
			<label for="pnfBatchSize">Batch Size:</label>
			<input id="pnfBatchSize" type="number" bind:value={pnfBatchSize} disabled={isPnfTraining} />
		</div>
		<button onclick={resetPnfModel} disabled={isPnfTraining}>Reset</button>
		<button onclick={trainPnfModel} disabled={isPnfTraining}>Train P&F Model</button>
		<button onclick={stopPnfTraining} disabled={!isPnfTraining}>Stop</button>
		<p>Status: {pnfTrainingStatus}</p>
		{#if isPnfTraining}
			<progress value={pnfTrainingProgress} max={pnfEpochs}></progress>
		{/if}
		<div class="training-controls">
			<label for="plot-select-pnf">Select a time series to guess from:</label>
			<select id="plot-select-pnf" bind:value={selectedPlotIndex}>
				{#each plots as plot, i}
					<option value={i}>Plot {i + 1} ({plot.type})</option>
				{/each}
			</select>
			<button onclick={predictNextPnfBar} disabled={isPnfTraining}>Guess Next P&F Bar</button>
		</div>
		{#if pnfPredictionResult}
			<p>{pnfPredictionResult}</p>
		{/if}
	</div>
</div>
