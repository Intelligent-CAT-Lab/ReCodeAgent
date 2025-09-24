package gohistogram

func Example() {
	h := NewHistogram(160)
	h.Add(160)
	_ = h.Quantile(0.25)
	_ = h.CDF(18)
	_ = h.Count()
	_ = h.Mean()
	_ = h.Variance()

	w := NewWeightedHistogram(160, 1)
	w.Add(160)
	_ = w.Quantile(0.25)
	_ = w.CDF(18)
	_ = w.Count()
	_ = w.Mean()
	_ = w.Variance()
}

// func example(hist Histogram) {
// 	hist.Add(160)
// 	_ = hist.Quantile(0.25)
// }
