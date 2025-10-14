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

	let isTraining = $state(false);
	let stopTrainingFlag = false;

	function resetModel() {
		if (model) {
			tf.dispose(model);
		}
		model = null;
		trainingStatus = 'Not started';
		predictionResult = null;
		trainingProgress = 0;
		finalTrainingStats = null;
		isTraining = false;
		stopTrainingFlag = false;
	}

	function stopTraining() {
		stopTrainingFlag = true;
	}

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
			tf.dispose(pnfModel);
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

	let guessingData = $state('');
	let selectedPlotForCopy = $state(0);
	let decimalPlaces = $state(2);
	let roundingStrategy = $state('round');

	function copyDataForGuessing() {
		const plot = plots[selectedPlotForCopy];
		if (!plot || !plot.rawData || plot.rawData.length === 0) {
			guessingData = 'No data to copy.';
			return;
		}

		const multiplier = Math.pow(10, decimalPlaces);
		/** @type {(num: number) => number} */
		let roundingFunction;
		switch (roundingStrategy) {
			case 'floor':
				roundingFunction = (num) => Math.floor(num * multiplier) / multiplier;
				break;
			case 'ceil':
				roundingFunction = (num) => Math.ceil(num * multiplier) / multiplier;
				break;
			case 'round':
			default:
				roundingFunction = (num) => Math.round(num * multiplier) / multiplier;
				break;
		}

		const formattedData = plot.rawData.map(roundingFunction);
		guessingData = formattedData.join(', ');
	}

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
			X.push(Array.from(data.slice(i, i + timeStep)));
			y.push(data[i + timeStep]);
		}
		return [X, y];
	}

	function trainModel() {
		isTraining = true;
		stopTrainingFlag = false;
		trainingStatus = 'Training...';
		trainingProgress = 0;
		finalTrainingStats = null;

		setTimeout(async () => {
			try {
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
					isTraining = false;
					return;
				}

				const flatX = allX.flat();
				const tensorX = tf.tensor3d(flatX, [allX.length, TIME_STEP, 1]);
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
							if (stopTrainingFlag) {
								newModel.stopTraining = true;
							}
							if (logs) {
								trainingStatus = `Epoch ${epoch}/${epochs - 1}: loss = ${logs.loss.toFixed(
									4
								)}, acc = ${(logs.clf_output_acc || 0).toFixed(4)}`;
								trainingProgress = epoch + 1;
							}
						}
					}
				});

				if (stopTrainingFlag) {
					trainingStatus = 'Training stopped by user.';
				} else {
					model = newModel;
					trainingStatus = 'Training complete!';
					const lastEpochIndex = history.epoch.length - 1;
					finalTrainingStats = {
						loss: /** @type {number} */ (history.history.loss[lastEpochIndex]),
						acc: /** @type {number} */ (history.history.clf_output_acc[lastEpochIndex]),
						mae: /** @type {number} */ (history.history.reg_output_mae[lastEpochIndex])
					};
				}
			} catch (/** @type {any} */ e) {
				trainingStatus = `Error: ${e.message}`;
			} finally {
				isTraining = false;
			}
		}, 10);
	}

	async function makePrediction() {
		if (!model) {
			predictionResult = 'Model not trained yet.';
			return;
		}

		const inputData = guessingData
			.split(/[,\\n]/)
			.map(s => parseFloat(s.trim()))
			.filter(n => !isNaN(n));

		if (inputData.length < TIME_STEP) {
			predictionResult = `Not enough data for prediction. Need at least ${TIME_STEP} numbers.`;
			return;
		}

		const sequence = inputData.slice(inputData.length - TIME_STEP);
		const inputTensor = tf.tensor(sequence).reshape([1, TIME_STEP, 1]);
		const [pred_clf, pred_reg] = /** @type {tf.Tensor[]} */ (model.predict(inputTensor));

		const predictedIndex = pred_clf.argMax(-1).dataSync()[0];
		const nextValue = pred_reg.dataSync()[0];

		predictionResult = `Predicted Distribution: ${plots[predictedIndex].type}, Next Value: ${nextValue.toFixed(4)}`;
	}

	const PNF_TIME_STEP = 5;
	/** @type {string[]} */
	let pnfVocab = $state([]);
	let pnfModelTraining = $state(false);

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
	textarea {
		width: 100%;
		padding: 8px;
		box-sizing: border-box;
		border: 1px solid #ccc;
		border-radius: 4px;
		margin-bottom: 10px;
	}
</style>

<div class="ml-container">
	<div class="training">
		<h2>I) Training</h2>
		<div class="training-controls">
			<label for="epochs">Epochs:</label>
			<input id="epochs" type="number" bind:value={epochs} disabled={isTraining} />
			<label for="batchSize">Batch Size:</label>
			<input id="batchSize" type="number" bind:value={batchSize} disabled={isTraining} />
		</div>
		<button onclick={resetModel} disabled={isTraining}>Reset</button>
		<button onclick={trainModel} disabled={isTraining}>Train</button>
		<button onclick={stopTraining} disabled={!isTraining}>Stop</button>
		<p>Status: {trainingStatus}</p>
		{#if isTraining}
			<progress value={trainingProgress} max={epochs}></progress>
		{/if}
		{#if finalTrainingStats && !isTraining}
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
		<div class="training-controls">
			<label for="plot-copy-select">Copy from:</label>
			<select id="plot-copy-select" bind:value={selectedPlotForCopy}>
				{#each plots as plot, i}
					<option value={i}>Plot {i + 1} ({plot.type})</option>
				{/each}
			</select>
			<label for="decimal-places">Decimals:</label>
			<input id="decimal-places" type="number" min="0" bind:value={decimalPlaces} style="width: 60px;" />
			<label for="rounding-strategy">Rounding:</label>
			<select id="rounding-strategy" bind:value={roundingStrategy}>
				<option value="round">Round</option>
				<option value="floor">Floor</option>
				<option value="ceil">Ceil</option>
			</select>
			<button onclick={copyDataForGuessing}>Copy Data</button>
		</div>
		<textarea
			bind:value={guessingData}
			rows="5"
			placeholder="Enter time series data here, separated by commas or newlines..."
		></textarea>
		<button onclick={makePrediction}>Guess</button>
		{#if predictionResult}
			<p>{predictionResult}</p>
		{/if}
	</div>

	<div class="training">
		<h2>III) P&F Prediction (LLM-like)</h2>
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
		<button onclick={predictNextPnfBar} disabled={isPnfTraining}>Guess Next P&F Bar</button>
		{#if pnfPredictionResult}
			<p>{pnfPredictionResult}</p>
		{/if}
	</div>
</div>