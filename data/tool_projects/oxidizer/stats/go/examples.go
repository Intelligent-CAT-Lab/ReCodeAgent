package stats

func Example() {

	// d := LoadRawData([]interface{}{1.1, "2", 3.0, 4, "5"})
	// d := LoadRawData([]int{1, 2, 3, 4, 5})
	var d Float64Data
	for _, v := range []int{1, 2, 3, 4, 5} {
		d = append(d, float64(v))
	}

	_, _ = Min(d)
	// Output: 1.1

	_, _ = Max(d)
	// Output: 5

	_, _ = Sum([]float64{1.1, 2.2, 3.3})
	// Output: 6.6

	_, _ = CumulativeSum([]float64{1.1, 2.2, 3.3})

	_, _ = Mean([]float64{1, 2, 3, 4, 5})
	// Output: 3

	_, _ = Median([]float64{1, 2, 3, 4, 5, 6, 7})
	// Output: 4

	_, _ = Mode([]float64{5, 5, 3, 3, 4, 2, 1})
	// Output: [5 3]

	_, _ = PopulationVariance([]float64{1, 2, 3, 4, 5})
	// Output: 2

	_, _ = SampleVariance([]float64{1, 2, 3, 4, 5})
	// Output: 2.5

	_, _ = MedianAbsoluteDeviationPopulation([]float64{1, 2, 3})
	// Output: 1

	_, _ = StandardDeviationPopulation([]float64{1, 2, 3})
	// Output: 0.816496580927726

	_, _ = StandardDeviationSample([]float64{1, 2, 3})
	// Output: 1

	_, _ = Percentile([]float64{1, 2, 3, 4, 5}, 75)
	// Output: 4

	_, _ = PercentileNearestRank([]float64{35, 20, 15, 40, 50}, 75)
	// Output: 40

	c := []Coordinate{
		{1, 2.3},
		{2, 3.3},
		{3, 3.7},
		{4, 4.3},
		{5, 5.3},
	}

	_, _ = LinearRegression(c)
	// Output: [{1 2.3800000000000026} {2 3.0800000000000014} {3 3.7800000000000002} {4 4.479999999999999} {5 5.179999999999998}]

	_, _ = ExponentialRegression(c)
	// Output: [{1 2.5150181024736638} {2 3.032084111136781} {3 3.6554544271334493} {4 4.406984298281804} {5 5.313022222665875}]

	_, _ = LogarithmicRegression(c)
	// Output: [{1 2.1520822363811702} {2 3.3305559222492214} {3 4.019918836568674} {4 4.509029608117273} {5 4.888413396683663}]

	_, _ = Sample([]float64{0.1, 0.2, 0.3, 0.4}, 3, false)
	// Output: [0.2,0.4,0.3]

	_, _ = Sample([]float64{0.1, 0.2, 0.3, 0.4}, 10, true)
	// Output: [0.2,0.2,0.4,0.1,0.2,0.4,0.3,0.2,0.2,0.1]

	_, _ = Quartile([]float64{7, 15, 36, 39, 40, 41})
	// Output: {15 37.5 40}

	_, _ = InterQuartileRange([]float64{102, 104, 105, 107, 108, 109, 110, 112, 115, 116, 118})
	// Output: 10

	_, _ = Midhinge([]float64{1, 3, 4, 4, 6, 6, 6, 6, 7, 7, 7, 8, 8, 9, 9, 10, 11, 12, 13})
	// Output: 7.5

	_, _ = Trimean([]float64{1, 3, 4, 4, 6, 6, 6, 6, 7, 7, 7, 8, 8, 9, 9, 10, 11, 12, 13})
	// Output: 7.25

	_, _ = QuartileOutliers([]float64{-1000, 1, 3, 4, 4, 6, 6, 6, 6, 7, 8, 15, 18, 100})
	// Output:  {Mild:[15 18] Extreme:[-1000 100]}

	_, _ = GeometricMean([]float64{10, 51.2, 8})
	// Output: 15.999999999999991

	_, _ = HarmonicMean([]float64{1, 2, 3, 4, 5})
	// Output: 2.18978102189781

	_, _ = Round(2.18978102189781, 3)
	// Output: 2.189

	_, _ = ChebyshevDistance([]float64{2, 3, 4, 5, 6, 7, 8}, []float64{8, 7, 6, 5, 4, 3, 2})
	// Output: 6

	_, _ = ManhattanDistance([]float64{2, 3, 4, 5, 6, 7, 8}, []float64{8, 7, 6, 5, 4, 3, 2})
	// Output: 24

	_, _ = EuclideanDistance([]float64{2, 3, 4, 5, 6, 7, 8}, []float64{8, 7, 6, 5, 4, 3, 2})
	// Output: 10.583005244258363

	_, _ = MinkowskiDistance([]float64{2, 3, 4, 5, 6, 7, 8}, []float64{8, 7, 6, 5, 4, 3, 2}, float64(1))
	// Output: 24

	_, _ = MinkowskiDistance([]float64{2, 3, 4, 5, 6, 7, 8}, []float64{8, 7, 6, 5, 4, 3, 2}, float64(2))
	// Output: 10.583005244258363

	_, _ = MinkowskiDistance([]float64{2, 3, 4, 5, 6, 7, 8}, []float64{8, 7, 6, 5, 4, 3, 2}, float64(99))
	// Output: 6

	_, _ = Correlation([]float64{1, 2, 3, 4, 5}, []float64{1, 2, 3, 5, 6})
	// Output: 0.9912407071619302

	_, _ = AutoCorrelation([]float64{1, 2, 3, 4, 5}, 1)
	// Output: 0.4

	_, _ = Sigmoid([]float64{3.0, 1.0, 2.1})
	// Output: [0.9525741268224334 0.7310585786300049 0.8909031788043871]

	_, _ = SoftMax([]float64{3.0, 1.0, 0.2})
	// Output: [0.8360188027814407 0.11314284146556013 0.05083835575299916]

	_, _ = Entropy([]float64{1.1, 2.2, 3.3})
	// Output: 1.0114042647073518

	p := 0.5
	begin := 1
	end := 2
	_, _ = ProbGeom(begin, end, p)
	// Output: 0.25

	prob1 := 0.5
	_, _ = ExpGeom(prob1)
	// Output:

	prob2 := 0.5
	_, _ = VarGeom(prob2)
	// Output: 2

	_, _ = Describe([]float64{1.0, 2.0, 3.0}, true, &[]float64{25.0, 50.0, 75.0})
}
