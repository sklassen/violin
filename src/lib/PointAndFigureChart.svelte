<script>
    import * as d3 from 'd3';

    let { data, boxSize, width = 320, height = 200 } = $props();

    let container;

    $effect(() => {
        if (container && data && data.length > 0 && boxSize > 0) {
            const margin = { top: 20, right: 20, bottom: 30, left: 40 };
            const innerWidth = width - margin.left - margin.right;
            const innerHeight = height - margin.top - margin.bottom;

            d3.select(container).select("svg").remove();

            const svg = d3.select(container)
                .append("svg")
                .attr("width", width)
                .attr("height", height)
                .append("g")
                .attr("transform", `translate(${margin.left}, ${margin.top})`);

            const allValues = data.flatMap(d => [d.from, d.to]);
            const yMin = Math.floor(d3.min(allValues) / boxSize) * boxSize;
            const yMax = Math.ceil(d3.max(allValues) / boxSize) * boxSize;

            const y = d3.scaleLinear()
                .domain([yMin, yMax])
                .range([innerHeight, 0]);

            const x = d3.scaleBand()
                .domain(d3.range(data.length))
                .range([0, innerWidth])
                .padding(0.1);

            svg.append("g").call(d3.axisLeft(y).ticks(5));
            svg.append("g")
               .attr("transform", `translate(0, ${innerHeight})`)
               .call(d3.axisBottom(x).tickFormat(i => i + 1));

            const columnWidth = x.bandwidth();

            data.forEach((col, i) => {
                const start = Math.min(col.from, col.to);
                const end = Math.max(col.from, col.to);
                const numBoxes = Math.floor((end - start) / boxSize);

                for (let j = 0; j <= numBoxes; j++) {
                    const yLevel = col.direction === 'Up'
                        ? Math.floor(col.from / boxSize) * boxSize + j * boxSize
                        : Math.ceil(col.from / boxSize) * boxSize - j * boxSize;

                    const symbol = col.direction === 'Up' ? 'X' : 'O';

                    svg.append("text")
                       .attr("x", x(i) + columnWidth / 2)
                       .attr("y", y(yLevel))
                       .attr("text-anchor", "middle")
                       .attr("dominant-baseline", "middle")
                       .style("font-size", `${Math.min(columnWidth, boxSize * 2)}px`)
                       .style("fill", col.direction === 'Up' ? 'green' : 'red')
                       .text(symbol);
                }
            });

            // Title
            svg.append("text")
                .attr("x", innerWidth / 2)
                .attr("y", -5)
                .attr("text-anchor", "middle")
                .style("font-size", "12px")
                .text("Point & Figure Chart");
        }
    });
</script>

<div bind:this={container}></div>