package textrank

// TextToRank function converts a ParsedSentence object to Rank object, it is
// the preparing process to later text ranking.
func TextToRank(sentence ParsedSentence, lang Language, ranks *Rank) {
	sentenceId := addSentence(ranks, sentence)
	addWord(ranks, sentence.GetWords(), lang, sentenceId)
}

func addWord(ranks *Rank, words []string, lang Language, sentenceID int) {
	prevWordID := -1
	var curWordID int

	for _, word := range words {
		if !lang.IsStopWord(word) {
			if found, rootWord := lang.FindRootWord(word); found {
				word = rootWord
			}

			if !ranks.IsWordExist(word) {
				curWordID = ranks.AddNewWord(word, prevWordID, sentenceID)
			} else {
				curWordID = ranks.UpdateWord(word, prevWordID, sentenceID)
			}

			ranks.Relation.AddRelation(curWordID, prevWordID, sentenceID)
			ranks.UpdateRightConnection(prevWordID, curWordID)

			prevWordID = curWordID
		}
	}
}

func addSentence(ranks *Rank, sentence ParsedSentence) int {
	ranks.SentenceMap[len(ranks.SentenceMap)] = sentence.GetOriginal()

	return len(ranks.SentenceMap) - 1
}
