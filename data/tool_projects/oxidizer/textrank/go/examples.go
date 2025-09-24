package textrank

func Example() {
	rawText := "Your long raw text, it could be a book. Lorem ipsum..."
	// TextRank object
	tr := NewTextRank()
	// Default Rule for parsing.
	rule := NewDefaultRule()
	// Default Language for filtering stop words.
	language := NewDefaultLanguage()
	// Default algorithm for ranking text.
	algorithmDef := NewDefaultAlgorithm()

	// Add text.
	tr.Populate(rawText, language, rule)
	// Run the ranking.
	tr.Ranking(algorithmDef)

	// Get all phrases by weight.
	_ = tr.FindPhrases()

	// Get all words order by weight.
	_ = tr.FindSingleWords()

	// Get the most important 10 sentences. Importance by phrase weights.
	_ = tr.FindSentencesByRelationWeight(10)

	// Get the most important 10 sentences. Importance by word occurrence.
	_ = tr.FindSentencesByWordQtyWeight(10)

	// Get the first 10 sentences, start from 5th sentence.
	_ = tr.FindSentencesFrom(5, 10)

	// Get sentences by phrase/word chains order by position in text.
	_ = tr.FindSentencesByPhraseChain([]string{"gnome", "shell", "extension"})
}
