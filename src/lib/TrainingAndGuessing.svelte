<script>
	import * as tf from '@tensorflow/tfjs';

	/**
	 * @typedef {{
	 *   id: number;
	 *   type: string;
	 *   params: any;
	 *   rawData: number[];
	 * }[]} Plots
	 */

	/** @type {{ plots: Plots }} */
	let { plots } = $props();
	/** @type {tf.LayersModel | null} */
	let model = $state(null);
	let trainingStatus = $state('Not started');
	/** @type {string | null} */
	let predictionResult = $state(null);
	let selectedPlotIndex = $state(0);
	let epochs = $state(20);
	let batchSize = $state(32);
	let trainingProgress = $state(0);
	/** @type {{ loss: number; acc: number; mae: number } | null} */
	let finalTrainingStats = $state(null);

	const N_SAMPLES = 300;
	const TIME_STEP = 20;

	/**
	 * @param {number[] | Float64Array} data
	 * @param {number} timeStep
	 * @returns {[number[][], number[]]}
	 */
	function createSequences(data, timeStep) {
		const X = [];
		const y = [];
		for (let i = 0; i < data.length - timeStep; i++) {
			X.push(data.slice(i, i + timeStep));
			y.push(data[i + timeStep]);
		}
		return [X, y];
	}

	async function trainModel() {
		trainingStatus = 'Preparing data...';
		trainingProgress = 0;
		finalTrainingStats = null;

		/** @type {number[][]} */
		const allX = [];
		/** @type {number[]} */
		const allY_reg = [];
		/** @type {number[]} */
		const allY_clf = [];

		plots.forEach((plot, index) => {
			if (plot.rawData.length > 0) {
				const [X, y_reg] = createSequences(plot.rawData, TIME_STEP);
				const y_clf = Array(X.length).fill(index);
				allX.push(...X);
				allY_reg.push(...y_reg);
				allY_clf.push(...y_clf);
			}
		});

		if (allX.length === 0) {
			trainingStatus = 'Error: No data to train on.';
			return;
		}

		trainingStatus = 'Training model...';

		const tensorX = tf.tensor3d(allX, [allX.length, TIME_STEP, 1]);
		const tensorY_reg = tf.tensor2d(allY_reg, [allY_reg.length, 1]);
		const tensorY_clf = tf.oneHot(tf.tensor1d(allY_clf, 'int32'), plots.length);

		const input = tf.input({ shape: [TIME_STEP, 1] });
		const lstm = /** @type {tf.SymbolicTensor} */ (
			tf.layers.lstm({ units: 32, returnSequences: false }).apply(input)
		);

		const clf_output = tf.layers
			.dense({ units: plots.length, activation: 'softmax', name: 'clf_output' })
			.apply(lstm);
		const reg_output = tf.layers.dense({ units: 1, name: 'reg_output' }).apply(lstm);

		const newModel = tf.model({
			inputs: input,
			outputs: [
				/** @type {tf.SymbolicTensor} */ (clf_output),
				/** @type {tf.SymbolicTensor} */ (reg_output)
			]
		});

		newModel.compile({
			optimizer: 'adam',
			loss: { clf_output: 'categoricalCrossentropy', reg_output: 'meanSquaredError' },
			metrics: { clf_output: 'accuracy', reg_output: 'mae' }
		});

		const history = await newModel.fit(tensorX, [tensorY_clf, tensorY_reg], {
			epochs: epochs,
			batchSize: batchSize,
			callbacks: {
				onEpochEnd: (epoch, logs) => {
					if (logs) {
						trainingStatus = `Epoch ${epoch + 1}/${epochs}: loss = ${logs.loss.toFixed(
							4
						)}, acc = ${(logs.clf_output_acc || 0).toFixed(4)}`;
						trainingProgress = epoch + 1;
					}
				}
			}
		});

		model = newModel;
		trainingStatus = 'Training complete!';
		const lastEpochIndex = history.epoch.length - 1;
		finalTrainingStats = {
			loss: /** @type {number} */ (history.history.loss[lastEpochIndex]),
			acc: /** @type {number} */ (history.history.clf_output_acc[lastEpochIndex]),
			mae: /** @type {number} */ (history.history.reg_output_mae[lastEpochIndex])
		};
	}

	async function makePrediction() {
		if (!model) {
			predictionResult = 'Model not trained yet.';
			return;
		}

		const plot = plots[selectedPlotIndex];
		if (plot.rawData.length < TIME_STEP) {
			predictionResult = 'Not enough data for prediction.';
			return;
		}

		const sequence = plot.rawData.slice(plot.rawData.length - TIME_STEP);
		const inputTensor = tf.tensor3d([sequence], [1, TIME_STEP, 1]);
		const [pred_clf, pred_reg] = /** @type {tf.Tensor[]} */ (model.predict(inputTensor));

		const predictedIndex = pred_clf.argMax(-1).dataSync()[0];
		const nextValue = pred_reg.dataSync()[0];

		predictionResult = `Predicted Distribution: ${plots[predictedIndex].type}, Next Value: ${nextValue.toFixed(4)}`;
	}
</script>

<style>
	.ml-container {
		border: 1px solid #ccc;
		padding: 20px;
		margin-top: 20px;
		border-radius: 8px;
	}
	.training,
	.guessing {
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
	.stats {
		margin-top: 15px;
		border: 1px solid #ddd;
		padding: 10px;
		border-radius: 4px;
	}
</style>

<div class="ml-container">
	<div class="training">
		<h2>I) Training</h2>
		<div class="training-controls">
			<label for="epochs">Epochs:</label>
			<input id="epochs" type="number" bind:value={epochs} />
			<label for="batchSize">Batch Size:</label>
			<input id="batchSize" type="number" bind:value={batchSize} />
		</div>
		<button onclick={trainModel}>Train Model</button>
		<p>Status: {trainingStatus}</p>
		{#if trainingProgress > 0}
			<progress value={trainingProgress} max={epochs}></progress>
		{/if}
		{#if finalTrainingStats}
			<div class="stats">
				<h3>Final Training Stats</h3>
				<p>Loss: {finalTrainingStats.loss.toFixed(4)}</p>
				<p>Accuracy: {finalTrainingStats.acc.toFixed(4)}</p>
				<p>MAE: {finalTrainingStats.mae.toFixed(4)}</p>
			</div>
		{/if}
	</div>

	<div class="guessing">
		<h2>II) Guessing</h2>
		<label for="plot-select">Select a time series to guess:</label>
		<select id="plot-select" bind:value={selectedPlotIndex}>
			{#each plots as plot, i}
				<option value={i}>Plot {i + 1} ({plot.type})</option>
			{/each}
		</select>
		<button onclick={makePrediction}>Guess</button>
		{#if predictionResult}
			<p>{predictionResult}</p>
		{/if}
	</div>
</div>