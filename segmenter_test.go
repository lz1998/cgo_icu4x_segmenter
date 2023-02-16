package cgo_icu4x_segmenter

import (
	"testing"
)

func testSegmentation(t *testing.T, breaker func(string) []*Segment, text string) {
	segments := breaker(text)
	t.Log("============")
	for _, segment := range segments {
		t.Logf("start: %d, end: %d, text: %v", segment.Start, segment.End, segment.Text)
	}
}

func TestBreakSentence(t *testing.T) {
	testSegmentation(t, BreakSentence, "如果不用担心低级别的实现细节，那就太好了。谁会关心空元组占用了多少空间呢？可悲的是，它有时很重要，我们需要担心这个问题。开发人员开始关心实现细节的最常见的原因是性能，但更重要的是，当与硬件、操作系统或其他语言直接对接时，这些细节会成为正确性的问题。")
	testSegmentation(t, BreakSentence, "Mr. Jones comes home. Dr. Smith Ph.D. is out. In the U.S.A. it is hot.")
	testSegmentation(t, BreakSentence, "    ")
}

func TestBreakWord(t *testing.T) {
	testSegmentation(t, BreakWord, "Welcome龟山岛龟山岛Welcome")
	testSegmentation(t, BreakWord, "うなぎうなじ")
	testSegmentation(t, BreakWord, "It would be great to not have to worry about low-level implementation details. Who could possibly care how much space the empty tuple occupies? Sadly, it sometimes matters and we need to worry about it. The most common reason developers start to care about implementation details is performance, but more importantly, these details can become a matter of correctness when interfacing directly with hardware, operating systems, or other languages.")
}
