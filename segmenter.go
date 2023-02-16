package cgo_icu4x_segmenter

type Segment struct {
	Text  string
	Start int
	End   int
}

func convert(text string, ret *SegmentResult) []*Segment {
	output := make([]*Segment, 0)
	prev := int(ret.Next())
	cur := int(ret.Next())
	for cur > 0 {
		output = append(output, &Segment{
			Text:  string([]rune(text)[prev:cur]),
			Start: prev,
			End:   cur,
		})
		prev, cur = cur, int(ret.Next())
	}
	return output
}

func BreakSentence(text string) []*Segment {
	ret := cBreakSentence(text)
	defer ret.Close()
	return convert(text, ret)
}

func BreakWord(text string) []*Segment {
	ret := cBreakWOrd(text)
	defer ret.Close()
	return convert(text, ret)
}
